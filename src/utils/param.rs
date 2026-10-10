// Parser for CSS color function parameters

const KIND_NORMAL: u8 = 0;
const KIND_DELIM: u8 = 1;
const KIND_OPEN: u8 = 2;
const KIND_CLOSE: u8 = 3;

// Compile-time 256-entry lookup table for 1-cycle character classification
const CHAR_KIND: [u8; 256] = {
    let mut table = [KIND_NORMAL; 256];
    let mut i = 0;
    while i < 256 {
        let b = i as u8;
        if b.is_ascii_whitespace() || b == b',' || b == b'/' {
            table[i] = KIND_DELIM;
        } else if b == b'(' {
            table[i] = KIND_OPEN;
        } else if b == b')' {
            table[i] = KIND_CLOSE;
        }
        i += 1;
    }
    table
};

pub(crate) struct ParamParser<'a> {
    bytes: &'a [u8],
}

impl<'a> ParamParser<'a> {
    // Input is ASCII only
    #[inline]
    pub fn new(s: &'a str) -> Self {
        Self {
            bytes: s.as_bytes(),
        }
    }

    // Returns `&str` from current index until space, comma, or slash is found.
    // Ignore space, comma, or slash inside parentheses.
    // Returns `None` if value not found.
    pub fn value(&mut self) -> Option<&'a str> {
        let &first = self.bytes.first()?;
        if CHAR_KIND[first as usize] == KIND_DELIM {
            return None;
        }

        let mut end = 0;
        let mut nesting = 0i32;

        for &b in self.bytes {
            match CHAR_KIND[b as usize] {
                KIND_OPEN => nesting += 1,
                KIND_CLOSE => {
                    if nesting > 0 {
                        nesting -= 1;
                    }
                }
                KIND_DELIM if nesting == 0 => {
                    break;
                }
                _ => {}
            }
            end += 1;
        }

        let (val, rest) = self.bytes.split_at(end);
        self.bytes = rest;

        // SAFETY: Input originated from valid UTF-8, and ASCII boundaries guarantee valid UTF-8 subslices.
        Some(unsafe { core::str::from_utf8_unchecked(val) })
    }

    // Consume one or more ASCII whitespace characters.
    // Returns `true` if at least one was found, `false` otherwise.
    pub fn space(&mut self) -> bool {
        let pos = self
            .bytes
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .unwrap_or(self.bytes.len());

        if pos > 0 {
            self.bytes = &self.bytes[pos..];
            true
        } else {
            false
        }
    }

    // Consume one or more spaces, or single comma.
    // Spaces is allowed around comma.
    // Returns true if one of them is found, false otherwise.
    pub fn comma_or_space(&mut self) -> bool {
        let mut found_comma = false;
        let mut found_space = false;
        let mut count = 0;

        for &b in self.bytes {
            if b.is_ascii_whitespace() {
                found_space = true;
                count += 1;
            } else if b == b',' {
                if found_comma {
                    break;
                }
                found_comma = true;
                count += 1;
            } else {
                break;
            }
        }

        self.bytes = &self.bytes[count..];
        found_comma || found_space
    }

    // Consume single comma or single slash.
    // Spaces is allowed around comma or slash.
    // Returns true if one of them is found, false otherwise.
    pub fn comma_or_slash(&mut self) -> bool {
        self.consume_delim_with_spaces(|b| b == b',' || b == b'/')
    }

    // Consume a single slash. Spaces is allowed around slash.
    // Returns true if a slash is found, false otherwise.
    pub fn slash(&mut self) -> bool {
        self.consume_delim_with_spaces(|b| b == b'/')
    }

    #[inline]
    fn consume_delim_with_spaces(&mut self, is_target_delim: impl Fn(u8) -> bool) -> bool {
        let mut found = false;
        let mut count = 0;

        for &b in self.bytes {
            if b.is_ascii_whitespace() {
                count += 1;
            } else if is_target_delim(b) {
                if found {
                    break;
                }
                found = true;
                count += 1;
            } else {
                break;
            }
        }

        self.bytes = &self.bytes[count..];
        found
    }

    // Returns true if we finished reading the str.
    #[inline]
    pub fn is_end(&self) -> bool {
        self.bytes.is_empty()
    }
}

#[cfg(test)]
mod t {
    use super::ParamParser;

    #[test]
    fn param_parser() {
        let s = "   ";
        let mut p = ParamParser::new(s);
        assert_eq!(p.is_end(), false);
        assert!(p.space());
        assert_eq!(p.space(), false);
        assert!(p.is_end());

        let s = "abc ";
        let mut p = ParamParser::new(s);
        assert_eq!(p.space(), false);
        assert_eq!(p.is_end(), false);
        assert_eq!(p.value(), Some("abc"));
        assert!(p.space());
        assert!(p.is_end());

        let s = ",,  , ";
        let mut p = ParamParser::new(s);
        assert!(p.comma_or_space());
        assert!(p.comma_or_space());
        assert!(p.comma_or_space());
        assert_eq!(p.comma_or_space(), false);
        assert!(p.is_end());

        let s = "97,ab/5  / 10.7 ";
        let mut p = ParamParser::new(s);
        assert_eq!(p.slash(), false);
        assert_eq!(p.value(), Some("97"));
        assert_eq!(p.slash(), false);
        assert!(p.comma_or_space());
        assert_eq!(p.value(), Some("ab"));
        assert!(p.slash());
        assert_eq!(p.value(), Some("5"));
        assert!(p.slash());
        assert_eq!(p.value(), Some("10.7"));
        assert!(p.space());
        assert!(p.is_end());

        let s = "  ab(1 2,3),45 , xy cd / 10";
        let mut p = ParamParser::new(s);
        assert_eq!(p.value(), None);
        assert!(p.space());
        assert_eq!(p.space(), false);

        assert_eq!(p.value(), Some("ab(1 2,3)"));
        assert!(p.comma_or_space());

        assert_eq!(p.value(), Some("45"));
        assert!(p.comma_or_space());
        assert_eq!(p.value(), Some("xy"));
        assert!(p.comma_or_space());
        assert_eq!(p.value(), Some("cd"));
        assert!(p.comma_or_slash());
        assert_eq!(p.is_end(), false);
        assert_eq!(p.value(), Some("10"));
        assert_eq!(p.space(), false);
        assert_eq!(p.value(), None);
        assert!(p.is_end());

        let s = "2.53/9,dog   cat,fx(1 2 (56, 78))";
        let mut p = ParamParser::new(s);
        assert_eq!(p.value(), Some("2.53"));
        assert!(p.comma_or_slash());
        assert_eq!(p.value(), Some("9"));
        assert!(p.comma_or_space());
        assert_eq!(p.value(), Some("dog"));
        assert!(p.space());
        assert_eq!(p.value(), Some("cat"));
        assert!(p.comma_or_slash());
        assert_eq!(p.value(), Some("fx(1 2 (56, 78))"));
        assert_eq!(p.comma_or_slash(), false);
        assert!(p.is_end());

        let s = ") ( (9)) (";
        let mut p = ParamParser::new(s);
        assert_eq!(p.value(), Some(")"));
        assert!(p.space());
        assert_eq!(p.value(), Some("( (9))"));
        assert!(p.space());
        assert_eq!(p.value(), Some("("));
        assert!(p.is_end());
    }
}
