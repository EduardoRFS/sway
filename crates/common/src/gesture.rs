//! Gesture parsing, matching and tracking, independent of C allocation and FFI.
#![forbid(unsafe_code)]

use std::{error::Error, fmt, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Kind {
    #[default]
    None,
    Hold,
    Pinch,
    Swipe,
    Unknown(u32),
}

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

    pub const fn as_raw(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Hold => 1,
            Self::Pinch => 2,
            Self::Swipe => 3,
            Self::Unknown(value) => value,
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
            other => Self::Unknown(other),
        }
    }
}

pub const DIRECTIONS: [&str; 8] = [
    "up",
    "down",
    "left",
    "right",
    "inward",
    "outward",
    "clockwise",
    "counterclockwise",
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Gesture {
    pub kind: Kind,
    /// Zero accepts any finger count.
    pub fingers: u8,
    pub directions: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
    /// The original parser exposes its partially parsed result on failure.
    pub partial: Gesture,
    /// Retain arbitrary input bytes for the C API's error messages.
    pub message: Vec<u8>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from_utf8_lossy(&self.message).fmt(f)
    }
}

impl Error for ParseError {}

impl Gesture {
    pub fn parse(input: &[u8]) -> Result<Self, ParseError> {
        let mut result = Self::default();
        // strtok skips empty fields, including leading and trailing separators.
        let parts: Vec<_> = input
            .split(|byte| *byte == b':')
            .filter(|part| !part.is_empty())
            .collect();
        let error = |partial, prefix: &[u8], value: &[u8]| ParseError {
            partial,
            message: [prefix, value].concat(),
        };
        if !(1..=3).contains(&parts.len()) {
            return Err(error(
                result,
                b"expected <gesture>[:<fingers>][:direction], got ",
                input,
            ));
        }
        result.kind = match parts[0] {
            b"hold" => Kind::Hold,
            b"pinch" => Kind::Pinch,
            b"swipe" => Kind::Swipe,
            other => return Err(error(result, b"expected hold|pinch|swipe, got ", other)),
        };
        if let Some(&next) = parts.get(1) {
            let direction = if let [finger @ b'1'..=b'9'] = next {
                result.fingers = finger - b'0';
                parts.get(2).copied()
            } else if parts.len() == 3 {
                return Err(error(result, b"expected 1-9, got ", next));
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
                    match DIRECTIONS.iter().position(|name| name.as_bytes() == item) {
                        Some(bit) => result.directions |= 1 << bit,
                        None => return Err(error(result, b"expected direction, got ", item)),
                    }
                }
            }
        }
        Ok(result)
    }

    pub fn accepts(&self, kind: Kind, fingers: u8) -> bool {
        self.kind == kind && (self.fingers == 0 || self.fingers == fingers)
    }

    pub fn matches(&self, input: &Self, exact: bool) -> bool {
        self.accepts(input.kind, input.fingers)
            && (!exact || self.directions == input.directions)
            && self.directions & input.directions == self.directions
    }

    pub fn specificity(&self) -> u32 {
        self.directions.count_ones()
    }
}

impl FromStr for Gesture {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input.as_bytes())
    }
}

impl fmt::Display for Gesture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:", self.kind.name(), self.fingers)?;
        if self.directions == 0 {
            return f.write_str("any");
        }
        let mut separator = "";
        for bit in 0..32 {
            if self.directions & (1 << bit) != 0 {
                write!(
                    f,
                    "{separator}{}",
                    DIRECTIONS.get(bit).unwrap_or(&"unknown")
                )?;
                separator = "+";
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Tracker {
    pub kind: Kind,
    pub fingers: u8,
    pub dx: f64,
    pub dy: f64,
    pub scale: f64,
    pub rotation: f64,
}

impl Tracker {
    pub fn begin(&mut self, kind: Kind, fingers: u8) {
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
        let mut directions = 0;
        if self.kind == Kind::Pinch {
            if self.rotation > 5.0 {
                directions |= 1 << 6;
            }
            if self.rotation < -5.0 {
                directions |= 1 << 7;
            }
            if self.scale > 1.1 {
                directions |= 1 << 5;
            }
            if self.scale < 0.9 {
                directions |= 1 << 4;
            }
        }
        if matches!(self.kind, Kind::Pinch | Kind::Swipe) {
            // Keep the original tie (including zero motion) and NaN behavior.
            directions |= if self.dx.abs() > self.dy.abs() {
                if self.dx > 0.0 {
                    1 << 3
                } else {
                    1 << 2
                }
            } else if self.dy > 0.0 {
                1 << 1
            } else {
                1
            };
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

    #[test]
    fn parsing_keeps_strtok_and_partial_failure_behavior() {
        assert_eq!(
            Gesture::parse(b"::swipe::3::up++left:").unwrap().directions,
            5
        );
        let error = Gesture::parse(b"pinch:4:inward+invalid").unwrap_err();
        assert_eq!(
            error.partial,
            Gesture {
                kind: Kind::Pinch,
                fingers: 4,
                directions: 16
            }
        );
        assert_eq!(error.message, b"expected direction, got invalid");
        assert_eq!(
            Gesture::parse(b"hold:\xff").unwrap_err().message,
            b"expected direction, got \xff"
        );
    }

    #[test]
    fn wildcard_matching_and_specificity() {
        let target: Gesture = "swipe:up".parse().unwrap();
        let input: Gesture = "swipe:3:up+left".parse().unwrap();
        assert!(target.matches(&input, false));
        assert!(!target.matches(&input, true));
        assert_eq!(input.specificity(), 2);
        assert_eq!(input.to_string(), "swipe:3:up+left");
    }

    #[test]
    fn tracking_keeps_thresholds_ties_and_reset() {
        let mut tracker = Tracker::default();
        tracker.begin(Kind::Pinch, 3);
        assert!(tracker.update(0.0, 0.0, 1.1, 5.0));
        assert_eq!(tracker.end().unwrap().directions, 1);
        assert!(tracker.end().is_none());
        tracker.begin(Kind::Pinch, 4);
        tracker.update(10.0, 1.0, 0.8, -6.0);
        assert_eq!(tracker.end().unwrap().directions, 8 | 16 | 128);
        tracker.begin(Kind::Hold, 2);
        assert!(!tracker.update(1.0, 1.0, 2.0, 10.0));
        assert_eq!(tracker.end().unwrap().directions, 0);
    }
}
