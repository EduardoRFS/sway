//! Native-endian i3/Sway IPC framing over ordinary Rust readers and writers.
#![forbid(unsafe_code)]

use std::{
    error::Error,
    fmt,
    io::{self, Read, Write},
};

pub const HEADER_SIZE: usize = 14;
const MAGIC: &[u8; 6] = b"i3-ipc";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    pub kind: u32,
    pub payload: Vec<u8>,
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

pub fn write_frame(writer: &mut impl Write, kind: u32, payload: &[u8]) -> Result<(), WriteError> {
    let length = u32::try_from(payload.len()).map_err(|_| WriteError {
        stage: Stage::Header,
        source: io::Error::new(io::ErrorKind::InvalidInput, "IPC payload exceeds u32"),
    })?;
    let mut header = [0; HEADER_SIZE];
    header[..6].copy_from_slice(MAGIC);
    header[6..10].copy_from_slice(&length.to_ne_bytes());
    header[10..14].copy_from_slice(&kind.to_ne_bytes());
    writer.write_all(&header).map_err(|source| WriteError {
        stage: Stage::Header,
        source,
    })?;
    writer.write_all(payload).map_err(|source| WriteError {
        stage: Stage::Payload,
        source,
    })
}

pub fn read_frame(reader: &mut impl Read) -> Result<Frame, ReadError> {
    let mut header = [0; HEADER_SIZE];
    reader.read_exact(&mut header).map_err(|source| ReadError {
        stage: Stage::Header,
        source,
    })?;
    // Like the original client, accept replies without validating the magic.
    let size = u32::from_ne_bytes(header[6..10].try_into().unwrap());
    let kind = u32::from_ne_bytes(header[10..14].try_into().unwrap());
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
    Ok(Frame { kind, payload })
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
        write_frame(&mut writer, 100, b"a\0\xffb").unwrap();
        write_frame(&mut writer, 0x8000_0007, b"").unwrap();
        let mut reader = Fragmented(Cursor::new(writer.0));
        assert_eq!(
            read_frame(&mut reader).unwrap(),
            Frame {
                kind: 100,
                payload: b"a\0\xffb".to_vec()
            }
        );
        assert_eq!(
            read_frame(&mut reader).unwrap(),
            Frame {
                kind: 0x8000_0007,
                payload: vec![]
            }
        );
    }

    #[test]
    fn truncation_is_an_error_instead_of_an_infinite_receive_loop() {
        let mut wire = Vec::new();
        write_frame(&mut wire, 4, b"payload").unwrap();
        for length in 0..wire.len() {
            let error = read_frame(&mut Cursor::new(&wire[..length])).unwrap_err();
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
        write_frame(&mut wire, 1, b"[]").unwrap();
        wire[..6].copy_from_slice(b"legacy");
        assert_eq!(read_frame(&mut Cursor::new(wire)).unwrap().payload, b"[]");
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
            write_frame(&mut FailingWriter(0), 0, b"data")
                .unwrap_err()
                .stage,
            Stage::Header
        );
        assert_eq!(
            write_frame(&mut FailingWriter(HEADER_SIZE), 0, b"data")
                .unwrap_err()
                .stage,
            Stage::Payload
        );
    }
}
