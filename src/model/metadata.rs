use std::default;

use crate::model::{
    Color, Square,
    castling::{CLASSIC_CASTLING, CastlingDetails, CastlingRights},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Metadata {
    pub to_move: Color = Color::White,
    pub turn: u16 = 1,
    pub castling_rights: CastlingRights = CastlingRights::full(),
    pub en_passant: Option<Square> = None,
    pub castling_details: CastlingDetails = CLASSIC_CASTLING,
}

impl Metadata {
    pub fn ply(&self) -> u16 {
        (self.turn - 1) * 2 + if self.to_move == Color::Black { 1 } else { 0 }
    }
}
