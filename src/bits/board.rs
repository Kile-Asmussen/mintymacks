use crate::{
    arrays::ArrayBoard,
    bits::BoardMask,
    model::{
        Color, ColoredChessPiece, ChessPiece, Square,
        castling::{self, CLASSIC_CASTLING, CastlingDetails, CastlingRights},
        metadata::Metadata,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HalfBitBoard {
    pub pawns: BoardMask,
    pub knights: BoardMask,
    pub bishops: BoardMask,
    pub rooks: BoardMask,
    pub queens: BoardMask,
    pub kings: BoardMask,
    pub total: BoardMask
}

impl HalfBitBoard {
    pub const fn empty() -> Self {
        Self {
            pawns: BoardMask::MIN,
            knights: BoardMask::MIN,
            bishops: BoardMask::MIN,
            rooks: BoardMask::MIN,
            queens: BoardMask::MIN,
            kings: BoardMask::MIN,
            total: BoardMask::MIN,
        }
    }
    pub const fn new(color: Color, board: &ArrayBoard<Option<ColoredChessPiece>>) -> Self {
        let mut res = Self {
            pawns: board.mask(color.piece(ChessPiece::Pawn)),
            knights: board.mask(color.piece(ChessPiece::Knight)),
            bishops: board.mask(color.piece(ChessPiece::Bishop)),
            rooks: board.mask(color.piece(ChessPiece::Rook)),
            queens: board.mask(color.piece(ChessPiece::Queen)),
            kings: board.mask(color.piece(ChessPiece::King)),
            total: 0,
        };
        res.total = res.pawns | res.knights | res.bishops | res.rooks | res.queens | res.kings;
        res
    }

    

    pub const fn at(&self, sq: Square) -> Option<ChessPiece> {
        let sq = sq.bit();
        if (self.pawns & sq) != 0 {
            Some(ChessPiece::Pawn)
        } else if (self.knights & sq) != 0 {
            Some(ChessPiece::Knight)
        } else if (self.bishops & sq) != 0 {
            Some(ChessPiece::Bishop)
        } else if (self.rooks & sq) != 0 {
            Some(ChessPiece::Rook)
        } else if (self.queens & sq) != 0 {
            Some(ChessPiece::Queen)
        } else if (self.kings & sq) != 0 {
            Some(ChessPiece::King)
        } else {
            None
        }
    }

    pub const fn render_to(&self, color: Color, board: &mut ArrayBoard<Option<ColoredChessPiece>>) {
        board.set_mask(self.pawns, Some(color.piece(ChessPiece::Pawn)));
        board.set_mask(self.knights, Some(color.piece(ChessPiece::Knight)));
        board.set_mask(self.bishops, Some(color.piece(ChessPiece::Bishop)));
        board.set_mask(self.rooks, Some(color.piece(ChessPiece::Rook)));
        board.set_mask(self.queens, Some(color.piece(ChessPiece::Queen)));
        board.set_mask(self.kings, Some(color.piece(ChessPiece::King)));
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
        board: &ArrayBoard<Option<ColoredChessPiece>>,
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
        use ColoredChessPiece::*;
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
            1,
            CastlingRights::full(),
            None,
            CLASSIC_CASTLING,
        )
    }

    pub const fn render(&self) -> ArrayBoard<Option<ColoredChessPiece>> {
        let mut res = ArrayBoard::new(None);
        self.white.render_to(Color::White, &mut res);
        self.black.render_to(Color::Black, &mut res);
        res
    }

    pub const fn at(&self, sq: Square) -> Option<ColoredChessPiece> {
        if let Some(p) = self.white.at(sq) {
            Some(p.color(Color::White))
        } else if let Some(p) = self.black.at(sq) {
            Some(p.color(Color::Black))
        } else {
            None
        }
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        Self::startpos()
    }
}

#[test]
fn bitboard_sizeof() {
    println!("size {}", size_of::<BitBoard>())
}
