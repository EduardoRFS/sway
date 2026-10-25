//! Native-endian i3/Sway IPC framing over ordinary Rust readers and writers.
#![forbid(unsafe_code)]

use std::{
    error::Error,
    fmt,
    io::{self, Read, Write},
};

pub const HEADER_SIZE: usize = 14;
const MAGIC: &[u8; 6] = b"i3-ipc";

/// Open IPC discriminator: unknown commands and events must survive decoding.
/// It is distinct from payload lengths, gesture kinds, and direction masks.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MessageType(u32);

impl MessageType {
    pub const COMMAND: Self = Self(0);
    pub const GET_WORKSPACES: Self = Self(1);
    pub const SUBSCRIBE: Self = Self(2);
    pub const GET_OUTPUTS: Self = Self(3);
    pub const GET_TREE: Self = Self(4);
    pub const GET_MARKS: Self = Self(5);
    pub const GET_BAR_CONFIG: Self = Self(6);
    pub const GET_VERSION: Self = Self(7);
    pub const GET_BINDING_MODES: Self = Self(8);
    pub const GET_CONFIG: Self = Self(9);
    pub const SEND_TICK: Self = Self(10);
    pub const SYNC: Self = Self(11);
    pub const GET_BINDING_STATE: Self = Self(12);
    pub const GET_INPUTS: Self = Self(100);
    pub const GET_SEATS: Self = Self(101);

    pub const EVENT_WORKSPACE: Self = Self(0x8000_0000);
    pub const EVENT_OUTPUT: Self = Self(0x8000_0001);
    pub const EVENT_MODE: Self = Self(0x8000_0002);
    pub const EVENT_WINDOW: Self = Self(0x8000_0003);
    pub const EVENT_BAR_CONFIG_UPDATE: Self = Self(0x8000_0004);
    pub const EVENT_BINDING: Self = Self(0x8000_0005);
    pub const EVENT_SHUTDOWN: Self = Self(0x8000_0006);
    pub const EVENT_TICK: Self = Self(0x8000_0007);
    pub const EVENT_BAR_STATE_UPDATE: Self = Self(0x8000_0014);
    pub const EVENT_INPUT: Self = Self(0x8000_0015);

    pub const fn is_event(self) -> bool {
        self.0 & 0x8000_0000 != 0
    }
}

impl From<u32> for MessageType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<MessageType> for u32 {
    fn from(kind: MessageType) -> Self {
        kind.0
    }
}

/// Decoded frames own their bytes by default. Encoding also accepts borrowed
/// slices (or other AsRef<[u8]> buffers) without allocating a payload copy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame<B = Vec<u8>> {
    pub kind: MessageType,
    pub payload: B,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage {
    Header,
    Payload,
}

#[derive(Debug)]
pub struct WriteError {
    pub stage: Stage,
    pub source: io::Error,
}

#[derive(Debug)]
pub struct ReadError {
    pub stage: Stage,
    pub source: io::Error,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reading IPC {:?}: {}", self.stage, self.source)
    }
}

impl Error for ReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "writing IPC {:?}: {}", self.stage, self.source)
    }
}

impl Error for WriteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl ReadError {
    pub fn kind(&self) -> io::ErrorKind {
        self.source.kind()
    }
}

impl<B: AsRef<[u8]>> Frame<B> {
    pub fn write_to(&self, writer: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        let payload = self.payload.as_ref();
        let length = u32::try_from(payload.len()).map_err(|_| WriteError {
            stage: Stage::Header,
            source: io::Error::new(io::ErrorKind::InvalidInput, "IPC payload exceeds u32"),
        })?;
        let mut header = [0; HEADER_SIZE];
        header[..6].copy_from_slice(MAGIC);
        header[6..10].copy_from_slice(&length.to_ne_bytes());
        header[10..14].copy_from_slice(&u32::from(self.kind).to_ne_bytes());
        writer.write_all(&header).map_err(|source| WriteError {
            stage: Stage::Header,
            source,
        })?;
        writer.write_all(payload).map_err(|source| WriteError {
            stage: Stage::Payload,
            source,
        })
    }
}

impl Frame {
    pub fn read_from(reader: &mut (impl Read + ?Sized)) -> Result<Self, ReadError> {
        let mut header = [0; HEADER_SIZE];
        reader.read_exact(&mut header).map_err(|source| ReadError {
            stage: Stage::Header,
            source,
        })?;
        // Like the original client, accept replies without validating the magic.
        let size = u32::from_ne_bytes(header[6..10].try_into().unwrap());
        let kind = MessageType::from(u32::from_ne_bytes(header[10..14].try_into().unwrap()));
        let mut payload = Vec::new();
        // Grow with received data instead of zero-initializing the advertised size.
        reader
            .take(u64::from(size))
            .read_to_end(&mut payload)
            .map_err(|source| ReadError {
                stage: Stage::Payload,
                source,
            })?;
        if payload.len() != size as usize {
            return Err(ReadError {
                stage: Stage::Payload,
                source: io::Error::new(io::ErrorKind::UnexpectedEof, "truncated IPC payload"),
            });
        }
        Ok(Self { kind, payload })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct Fragmented<T>(T);

    impl<T: Read> Read for Fragmented<T> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let size = buffer.len().min(1);
            self.0.read(&mut buffer[..size])
        }
    }

    impl<T: Write> Write for Fragmented<T> {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            self.0.write(&buffer[..buffer.len().min(1)])
        }
        fn flush(&mut self) -> io::Result<()> {
            self.0.flush()
        }
    }

    #[test]
    fn fragmented_io_preserves_bytes_and_frame_boundaries() {
        let mut writer = Fragmented(Vec::new());
        Frame {
            kind: MessageType::GET_INPUTS,
            payload: b"a\0\xffb",
        }
        .write_to(&mut writer)
        .unwrap();
        Frame {
            kind: MessageType::EVENT_TICK,
            payload: b"",
        }
        .write_to(&mut writer)
        .unwrap();
        let mut reader = Fragmented(Cursor::new(writer.0));
        assert_eq!(
            Frame::read_from(&mut reader).unwrap(),
            Frame {
                kind: MessageType::GET_INPUTS,
                payload: b"a\0\xffb".to_vec()
            }
        );
        assert_eq!(
            Frame::read_from(&mut reader).unwrap(),
            Frame {
                kind: MessageType::EVENT_TICK,
                payload: vec![]
            }
        );
    }

    #[test]
    fn truncation_is_an_error_instead_of_an_infinite_receive_loop() {
        let mut wire = Vec::new();
        Frame {
            kind: MessageType::GET_TREE,
            payload: b"payload",
        }
        .write_to(&mut wire)
        .unwrap();
        for length in 0..wire.len() {
            let error = Frame::read_from(&mut Cursor::new(&wire[..length])).unwrap_err();
            assert_eq!(
                error.kind(),
                io::ErrorKind::UnexpectedEof,
                "prefix {length}"
            );
        }
    }

    #[test]
    fn preserves_legacy_acceptance_of_reply_magic() {
        let mut wire = Vec::new();
        Frame {
            kind: MessageType::GET_WORKSPACES,
            payload: b"[]",
        }
        .write_to(&mut wire)
        .unwrap();
        wire[..6].copy_from_slice(b"legacy");
        assert_eq!(
            Frame::read_from(&mut Cursor::new(wire)).unwrap().payload,
            b"[]"
        );
    }

    #[test]
    fn reports_which_write_failed() {
        struct FailingWriter(usize);
        impl Write for FailingWriter {
            fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
                if self.0 == 0 {
                    return Err(io::ErrorKind::BrokenPipe.into());
                }
                let count = self.0.min(buffer.len());
                self.0 -= count;
                Ok(count)
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        assert_eq!(
            Frame {
                kind: MessageType::COMMAND,
                payload: b"data"
            }
            .write_to(&mut FailingWriter(0))
            .unwrap_err()
            .stage,
            Stage::Header
        );
        assert_eq!(
            Frame {
                kind: MessageType::COMMAND,
                payload: b"data"
            }
            .write_to(&mut FailingWriter(HEADER_SIZE))
            .unwrap_err()
            .stage,
            Stage::Payload
        );
    }

    #[test]
    fn owned_and_borrowed_frames_support_io_trait_objects_and_unknown_types() {
        let payload = b"binary\0\xff";
        for raw in [0, 7, 100, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff] {
            let kind = MessageType::from(raw);
            assert_eq!(u32::from(kind), raw);
            assert_eq!(kind.is_event(), raw & 0x8000_0000 != 0);
            let borrowed = Frame {
                kind,
                payload: payload.as_slice(),
            };
            let owned = Frame {
                kind,
                payload: payload.to_vec(),
            };
            let mut wire = Vec::new();
            borrowed.write_to(&mut wire as &mut dyn Write).unwrap();
            let mut other = Vec::new();
            owned.write_to(&mut other as &mut dyn Write).unwrap();
            assert_eq!(wire, other);
            let decoded = Frame::read_from(&mut Cursor::new(wire) as &mut dyn Read).unwrap();
            assert_eq!(decoded, owned);
        }
    }
}
