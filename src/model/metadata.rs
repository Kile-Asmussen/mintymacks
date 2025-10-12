use std::default;

use crate::{
    model::{
        Color, Square,
        castling::{CLASSIC_CASTLING, CastlingDetails, CastlingRights},
    },
    zobrist::ZobHash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Metadata {
    pub to_move: Color = Color::White,
    pub turn: u16 = 1,
    pub castling_rights: CastlingRights = CastlingRights::full(),
    pub en_passant: Option<Square> = None,
    pub castling_details: CastlingDetails = CLASSIC_CASTLING,
    pub hash: ZobHash,
    pub halfmove_clock: u8,
}

impl Metadata {
    pub fn ply(&self) -> u16 {
        (self.turn - 1) * 2 + if self.to_move == Color::Black { 1 } else { 0 }
    }

    pub fn equiv(&self, other: &Self) -> bool {
        self.to_move == other.to_move
            && self.castling_rights == other.castling_rights
            && self.castling_details == other.castling_details
            && self.en_passant == other.en_passant
    }
}
