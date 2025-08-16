use crate::{
    bits::Mask,
    board::{
        Color, Square,
        castling::{CastlingDetails, CastlingRights},
    },
};

pub struct HalfBitBoard {
    pub pawns: Mask,
    pub knights: Mask,
    pub bishops: Mask,
    pub rooks: Mask,
    pub queens: Mask,
    pub kings: Mask,
}

impl HalfBitBoard {
    pub const fn total(&self) -> Mask {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.kings
    }
}

pub struct BitBoard {
    pub white: HalfBitBoard,
    pub black: HalfBitBoard,
    pub to_move: Color,
    pub tempo: u16,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub castling_details: &'static CastlingDetails,
}
