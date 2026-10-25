//! Gesture parsing, matching and tracking, independent of C allocation and FFI.
#![forbid(unsafe_code)]

use std::{error::Error, fmt, num::NonZeroU8, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Kind {
    #[default]
    None,
    Hold,
    Pinch,
    Swipe,
    Unknown(UnknownKind),
}

/// An unrecognized kind from the wire. The private field prevents aliases such
/// as Unknown(1), which would encode Hold but compare unequal to it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnknownKind(u32);

impl Kind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Hold => "hold",
            Self::Pinch => "pinch",
            Self::Swipe => "swipe",
            Self::Unknown(_) => "(null)",
        }
    }
}

impl From<u32> for Kind {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Hold,
            2 => Self::Pinch,
            3 => Self::Swipe,
            other => Self::Unknown(UnknownKind(other)),
        }
    }
}

impl From<Kind> for u32 {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::None => 0,
            Kind::Hold => 1,
            Kind::Pinch => 2,
            Kind::Swipe => 3,
            Kind::Unknown(UnknownKind(value)) => value,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A binding's finger constraint. Wire values above nine remain representable;
/// the textual configuration grammar alone limits counts to one through nine.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FingerCount {
    #[default]
    Any,
    Exact(NonZeroU8),
}

impl FingerCount {
    pub fn accepts(self, input: Self) -> bool {
        self == Self::Any || self == input
    }
}

impl From<u8> for FingerCount {
    fn from(value: u8) -> Self {
        NonZeroU8::new(value).map_or(Self::Any, Self::Exact)
    }
}

impl From<FingerCount> for u8 {
    fn from(count: FingerCount) -> Self {
        match count {
            FingerCount::Any => 0,
            FingerCount::Exact(count) => count.get(),
        }
    }
}

bitflags::bitflags! {
    /// Gesture direction constraints. Empty means any direction. All wire bits
    /// are retained, including bits without a known direction name.
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Directions: u32 {
        const UP = 1 << 0;
        const DOWN = 1 << 1;
        const LEFT = 1 << 2;
        const RIGHT = 1 << 3;
        const INWARD = 1 << 4;
        const OUTWARD = 1 << 5;
        const CLOCKWISE = 1 << 6;
        const COUNTERCLOCKWISE = 1 << 7;
        const _ = !0;
    }
}

impl Directions {
    const NAMES: [(Self, &'static str); 8] = [
        (Self::UP, "up"),
        (Self::DOWN, "down"),
        (Self::LEFT, "left"),
        (Self::RIGHT, "right"),
        (Self::INWARD, "inward"),
        (Self::OUTWARD, "outward"),
        (Self::CLOCKWISE, "clockwise"),
        (Self::COUNTERCLOCKWISE, "counterclockwise"),
    ];

    fn named(bytes: &[u8]) -> Option<Self> {
        Self::NAMES
            .iter()
            .find_map(|&(direction, name)| (name.as_bytes() == bytes).then_some(direction))
    }
}

impl fmt::Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return f.write_str("any");
        }
        let mut separator = "";
        // Unlike bitflags' default formatter, the legacy grammar prints one
        // "unknown" for each unnamed bit, in ascending bit order.
        for bit in 0..u32::BITS {
            if self.bits() & (1 << bit) != 0 {
                let name = Self::NAMES
                    .get(bit as usize)
                    .map_or("unknown", |&(_, name)| name);
                write!(f, "{separator}{name}")?;
                separator = "+";
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchMode {
    Subset,
    Exact,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Gesture {
    pub kind: Kind,
    pub fingers: FingerCount,
    pub directions: Directions,
}

/// Machine-readable failures retain the original input bytes, including
/// non-UTF-8 bytes needed by the C API's diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    InvalidSyntax(Vec<u8>),
    UnknownKind(Vec<u8>),
    InvalidFingerCount(Vec<u8>),
    UnknownDirection(Vec<u8>),
}

impl ParseErrorKind {
    fn parts(&self) -> (&'static str, &[u8]) {
        match self {
            Self::InvalidSyntax(value) => {
                ("expected <gesture>[:<fingers>][:direction], got ", value)
            }
            Self::UnknownKind(value) => ("expected hold|pinch|swipe, got ", value),
            Self::InvalidFingerCount(value) => ("expected 1-9, got ", value),
            Self::UnknownDirection(value) => ("expected direction, got ", value),
        }
    }

    pub fn message_bytes(&self) -> Vec<u8> {
        let (prefix, value) = self.parts();
        [prefix.as_bytes(), value].concat()
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (prefix, value) = self.parts();
        write!(f, "{prefix}{}", String::from_utf8_lossy(value))
    }
}

impl Error for ParseErrorKind {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
    /// The original parser exposes its partially parsed result on failure.
    pub partial: Gesture,
    pub kind: ParseErrorKind,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

impl Gesture {
    pub fn parse(input: &[u8]) -> Result<Self, ParseError> {
        let mut result = Self::default();
        // strtok skips empty fields, including leading and trailing separators.
        let parts: Vec<_> = input
            .split(|byte| *byte == b':')
            .filter(|part| !part.is_empty())
            .collect();
        if !(1..=3).contains(&parts.len()) {
            return Err(ParseError {
                partial: result,
                kind: ParseErrorKind::InvalidSyntax(input.into()),
            });
        }
        result.kind = match parts[0] {
            b"hold" => Kind::Hold,
            b"pinch" => Kind::Pinch,
            b"swipe" => Kind::Swipe,
            other => {
                return Err(ParseError {
                    partial: result,
                    kind: ParseErrorKind::UnknownKind(other.into()),
                })
            }
        };
        if let Some(&next) = parts.get(1) {
            let direction = if let [finger @ b'1'..=b'9'] = next {
                result.fingers = FingerCount::from(finger - b'0');
                parts.get(2).copied()
            } else if parts.len() == 3 {
                return Err(ParseError {
                    partial: result,
                    kind: ParseErrorKind::InvalidFingerCount(next.into()),
                });
            } else {
                Some(next)
            };
            if let Some(direction) = direction {
                for item in direction
                    .split(|byte| *byte == b'+')
                    .filter(|part| !part.is_empty())
                {
                    if item == b"any" {
                        continue;
                    }
                    match Directions::named(item) {
                        Some(direction) => result.directions.insert(direction),
                        None => {
                            return Err(ParseError {
                                partial: result,
                                kind: ParseErrorKind::UnknownDirection(item.into()),
                            })
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    pub fn accepts(&self, kind: Kind, fingers: FingerCount) -> bool {
        self.kind == kind && self.fingers.accepts(fingers)
    }

    pub fn matches(&self, input: &Self, mode: MatchMode) -> bool {
        self.accepts(input.kind, input.fingers)
            && match mode {
                MatchMode::Exact => self.directions == input.directions,
                MatchMode::Subset => input.directions.contains(self.directions),
            }
    }

    /// Sorting key for binding priority, independent of gesture identity.
    /// Gestures with equal specificity can be unequal; this is not an Ord impl.
    pub fn specificity(&self) -> u32 {
        self.directions.bits().count_ones()
    }
}

impl FromStr for Gesture {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input.as_bytes())
    }
}

impl TryFrom<&[u8]> for Gesture {
    type Error = ParseError;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl fmt::Display for Gesture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The legacy renderer emits zero for the wildcard finger count, even
        // though the parser's numeric syntax only accepts one through nine.
        write!(
            f,
            "{}:{}:{}",
            self.kind,
            u8::from(self.fingers),
            self.directions
        )
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Tracker {
    pub kind: Kind,
    pub fingers: FingerCount,
    pub dx: f64,
    pub dy: f64,
    pub scale: f64,
    pub rotation: f64,
}

impl Tracker {
    pub fn begin(&mut self, kind: Kind, fingers: FingerCount) {
        *self = Self {
            kind,
            fingers,
            scale: 1.0,
            ..Self::default()
        };
    }

    /// Holds do not have motion updates.
    pub fn update(&mut self, dx: f64, dy: f64, scale: f64, rotation: f64) -> bool {
        if self.kind == Kind::Hold {
            return false;
        }
        self.dx += dx;
        self.dy += dy;
        if self.kind == Kind::Pinch {
            self.scale = scale;
            self.rotation += rotation;
        }
        true
    }

    pub fn end(&mut self) -> Option<Gesture> {
        if self.kind == Kind::None {
            return None;
        }
        let mut directions = Directions::empty();
        if self.kind == Kind::Pinch {
            if self.rotation > 5.0 {
                directions.insert(Directions::CLOCKWISE);
            }
            if self.rotation < -5.0 {
                directions.insert(Directions::COUNTERCLOCKWISE);
            }
            if self.scale > 1.1 {
                directions.insert(Directions::OUTWARD);
            }
            if self.scale < 0.9 {
                directions.insert(Directions::INWARD);
            }
        }
        if matches!(self.kind, Kind::Pinch | Kind::Swipe) {
            // Keep the original tie (including zero motion) and NaN behavior.
            directions.insert(if self.dx.abs() > self.dy.abs() {
                if self.dx > 0.0 {
                    Directions::RIGHT
                } else {
                    Directions::LEFT
                }
            } else if self.dy > 0.0 {
                Directions::DOWN
            } else {
                Directions::UP
            });
        }
        let gesture = Gesture {
            kind: self.kind,
            fingers: self.fingers,
            directions,
        };
        self.kind = Kind::None;
        Some(gesture)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn parsing_keeps_strtok_and_partial_failure_behavior() {
        assert_eq!(
            Gesture::try_from(&b"::swipe::3::up++left:"[..])
                .unwrap()
                .directions,
            Directions::UP | Directions::LEFT
        );
        let error = Gesture::parse(b"pinch:4:inward+invalid").unwrap_err();
        assert_eq!(
            error.partial,
            Gesture {
                kind: Kind::Pinch,
                fingers: FingerCount::from(4),
                directions: Directions::INWARD,
            }
        );
        assert_eq!(
            error.kind,
            ParseErrorKind::UnknownDirection(b"invalid".to_vec())
        );
        assert_eq!(
            error.source().unwrap().to_string(),
            "expected direction, got invalid"
        );
        assert_eq!(
            Gesture::parse(b"hold:\xff")
                .unwrap_err()
                .kind
                .message_bytes(),
            b"expected direction, got \xff"
        );
        for (input, expected) in [
            (&b""[..], ParseErrorKind::InvalidSyntax(vec![])),
            (
                &b"other"[..],
                ParseErrorKind::UnknownKind(b"other".to_vec()),
            ),
            (
                &b"swipe:10:up"[..],
                ParseErrorKind::InvalidFingerCount(b"10".to_vec()),
            ),
        ] {
            assert_eq!(Gesture::parse(input).unwrap_err().kind, expected);
        }
    }

    #[test]
    fn wildcard_matching_and_specificity() {
        let target: Gesture = "swipe:up".parse().unwrap();
        let input: Gesture = "swipe:3:up+left".parse().unwrap();
        assert!(target.matches(&input, MatchMode::Subset));
        assert!(!target.matches(&input, MatchMode::Exact));
        assert_eq!(input.specificity(), 2);
        assert_eq!(input.to_string(), "swipe:3:up+left");
        assert_eq!(target.to_string(), "swipe:0:up");
        let other: Gesture = "swipe:down".parse().unwrap();
        assert_eq!(target.specificity(), other.specificity());
        assert_eq!(HashSet::from([target, target, other]).len(), 2);
    }

    #[test]
    fn nominal_types_preserve_all_wire_values() {
        for value in 0..=u8::MAX {
            assert_eq!(u8::from(FingerCount::from(value)), value);
        }
        for value in [0, 1, 2, 3, 4, u32::MAX] {
            assert_eq!(u32::from(Kind::from(value)), value);
        }
        let unknown = Directions::from_bits_retain(0x8000_0101);
        assert_eq!(unknown.bits(), 0x8000_0101);
        assert_eq!(unknown.to_string(), "up+unknown+unknown");
        assert!(unknown.contains(Directions::UP));
        assert_eq!((!unknown).bits(), !0x8000_0101);
        let target = Gesture {
            kind: Kind::Swipe,
            fingers: FingerCount::Any,
            directions: unknown,
        };
        let input = Gesture {
            fingers: FingerCount::from(255),
            ..target
        };
        assert!(target.matches(&input, MatchMode::Exact));
        assert!(!input.matches(&target, MatchMode::Subset));
    }

    #[test]
    fn tracking_keeps_thresholds_ties_and_reset() {
        let mut tracker = Tracker::default();
        tracker.begin(Kind::Pinch, FingerCount::from(3));
        assert!(tracker.update(0.0, 0.0, 1.1, 5.0));
        assert_eq!(tracker.end().unwrap().directions, Directions::UP);
        assert!(tracker.end().is_none());
        tracker.begin(Kind::Pinch, FingerCount::from(4));
        tracker.update(10.0, 1.0, 0.8, -6.0);
        assert_eq!(
            tracker.end().unwrap().directions,
            Directions::RIGHT | Directions::INWARD | Directions::COUNTERCLOCKWISE
        );
        tracker.begin(Kind::Hold, FingerCount::from(2));
        assert!(!tracker.update(1.0, 1.0, 2.0, 10.0));
        assert_eq!(tracker.end().unwrap().directions, Directions::empty());
    }
}
