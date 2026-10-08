use super::strip_prefix;

const MAX_DEPTH: usize = 65;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token<'a> {
    Num(f32),
    Ident(&'a str),
    Op(Op),
    LParen,
    RParen,
    Unknown,
}

struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
    peeked: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    #[inline]
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
            peeked: None,
        }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        if let Some(tok) = self.peeked.take() {
            return Some(tok);
        }

        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return None;
        }

        let b = self.input[self.pos];
        self.pos += 1;

        match b {
            b'+' => Some(Token::Op(Op::Add)),
            b'-' => Some(Token::Op(Op::Sub)),
            b'*' => Some(Token::Op(Op::Mul)),
            b'/' => Some(Token::Op(Op::Div)),
            b'(' => Some(Token::LParen),
            b')' => Some(Token::RParen),
            b'0'..=b'9' | b'.' => {
                let start = self.pos - 1;
                let mut dot_seen = b == b'.';

                while self.pos < self.input.len() {
                    let c = self.input[self.pos];
                    if c.is_ascii_digit() {
                        self.pos += 1;
                    } else if c == b'.' && !dot_seen {
                        dot_seen = true; // Break on second dot to avoid failing f32 parse
                        self.pos += 1;
                    } else {
                        break;
                    }
                }

                // Safe: we only advanced over ascii digits and dots
                let num = unsafe { core::str::from_utf8_unchecked(&self.input[start..self.pos]) };

                if let Ok(val) = num.parse::<f32>() {
                    Some(Token::Num(val))
                } else {
                    Some(Token::Unknown) // Triggered by isolated dots (e.g., ".")
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' => {
                let start = self.pos - 1;
                while self.pos < self.input.len() && self.input[self.pos].is_ascii_alphabetic() {
                    self.pos += 1;
                }

                // Safe: we only advanced over ascii alphabetic characters.
                let ident = unsafe { core::str::from_utf8_unchecked(&self.input[start..self.pos]) };
                Some(Token::Ident(ident))
            }
            _ => Some(Token::Unknown),
        }
    }

    fn peek_token(&mut self) -> Option<Token<'a>> {
        if self.peeked.is_none() {
            self.peeked = self.next_token();
        }
        self.peeked
    }
}

fn binding_power(op: Op) -> (u8, u8) {
    match op {
        Op::Add | Op::Sub => (1, 2),
        Op::Mul | Op::Div => (3, 4),
    }
}

fn eval_expr_inner<'a, F>(
    lexer: &mut Lexer<'a>,
    min_bp: u8,
    vars: &F,
    depth: usize,
) -> Result<f32, &'static str>
where
    F: Fn(&str) -> Option<f32>,
{
    if depth > MAX_DEPTH {
        return Err("Recursion limit exceeded");
    }

    let mut left = match lexer.next_token() {
        Some(Token::Num(n)) => n,
        Some(Token::Ident(name)) => vars(name).ok_or("Undefined variable")?,
        Some(Token::Op(Op::Sub)) => {
            let val = eval_expr_inner(lexer, 5, vars, depth + 1)?;
            -val
        }
        Some(Token::Op(Op::Add)) => eval_expr_inner(lexer, 5, vars, depth + 1)?,
        Some(Token::LParen) => {
            let val = eval_expr_inner(lexer, 0, vars, depth + 1)?;
            match lexer.next_token() {
                Some(Token::RParen) => val,
                _ => return Err("Expected closing parenthesis"),
            }
        }
        _ => return Err("Expected number, variable, or opening parenthesis"),
    };

    while let Some(Token::Op(op)) = lexer.peek_token() {
        let (l_bp, r_bp) = binding_power(op);
        if l_bp < min_bp {
            break;
        }

        lexer.next_token(); // Consume operator
        let right = eval_expr_inner(lexer, r_bp, vars, depth + 1)?;

        left = match op {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        };
    }

    Ok(left)
}

fn eval_expr<'a, F>(lexer: &mut Lexer<'a>, vars: &F) -> Result<f32, &'static str>
where
    F: Fn(&str) -> Option<f32>,
{
    let val = eval_expr_inner(lexer, 0, vars, 0)?;

    // Ensure all tokens are consumed
    if lexer.peek_token().is_some() {
        return Err("Unexpected trailing token");
    }

    Ok(val)
}

pub(crate) fn calc_values(
    values: [&str; 4],
    variables: [(&str, f32); 4],
) -> Option<[(f32, bool); 4]> {
    let parse_v = |s: &str| -> Option<f32> {
        if let Ok(val) = s.parse::<f32>() {
            if val.is_finite() {
                return Some(val);
            }
            return None;
        };
        for (var, val) in variables {
            if s.eq_ignore_ascii_case(var) {
                return Some(val);
            }
        }
        None
    };

    let mut result = [(0.0, false); 4];

    for (i, s) in values.iter().enumerate() {
        // none
        if s.eq_ignore_ascii_case("none") {
            continue;
        }

        // bare number or keyword
        if let Some(t) = parse_v(s) {
            result[i].0 = t;
            continue;
        }

        // percentage
        if let Some(s) = s.strip_suffix('%') {
            if let Ok(v) = s.parse::<f32>() {
                if v.is_finite() {
                    result[i] = (v / 100.0, true);
                    continue;
                }
            };
            return None;
        }

        // calc(...)
        if let Some(s) = strip_prefix(s, "calc(") {
            if let Some(s) = s.strip_suffix(')') {
                let mut lexer = Lexer::new(s);
                if let Ok(v) = eval_expr(&mut lexer, &parse_v) {
                    if v.is_finite() {
                        result[i].0 = v;
                        continue;
                    }
                }
            }
        }

        // invalid value
        return None;
    }

    Some(result)
}

#[cfg(test)]
mod t {
    use super::*;

    #[track_caller]
    fn assert_near(a: f32, b: f32, eps: f32, s: &str) {
        if (a - b).abs() > eps {
            panic!("assertion failed: {s:?}\n  left: `{a:?}`\n right: `{b:?}`");
        }
    }

    #[test]
    fn parse_calc_() {
        fn f(s: &str) -> Option<f32> {
            s.parse().ok()
        }

        let test_data = [
            ("9", 9.0),
            (".1", 0.1),
            ("(3)", 3.0),
            ("1+7", 8.0),
            ("(1+3.07)", 4.07),
            ("( 0.35 - -0.5 )", 0.85),
            ("(2.0*(7-5))", 4.0),
            ("((5*10) / (7+3))", 5.0),
            ("(0.5 * (5 + (7 * (9 - (3 * (1 + 1))))))", 13.0),
            ("((1+3))", 4.0),
            ("(5+(1+2/3))", 2.0 / 3.0 + 6.0),
        ];

        for (s, expected) in test_data {
            let mut lexer = Lexer::new(s);
            let v = eval_expr(&mut lexer, &f);

            assert!(v.is_ok(), "{s:?}");
            assert_near(v.unwrap(), expected, 0.000001, s);
        }

        let invalids = [
            "",
            "g",
            "()",
            "(())",
            "(())",
            "(()+(1*5))",
            "(1-8",
            "7+0.3)",
            "(5+(3*2)",
            "((5-1)",
            "(4+5(1*3))",
            "((1+2)1*5)",
        ];

        for s in invalids {
            let mut lexer = Lexer::new(s);
            let v = eval_expr(&mut lexer, &f);

            assert!(v.is_err(), "{s:?}");
        }
    }

    #[test]
    fn parse_values_() {
        fn parse_value(s: &str, variables: [(&str, f32); 4]) -> Option<f32> {
            if let Some([t, ..]) = calc_values([s, "1", "1", "1"], variables) {
                return Some(t.0);
            }
            None
        }

        let vars = [("r", 255.0), ("g", 127.0), ("b", 0.0), ("alpha", 0.5)];

        let test_data = [
            // simple value
            ("130", 130.0),
            ("-0.5", -0.5),
            ("g", 127.0),
            ("none", 0.0),
            // calc() simple
            ("calc(5)", 5.0),
            ("calc(+13)", 13.0),
            ("calc(g)", 127.0),
            ("calc(4+5.5)", 9.5),
            ("calc( 10 - 7 )", 3.0),
            ("CALC(2.5 *2)", 5.0),
            ("CaLc(21.0/ 3)", 7.0),
            ("calc(r-55)", 200.0),
            ("calc(10 + g)", 137.0),
            ("calc(alpha*1.5)", 0.75),
            // calc() complex
            ("calc(5+1-4)", 2.0),
            ("calc(5+(1.5))", 6.5),
            ("calc(5+(1.5*2/3))", 6.0),
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
            assert_eq!(parse_value(s, vars), Some(expected), "{s:?}");
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
            "calc(g-)",
            "calc(1 * 7 +)",
            "calc(5 + (2 - ab))",
            "calc(nan)",
            "calc(1 + inf)",
            "calc(1 + infinity)",
            "calc(2 - (3 + 1e400))",
        ];

        for s in invalids {
            assert_eq!(parse_value(s, vars), None, "{s:?}");
        }
    }
}
