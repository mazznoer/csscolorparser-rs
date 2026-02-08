use core::{error, fmt};

use crate::{Color, ParseColorError, parse};

/// Error wrapper
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseColorsError<'a> {
    /// The error
    pub err: ParseColorError,
    /// The invalid string slice
    pub s: &'a str,
}

impl fmt::Display for ParseColorsError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.err, self.s)
    }
}

impl error::Error for ParseColorsError<'_> {}

/// Iterator
#[derive(Debug, Clone)]
pub struct ParseColors<'a> {
    s: &'a str,
    pos: usize,
    inside: bool,
}

impl<'a> Iterator for ParseColors<'a> {
    type Item = Result<Color, ParseColorsError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.s.len() {
            return None;
        }
        let start = self.pos;
        for (i, c) in self.s[self.pos..].char_indices() {
            if c == ',' && !self.inside {
                self.pos = self.pos + i + 1;
                let s = &self.s[start..self.pos - 1];
                if s.trim().is_empty() {
                    return self.next();
                }
                return Some(parse(s).map_err(|err| ParseColorsError { err, s }));
            } else if c == '(' {
                self.inside = true;
            } else if c == ')' {
                self.inside = false;
            }
        }
        self.pos = self.s.len() + 1;
        let s = &self.s[start..];
        if s.trim().is_empty() {
            return None;
        }
        Some(parse(s).map_err(|err| ParseColorsError { err, s }))
    }
}

/// Parse multiple colors separated by comma.
pub const fn parse_colors(s: &str) -> ParseColors<'_> {
    ParseColors {
        s,
        pos: 0,
        inside: false,
    }
}
