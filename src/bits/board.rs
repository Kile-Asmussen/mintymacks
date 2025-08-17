use crate::{
    arrays::ArrayBoard,
    bits::Mask,
    model::{
        Color, ColorPiece, Piece, Square,
        castling::{self, CastlingDetails, CastlingRights},
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
    pub const fn empty() -> Self {
        Self {
            pawns: Mask::MIN,
            knights: Mask::MIN,
            bishops: Mask::MIN,
            rooks: Mask::MIN,
            queens: Mask::MIN,
            kings: Mask::MIN,
        }
    }
    pub const fn new(color: Color, board: &ArrayBoard<Option<ColorPiece>>) -> Self {
        Self {
            pawns: board.mask(color.piece(Piece::Pawn)),
            knights: board.mask(color.piece(Piece::Knight)),
            bishops: board.mask(color.piece(Piece::Bishop)),
            rooks: board.mask(color.piece(Piece::Rook)),
            queens: board.mask(color.piece(Piece::Queen)),
            kings: board.mask(color.piece(Piece::King)),
        }
    }

    pub const fn total(&self) -> Mask {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.kings
    }

    pub const fn at(&self, sq: Square) -> Option<Piece> {
        let sq = sq.bit();
        if (self.pawns & sq) != 0 {
            Some(Piece::Pawn)
        } else if (self.knights & sq) != 0 {
            Some(Piece::Knight)
        } else if (self.bishops & sq) != 0 {
            Some(Piece::Bishop)
        } else if (self.rooks & sq) != 0 {
            Some(Piece::Rook)
        } else if (self.queens & sq) != 0 {
            Some(Piece::Queen)
        } else if (self.kings & sq) != 0 {
            Some(Piece::King)
        } else {
            None
        }
    }
}

pub struct BitBoard {
    pub white: HalfBitBoard,
    pub black: HalfBitBoard,
    pub to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub castling_details: CastlingDetails,
}

impl BitBoard {
    pub const fn new(
        board: &ArrayBoard<Option<ColorPiece>>,
        to_move: Color,
        castling_rights: CastlingRights,
        en_passant: Option<Square>,
        castling_details: CastlingDetails,
    ) -> Self {
        Self {
            white: HalfBitBoard::new(Color::White, board),
            black: HalfBitBoard::new(Color::Black, board),
            to_move,
            castling_rights,
            en_passant,
            castling_details,
        }
    }
}

#[test]
fn bitboard_sizeof() {
    println!("size {}", size_of::<BitBoard>())
}
