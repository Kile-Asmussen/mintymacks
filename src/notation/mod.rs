pub mod algebraic;
pub mod fen;
pub mod longalg;
pub mod pgn;
pub mod squares;
pub mod tests;
pub mod uci;

#[macro_export]
macro_rules! regexp {
    ($pat:literal) => {{
        lazy_static::lazy_static! {
            static ref PATTERN: regex::Regex =
                regex::Regex::new($pat).expect(concat!("invalid regex: `", $pat, "'"));
        }
        &PATTERN
    }};
}

pub use regexp;

#[test]
fn test() {
    assert!(regexp!("a").is_match("a"));
}
