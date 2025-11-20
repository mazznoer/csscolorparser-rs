use super::strip_prefix;

struct CalcParser<'a> {
    s: &'a str,
    idx: usize,
}

impl<'a> CalcParser<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, idx: 0 }
    }

    // Returns everything until operator is found.
    // Ignore operator inside parentheses.
    fn operand(&mut self) -> Option<&'a str> {
        if self.is_end() {
            return None;
        }

        let start = self.idx;

        match self.s.as_bytes()[self.idx] {
            b'-' => self.idx += 1,
            b'+' => return None,
            b'*' => return None,
            b'/' => return None,
            _ => (),
        }

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
                b'+' | b'-' | b'*' | b'/' | b' ' => {
                    if nesting == 0 {
                        // operator is *outside* parentheses
                        break;
                    }
                    self.idx += 1;
                }
                _ => self.idx += 1,
            }
        }

        Some(&self.s[start..self.idx])
    }

    // Returns first operator found. Skip spaces.
    fn operator(&mut self) -> Option<u8> {
        if self.is_end() {
            return None;
        }

        let ch = self.s.as_bytes()[self.idx];
        match ch {
            b'+' | b'-' | b'*' | b'/' => {
                self.idx += 1;
                Some(ch)
            }
            _ => None,
        }
    }

    fn is_end(&mut self) -> bool {
        // Consume all spaces until other character is found.
        while self.idx < self.s.len() && self.s.as_bytes()[self.idx] == b' ' {
            self.idx += 1;
        }
        self.idx >= self.s.len()
    }

    fn parse(&mut self) -> Option<(&str, u8, &str)> {
        if let (Some(va), Some(op), Some(vb), true) = (
            self.operand(),
            self.operator(),
            self.operand(),
            self.is_end(),
        ) {
            Some((va, op, vb))
        } else {
            None
        }
    }
}

pub fn parse_value(s: &str, variables: [(&str, f32); 4]) -> Option<f32> {
    let parse_v = |s: &str| -> Option<f32> {
        if let Ok(value) = s.parse() {
            return Some(value);
        };
        for (var, value) in variables {
            if s.eq_ignore_ascii_case(var) {
                return Some(value);
            }
        }
        None
    };

    if let Some(t) = parse_v(s) {
        return Some(t);
    }

    if let Some(s) = strip_prefix(s, "calc") {
        return parse_calc(s, &parse_v);
    }

    None
}

fn parse_calc<F>(s: &str, f: &F) -> Option<f32>
where
    F: Fn(&str) -> Option<f32>,
{
    if let Some(s) = s.strip_prefix('(') {
        if let Some(s) = s.strip_suffix(')') {
            let mut p = CalcParser::new(s);
            let (va, op, vb) = p.parse()?;

            let va = if let Some(v) = f(va) {
                v
            } else if let Some(v) = parse_calc(va, f) {
                v
            } else {
                return None;
            };

            let vb = if let Some(v) = f(vb) {
                v
            } else if let Some(v) = parse_calc(vb, f) {
                v
            } else {
                return None;
            };

            match op {
                b'+' => return Some(va + vb),
                b'-' => return Some(va - vb),
                b'*' => return Some(va * vb),
                b'/' => {
                    if vb == 0.0 {
                        return None;
                    }
                    return Some(va / vb);
                }
                _ => unreachable!(),
            }
        }
    }

    None
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn calc_parser() {
        let s = "78+0.573";
        let mut p = CalcParser::new(s);
        assert_eq!(p.operator(), None);
        assert_eq!(p.operand(), Some("78"));
        assert_eq!(p.operand(), None);
        assert_eq!(p.operator(), Some(b'+'));
        assert_eq!(p.operator(), None);
        assert_eq!(p.operand(), Some("0.573"));
        assert_eq!(p.operator(), None);
        assert_eq!(p.operand(), None);
        assert!(p.is_end());
        assert_eq!(p.parse(), None);

        #[rustfmt::skip]
        let test_data = [
            (
                "78+0.573",
                ("78", b'+', "0.573"),
            ),
            (
                "g-100",
                ("g", b'-', "100"),
            ),
            (
                " 9 * alpha  ",
                ("9", b'*', "alpha"),
            ),
            (
                "alpha/2",
                ("alpha", b'/', "2"),
            ),
            (
                "-360+-55.07",
                ("-360", b'+', "-55.07"),
            ),
            (
                "-7--5",
                ("-7", b'-', "-5"),
            ),
            (
                "h+(4*0.75)",
                ("h", b'+', "(4*0.75)"),
            ),
            (
                "(0.35*r) / (alpha - 10)",
                ("(0.35*r)", b'/', "(alpha - 10)"),
            ),
        ];
        for (s, expected) in test_data {
            let mut p = CalcParser::new(s);
            assert_eq!(p.parse(), Some(expected), "{:?}", s);
            assert!(p.is_end(), "{:?}", s);
        }

        #[rustfmt::skip]
        let invalids = [
            "",
            " ",
            "5",
            "g+",
            "-",
            "7---3",
            "*3+2",
            "4+5/",
        ];
        for s in invalids {
            let mut p = CalcParser::new(s);
            assert_eq!(p.parse(), None, "{:?}", s);
        }
    }

    #[test]
    fn parse_calc_() {
        fn f(s: &str) -> Option<f32> {
            s.parse().ok()
        }

        let test_data = [
            ("(1+3.7)", 4.7),
            ("( 0.35 - -0.5 )", 0.85),
            ("(2.0*(7-5))", 4.0),
            ("((5*10) / (7+3))", 5.0),
            ("(0.5 * (5 + (7 * (9 - (3 * (1 + 1))))))", 13.0),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_calc(s, &f), Some(expected), "{:?}", s);
        }

        let invalids = [
            "",
            "5",
            "g",
            "1+7",
            "()",
            "(())",
            "(())",
            "(()+(1*5))",
            "(9)",
            "(4/0)",
            "(1-8",
            "7+0.3)",
            "(5+(3*2)",
            "((5-1)",
            "((1+2))",
            "(5+(1+2/3))",
            "(4+5(1*3))",
            "((1+2)1*5)",
        ];
        for s in invalids {
            assert_eq!(parse_calc(s, &f), None, "{:?}", s);
        }
    }

    #[test]
    fn parse_value_() {
        let vars = [("r", 255.0), ("g", 127.0), ("b", 0.0), ("alpha", 0.5)];
        let test_data = [
            // simple value
            ("130", 130.0),
            ("-0.5", -0.5),
            ("g", 127.0),
            // calc() simple
            ("calc(4+5.5)", 9.5),
            ("calc( 10 - 7 )", 3.0),
            ("CALC(2.5 *2)", 5.0),
            ("CaLc(21.0/ 3)", 7.0),
            ("calc(r-55)", 200.0),
            ("calc(10 + g)", 137.0),
            ("calc(alpha*1.5)", 0.75),
            // calc() negative number
            ("calc(-97+-18)", -115.0),
            ("calc( -1 * -45)", 45.0),
            ("calc(100--35)", 135.0),
            ("calc(100 - -35)", 135.0),
            // calc() recursive
            ("calc(1.5*(4/2))", 3.0),
            ("calc( ( 19 + 6 ) / 5 )", 5.0),
            ("calc((2/(1.5+0.5)) - (0.75 - 0.25))", 0.5),
            ("calc((r + g) / 2)", 191.0),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_value(s, vars), Some(expected), "{:?}", s);
        }

        let invalids = [
            "",
            "7x",
            "h",
            "(4+5)",
            "cal(4+5)",
            "calcs(4+5)",
            "calc()",
            "calc(-)",
            "calc(5)",
            "calc(+5)",
            "calc(b)",
            "calc(g-)",
            "calc(5+1-4)",
            "calc(1 * 7 +)",
            "calc(5 + (1.5))",
            "calc(5 + (1.5 * 2 / 3))",
            "calc(5 + (2 - ab))",
        ];
        for s in invalids {
            assert_eq!(parse_value(s, vars), None, "{:?}", s);
        }
    }
}
