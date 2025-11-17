// Strip prefix ignore case.
pub fn strip_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix.len() > s.len() {
        return None;
    }
    let s_start = &s[..prefix.len()];
    if s_start.eq_ignore_ascii_case(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn strip_prefix_() {
        assert_eq!(strip_prefix("rgb(77)", "rgb"), Some("(77)"));
        assert_eq!(strip_prefix("RGB(0,0)", "rgb"), Some("(0,0)"));
        assert_eq!(strip_prefix("Hsv()", "HSV"), Some("()"));

        assert_eq!(strip_prefix("", "rgb"), None);
        assert_eq!(strip_prefix("10", "rgb"), None);
        assert_eq!(strip_prefix("hsv(0,0)", "hsva"), None);
        assert_eq!(strip_prefix("hsv", "hsva"), None);
    }
}
