use core::error::Error;
use core::fmt;
use core::iter::FusedIterator;

use crate::{Color, ParseColorError, parse};

/// Error wrapper
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseColorsError<'a> {
    /// The invalid string slice
    pub color: &'a str,
    /// The original error
    pub source: ParseColorError,
}

impl fmt::Display for ParseColorsError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid color {:?}", self.color)
    }
}

impl Error for ParseColorsError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

/// Iterator
#[derive(Debug, Clone)]
pub struct ParseColors<'a> {
    remaining: &'a str,
}

impl<'a> ParseColors<'a> {
    /// Split at the first top-level comma: `(segment, rest_after_comma)`.
    fn split_first(&self) -> (&'a str, &'a str) {
        let s = self.remaining;
        let mut depth: usize = 0;
        // `(`, `)` and `,` are ASCII, so byte indices are valid char boundaries.
        for (i, &b) in s.as_bytes().iter().enumerate() {
            match b {
                b'(' => depth += 1,
                b')' => depth = depth.saturating_sub(1),
                b',' if depth == 0 => return (&s[..i], &s[i + 1..]),
                _ => {}
            }
        }
        (s, "")
    }
}

impl<'a> Iterator for ParseColors<'a> {
    type Item = Result<Color, ParseColorsError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.remaining.is_empty() {
            let (seg, rest) = self.split_first();
            self.remaining = rest;

            let color = seg.trim();
            if !color.is_empty() {
                return Some(parse(color).map_err(|source| ParseColorsError { color, source }));
            }
        }
        None
    }
}

impl FusedIterator for ParseColors<'_> {}

/// Parse string of multiple colors separated by comma.
///
/// ```rust
/// let input = "#2e8b57, rgb(255, 138, 75), gold, xyz(0.756, 0.18, 0.93), hwb(300 63% 13%)";
///
/// for res in csscolorparser::parse_colors(input) {
///     match res {
///         Ok(col) => println!("{col}"),
///         Err(err) => println!("{err}"),
///     }
/// }
/// ```
#[inline]
pub const fn parse_colors(s: &str) -> ParseColors<'_> {
    ParseColors { remaining: s }
}
