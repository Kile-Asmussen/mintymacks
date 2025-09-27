pub mod fen;
pub mod longalg;
pub mod squares;
mod tests;

#[macro_export]
macro_rules! regex {
    ($pat:literal) => {{
        use regex::Regex;
        use std::sync::LazyLock;
        static PATTERN: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pat).expect(concat!("invalid regex: `", $pat, "'")));
        &PATTERN
    }};
}

#[test]
fn test() {
    assert!(regex!("a").is_match("a"));
}
