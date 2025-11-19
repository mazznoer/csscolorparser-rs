pub struct ParamParser<'a> {
    s: &'a str,
    idx: usize,
}

impl<'a> ParamParser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s, idx: 0 }
    }

    // Returns `&str` from current index until space, comma, or slash is found.
    // Ignore space, comma, or slash inside parentheses.
    // Returns `None` if value not found.
    pub fn value(&mut self) -> Option<&'a str> {
        if self.is_end() {
            return None;
        }

        match self.s.as_bytes()[self.idx] {
            b' ' => return None,
            b',' => return None,
            b'/' => return None,
            _ => (),
        }

        let start = self.idx;

        // parenthesis depth
        let mut nesting = 0i32;

        while self.idx < self.s.len() {
            let ch = self.s.as_bytes()[self.idx];
            match ch {
                b'(' => {
                    nesting += 1;
                    self.idx += 1;
                }
                b')' => {
                    if nesting > 0 {
                        nesting -= 1;
                    }
                    self.idx += 1;
                }
                b' ' | b',' | b'/' => {
                    if nesting == 0 {
                        // delimiter is *outside* parentheses
                        break;
                    }
                    self.idx += 1;
                }
                _ => self.idx += 1,
            }
        }

        Some(&self.s[start..self.idx])
    }

    // Consume one or more spaces.
    // Returns true if space is found, false otherwise.
    pub fn space(&mut self) -> bool {
        let mut found = false;
        while self.idx < self.s.len() && self.s.as_bytes()[self.idx] == b' ' {
            self.idx += 1;
            found = true;
        }
        found
    }

    // Consume one or more spaces, or single comma.
    // Spaces is allowed around comma.
    // Returns true if one of them is found, false otherwise.
    pub fn comma_or_space(&mut self) -> bool {
        let mut found_comma = false;
        let mut found_space = false;

        while self.idx < self.s.len() {
            let ch = self.s.as_bytes()[self.idx];
            match ch {
                b' ' => {
                    found_space = true;
                    self.idx += 1;
                }
                b',' => {
                    if found_comma {
                        break;
                    }
                    found_comma = true;
                    self.idx += 1;
                }
                _ => {
                    break;
                }
            }
        }

        found_comma || found_space
    }

    // Consume single comma or single slash.
    // Spaces is allowed around comma or slash.
    // Returns true if one of them is found, false otherwise.
    pub fn comma_or_slash(&mut self) -> bool {
        let mut found = false;

        while self.idx < self.s.len() {
            let ch = self.s.as_bytes()[self.idx];
            match ch {
                b' ' => {
                    self.idx += 1;
                }
                b',' | b'/' => {
                    if found {
                        break;
                    }
                    found = true;
                    self.idx += 1;
                }
                _ => {
                    break;
                }
            }
        }

        found
    }

    // Consume a single slash. Spaces is allowed around slash.
    // Returns true if a slash is found, false otherwise.
    pub fn slash(&mut self) -> bool {
        let mut found = false;

        while self.idx < self.s.len() {
            let ch = self.s.as_bytes()[self.idx];
            match ch {
                b' ' => {
                    self.idx += 1;
                }
                b'/' => {
                    if found {
                        break;
                    }
                    found = true;
                    self.idx += 1;
                }
                _ => {
                    break;
                }
            }
        }

        found
    }

    // Returns true is we finished reading the str.
    pub fn is_end(&self) -> bool {
        self.idx >= self.s.len()
    }
}

#[cfg(test)]
mod t {
    use super::*;

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
