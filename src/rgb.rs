//! This module contains [RGB] structure.

use core::fmt::{self, Display};
use core::{num::ParseIntError, str};

/// Red Green Blue
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RGB<T = u8> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
}

impl<T> RGB<T> {
    /// Creates a new [RGB]
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl From<(u8, u8, u8)> for RGB {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl str::FromStr for RGB {
    type Err = ParseRGBError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseRGBError::new(RGBErrorKind::Invalid));
        }

        if let Some(stripped) = s.strip_prefix('#') {
            s = stripped;
        }

        if s.len() != 6 {
            return Err(ParseRGBError::new(RGBErrorKind::Size));
        }

        let r = u8::from_str_radix(&s[..2], 16)
            .map_err(|e| ParseRGBError::new(RGBErrorKind::Format { error: e, pos: 0 }))?;

        let g = u8::from_str_radix(&s[2..4], 16)
            .map_err(|e| ParseRGBError::new(RGBErrorKind::Format { error: e, pos: 1 }))?;

        let b = u8::from_str_radix(&s[4..6], 16)
            .map_err(|e| ParseRGBError::new(RGBErrorKind::Format { error: e, pos: 2 }))?;

        Ok(Self { r, g, b })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRGBError {
    kind: RGBErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RGBErrorKind {
    Size,
    Invalid,
    Format { pos: usize, error: ParseIntError },
}

impl ParseRGBError {
    fn new(kind: RGBErrorKind) -> Self {
        Self { kind }
    }

    /// Outputs the detailed cause of parsing an integer failing.
    #[must_use]
    pub fn kind(&self) -> &RGBErrorKind {
        &self.kind
    }
}

impl Display for ParseRGBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            RGBErrorKind::Size => {
                "cannot parse rgb from string with size not equal to 6 (or 7 if '#' is used)".fmt(f)
            }
            RGBErrorKind::Invalid => "invalid string which contains UTF8 symbols".fmt(f),
            RGBErrorKind::Format { pos, error } => {
                error.fmt(f)?;
                " on position ".fmt(f)?;
                pos.fmt(f)
            }
        }
    }
}

// rgb!(#234312)
// rgb!(0xFF, 0xAA, 0xCC)
