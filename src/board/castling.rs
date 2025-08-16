use crate::{
    bits::{Mask, mask},
    board::{Color, File, Rank, Square, moves::PseudoMove},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const fn new() -> Self {
        Self(0b_00001111)
    }

    pub const fn can_ooo(self, c: Color) -> bool {
        0 != match c {
            Color::White => self.0 & 1,
            Color::Black => self.0 & 4,
        }
    }

    pub const fn can_oo(self, c: Color) -> bool {
        0 != match c {
            Color::White => self.0 & 2,
            Color::Black => self.0 & 8,
        }
    }

    pub const fn lose(self, c: Color) -> Self {
        Self(match c {
            Color::White => self.0 & !3,
            Color::Black => self.0 & !12,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingDetails {
    ooo: CastlingDetail,
    oo: CastlingDetail,
}

#[test]
fn castling_sizeof() {
    println!("size {}", size_of::<CastlingDetails>())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingDetail {
    rook_from: File,
    rook_to: File,
    king_from: File,
    king_to: File,
    threat_mask: u8,
    clear_mask: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingMove {
    rook_move: PseudoMove,
    king_move: PseudoMove,
    threat_mask: Mask,
    clear_mask: Mask,
}

impl CastlingDetail {
    pub const fn reify(self, c: Color) -> CastlingMove {
        let rank: Rank;

        match c {
            Color::White => rank = Rank::_1,
            Color::Black => rank = Rank::_8,
        };

        let rook_move = PseudoMove {
            from: self.rook_from.by(rank),
            to: self.rook_to.by(rank),
        };

        let king_move = PseudoMove {
            from: self.king_from.by(rank),
            to: self.king_to.by(rank),
        };

        let threat_mask: Mask;
        let clear_mask: Mask;

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
    ooo: CastlingDetail {
        rook_from: File::A,
        rook_to: File::D,
        king_from: File::E,
        king_to: File::C,
        threat_mask: 0b_00111000,
        clear_mask: 0b_01110000,
    },
    oo: CastlingDetail {
        rook_from: File::H,
        rook_to: File::F,
        king_from: File::E,
        king_to: File::G,
        threat_mask: 0b_00001110,
        clear_mask: 0b_00000110,
    },
};
