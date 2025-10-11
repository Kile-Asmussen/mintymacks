use std::collections::HashMap;

use crate::{
    bits::board::{BitBoard, HalfBitBoard},
    model::{ChessPiece, Color, Victory, moves::ChessMove},
    zobrist::ZobHash,
};

impl Victory {
    pub fn from_color(c: Color) -> Self {
        match c {
            Color::White => Self::WhiteWins,
            Color::Black => Self::BlackWins,
        }
    }

    pub fn determine(
        board: &BitBoard,
        hash: ZobHash,
        moves: &[ChessMove],
        halfmove: u16,
        seen_positions: &HashMap<ZobHash, u8>,
    ) -> Option<Self> {
        let (active, passive) = board.active_passive(board.metadata.to_move);

        if moves.is_empty()
            && (active.kings & passive.attacks(board.metadata.to_move.opposite(), active.total)
                != 0)
        {
            return Some(Self::from_color(board.metadata.to_move.opposite()));
        }

        if halfmove >= 150 {
            return Some(Self::Draw);
        }

        if let Some(3..) = seen_positions.get(&hash) {
            return Some(Self::Draw);
        }

        if !active.sufficient() && !passive.sufficient() {
            return Some(Self::Draw);
        }

        None
    }
}

impl HalfBitBoard {
    fn count_pawns(&self) -> u32 {
        self.pawns.count_ones()
    }

    fn count_knights(&self) -> u32 {
        self.knights.count_ones()
    }

    fn count_bishops(&self) -> (u32, u32) {
        (
            (self.bishops & 0x5555_5555_5555_5555).count_ones(),
            (self.bishops & 0xAAAA_AAAA_AAAA_AAAA).count_ones(),
        )
    }

    fn count_rooks(&self) -> u32 {
        self.rooks.count_ones()
    }

    fn count_queens(&self) -> u32 {
        self.rooks.count_ones()
    }

    fn sufficient(&self) -> bool {
        if self.count_pawns() > 0 {
            return true;
        }

        if self.count_knights() > 2 {
            return true;
        }

        if let (1.., 1..) = self.count_bishops() {
            return true;
        }

        if self.count_rooks() > 0 {
            return true;
        }

        if self.count_queens() > 0 {
            return true;
        }

        return false;
    }
}

impl ChessMove {
    fn reversible(self) -> bool {
        if self.piece.piece() == ChessPiece::Pawn {
            return false;
        }

        if self.special.is_some() {
            return false;
        }

        if self.cap.is_some() {
            return false;
        }

        return true;
    }
}
