use crate::{
    arrays::ArrayBoard,
    bits::Mask,
    model::{
        Color, ColorPiece, Piece, Square,
        castling::{self, CLASSIC_CASTLING, CastlingDetails, CastlingRights},
        metadata::Metadata,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub const fn render_to(&self, color: Color, board: &mut ArrayBoard<Option<ColorPiece>>) {
        board.set_mask(self.pawns, Some(color.piece(Piece::Pawn)));
        board.set_mask(self.knights, Some(color.piece(Piece::Knight)));
        board.set_mask(self.bishops, Some(color.piece(Piece::Bishop)));
        board.set_mask(self.rooks, Some(color.piece(Piece::Rook)));
        board.set_mask(self.queens, Some(color.piece(Piece::Queen)));
        board.set_mask(self.kings, Some(color.piece(Piece::King)));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitBoard {
    pub white: HalfBitBoard,
    pub black: HalfBitBoard,
    pub metadata: Metadata,
}

#[test]
fn bitmetadata_sizeof() {
    println!("{}", size_of::<Metadata>())
}

impl BitBoard {
    pub const fn new(
        board: &ArrayBoard<Option<ColorPiece>>,
        to_move: Color,
        turn: u16,
        castling_rights: CastlingRights,
        en_passant: Option<Square>,
        castling_details: CastlingDetails,
    ) -> Self {
        Self {
            white: HalfBitBoard::new(Color::White, board),
            black: HalfBitBoard::new(Color::Black, board),
            metadata: Metadata {
                to_move,
                turn,
                castling_rights,
                en_passant,
                castling_details,
            },
        }
    }

    pub const fn startpos() -> Self {
        use ColorPiece::*;
        Self::new(
            &ArrayBoard::setup([
                [
                    Some(BlackRook),
                    Some(BlackKnight),
                    Some(BlackBishop),
                    Some(BlackQueen),
                    Some(BlackKing),
                    Some(BlackBishop),
                    Some(BlackKnight),
                    Some(BlackRook),
                ],
                [Some(BlackPawn); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(WhitePawn); 8],
                [
                    Some(WhiteRook),
                    Some(WhiteKnight),
                    Some(WhiteBishop),
                    Some(WhiteQueen),
                    Some(WhiteKing),
                    Some(WhiteBishop),
                    Some(WhiteKnight),
                    Some(WhiteRook),
                ],
            ]),
            Color::White,
            0,
            CastlingRights::full(),
            None,
            CLASSIC_CASTLING,
        )
    }

    pub const fn render(&self) -> ArrayBoard<Option<ColorPiece>> {
        let mut res = ArrayBoard::new(None);
        self.white.render_to(Color::White, &mut res);
        self.black.render_to(Color::Black, &mut res);
        res
    }

    pub const fn at(&self, sq: Square) -> Option<ColorPiece> {
        if let Some(p) = self.white.at(sq) {
            Some(p.color(Color::White))
        } else if let Some(p) = self.black.at(sq) {
            Some(p.color(Color::Black))
        } else {
            None
        }
    }
}

#[test]
fn bitboard_sizeof() {
    println!("size {}", size_of::<BitBoard>())
}
