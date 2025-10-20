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
        static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            regex::Regex::new($pat).expect(concat!("invalid regex: `", $pat, "'"))
        });
        &PATTERN
    }};
}

pub use regexp;

use crate::model::{
    ChessPiece,
    moves::{ChessMove, PseudoMove, SpecialMove},
};

pub type LongAlg = (PseudoMove, Option<ChessPiece>);

pub trait MoveMatcher {
    fn matches(&self, cm: ChessMove) -> bool;
}

impl MoveMatcher for LongAlg {
    fn matches(&self, cm: ChessMove) -> bool {
        match self.1 {
            None => self.0 == cm.pmv,
            Some(p) => cm.spc == Some(SpecialMove::Promotion(p)),
        }
    }
}

impl MoveMatcher for ChessMove {
    fn matches(&self, cm: ChessMove) -> bool {
        *self == cm
    }
}
