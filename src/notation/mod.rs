use crate::{
    arrays::ArrayBoard,
    model::{
        Piece, Square,
        moves::{Move, PseudoMove, Special},
    },
};

pub mod fen;
pub mod longalg;
pub mod squares;
mod tests;

const COMMANDS: &[(&'static str, &'static str, fn(&[&str]))] = &[];
