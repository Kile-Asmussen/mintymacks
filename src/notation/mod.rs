pub mod algebraic;
pub mod fen;
pub mod longalg;
pub mod squares;
pub mod tests;
pub mod pgn;
pub mod uci;

#[macro_export]
macro_rules! regexp {
    ($pat:literal) => {{
        use regex::Regex;
        use std::sync::LazyLock;
        static PATTERN: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pat).expect(concat!("invalid regex: `", $pat, "'")));
        &PATTERN
    }};
}

pub use regexp;

#[test]
fn test() {
    assert!(regexp!("a").is_match("a"));
}
