use crate::{
    bits::{BoardMask, mask},
    model::{BoardFile, BoardRank, Color, Square, moves::PseudoMove},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const fn nil() -> Self {
        Self(0)
    }
    pub const fn full() -> Self {
        Self(0b_00001111)
    }

    pub const fn new(wooo: bool, woo: bool, booo: bool, boo: bool) -> Self {
        Self((booo as u8) << 3 | (boo as u8) << 2 | (wooo as u8) << 1 | woo as u8)
    }

    pub const fn get(self, c: Color) -> u8 {
        match c {
            Color::White => self.0 & 3,
            Color::Black => self.0 & 12,
        }
    }

    pub const fn westward(self, c: Color) -> bool {
        0 != match c {
            Color::White => self.0 & 1,
            Color::Black => self.0 & 4,
        }
    }

    pub const fn eastward(self, c: Color) -> bool {
        0 != match c {
            Color::White => self.0 & 2,
            Color::Black => self.0 & 8,
        }
    }

    #[must_use]
    pub const fn move_king(self, c: Color) -> Self {
        Self(match c {
            Color::White => self.0 & !3,
            Color::Black => self.0 & !12,
        })
    }

    #[must_use]
    pub const fn move_east_rook(self, c: Color) -> Self {
        Self(match c {
            Color::White => self.0 & !2,
            Color::Black => self.0 & !8,
        })
    }

    #[must_use]
    pub const fn move_west_rook(self, c: Color) -> Self {
        Self(match c {
            Color::White => self.0 & !1,
            Color::Black => self.0 & !4,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingDetails {
    pub capture_own_rook: bool,
    pub westward: CastlingDetail,
    pub eastward: CastlingDetail,
}

#[test]
fn castling_sizeof() {
    println!("size {}", size_of::<CastlingDetails>())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingDetail {
    pub rook_from: BoardFile,
    pub rook_to: BoardFile,
    pub king_from: BoardFile,
    pub king_to: BoardFile,
    pub threat_mask: u8,
    pub clear_mask: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingMove {
    pub rook_move: PseudoMove,
    pub king_move: PseudoMove,
    pub threat_mask: BoardMask,
    pub clear_mask: BoardMask,
}

impl CastlingDetail {
    pub const fn reify(self, c: Color) -> CastlingMove {
        let rank: BoardRank;

        match c {
            Color::White => rank = BoardRank::_1,
            Color::Black => rank = BoardRank::_8,
        };

        let rook_move = PseudoMove {
            from: self.rook_from.by(rank),
            to: self.rook_to.by(rank),
        };

        let king_move = PseudoMove {
            from: self.king_from.by(rank),
            to: self.king_to.by(rank),
        };

        let threat_mask: BoardMask;
        let clear_mask: BoardMask;

        match c {
            Color::White => {
                threat_mask = mask([0, 0, 0, 0, 0, 0, 0, self.threat_mask]);
                clear_mask = mask([0, 0, 0, 0, 0, 0, 0, self.clear_mask]);
            }
            Color::Black => {
                threat_mask = mask([self.threat_mask, 0, 0, 0, 0, 0, 0, 0]);
                clear_mask = mask([self.clear_mask, 0, 0, 0, 0, 0, 0, 0]);
            }
        }

        CastlingMove {
            rook_move,
            king_move,
            threat_mask,
            clear_mask,
        }
    }
}

pub const CLASSIC_CASTLING: CastlingDetails = CastlingDetails {
    capture_own_rook: false,
    westward: CastlingDetail {
        rook_from: BoardFile::A,
        rook_to: BoardFile::D,
        king_from: BoardFile::E,
        king_to: BoardFile::C,
        threat_mask: 0b_00111000,
        clear_mask: 0b_01110000,
    },
    eastward: CastlingDetail {
        rook_from: BoardFile::H,
        rook_to: BoardFile::F,
        king_from: BoardFile::E,
        king_to: BoardFile::G,
        threat_mask: 0b_00001110,
        clear_mask: 0b_00000110,
    },
};
