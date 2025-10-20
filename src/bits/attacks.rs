use crate::{
    arrays::ArrayBoard,
    bits::{
        self, BoardMask, Squares,
        board::HalfBitBoard,
        jumps::{self, BLACK_PAWN_CAPTURE, KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE},
        one_bit,
        opdif::{bishop_rays, queen_rays, rook_rays},
        slides::{
            self, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH,
            RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST, simple_diagonal_attacks,
            simple_omnidirectional_attacks, simple_orthogonal_attacks,
        },
        two_bits,
    },
    model::{
        BoardFile, ChessPiece, Color, ColoredChessPiece, ColoredChessPieceWithCapture, Square,
        moves::PseudoMove,
    },
};

impl HalfBitBoard {
    pub fn attacks(&self, c: Color, enemy: BoardMask) -> BoardMask {
        use ChessPiece::*;
        let total = self.total | enemy;

        return pawn_attacks(self.pawns, c)
            | knight_attacks(self.knights)
            | king_attacks(self.kings)
            | rook_attacks(self.rooks, total)
            | bishop_attacks(self.bishops, total)
            | queen_attacks(self.queens, total);
    }

    pub fn attacks_after_enemy_move(
        &self,
        c: Color,
        enemy: BoardMask,
        mv: PseudoMove,
        cap_sq: Option<Square>,
        cap_p: Option<ChessPiece>,
    ) -> BoardMask {
        use ChessPiece::*;

        let enemy = enemy ^ mv.bits();
        let friendly = self.total ^ one_bit(cap_sq);
        let total = friendly | enemy;

        return pawn_attacks(self.pawns ^ is_cap(Pawn, cap_p, cap_sq), c)
            | knight_attacks(self.knights ^ is_cap(Knight, cap_p, cap_sq))
            | king_attacks(self.kings)
            | rook_attacks(self.rooks ^ is_cap(Rook, cap_p, cap_sq), total)
            | bishop_attacks(self.bishops ^ is_cap(Bishop, cap_p, cap_sq), total)
            | queen_attacks(self.queens ^ is_cap(Queen, cap_p, cap_sq), total);

        #[inline]
        fn is_cap(is: ChessPiece, cap: Option<ChessPiece>, sq: Option<Square>) -> BoardMask {
            match (cap, sq) {
                (Some(p), Some(sq)) if p == is => sq.bit(),
                _ => BoardMask::MIN,
            }
        }
    }

    pub fn attacks_after_friendly_move(
        &self,
        c: ColoredChessPieceWithCapture,
        enemy: BoardMask,
        mv: PseudoMove,
        cap: Option<Square>,
    ) -> BoardMask {
        use ChessPiece::*;

        let enemy = enemy ^ one_bit(cap);
        let friendly = self.total ^ mv.bits();
        let total = friendly | enemy;

        return pawn_attacks(self.pawns ^ is_mv(c, Pawn, mv), c.color())
            | knight_attacks(self.knights ^ is_mv(c, Knight, mv))
            | king_attacks(self.kings ^ is_mv(c, King, mv))
            | rook_attacks(self.rooks ^ is_mv(c, Rook, mv), total)
            | bishop_attacks(self.bishops ^ is_mv(c, Bishop, mv), total)
            | queen_attacks(self.queens ^ is_mv(c, Queen, mv), total);

        #[inline]
        fn is_mv(c: ColoredChessPieceWithCapture, is: ChessPiece, mv: PseudoMove) -> BoardMask {
            if c.piece() == is {
                mv.bits()
            } else {
                BoardMask::MIN
            }
        }
    }
}

#[inline]
pub fn pawn_attacks(p: BoardMask, c: Color) -> BoardMask {
    match c {
        Color::White => WHITE_PAWN_CAPTURE.overlay(p),
        Color::Black => BLACK_PAWN_CAPTURE.overlay(p),
    }
}

#[inline]
pub fn knight_attacks(n: BoardMask) -> BoardMask {
    KNIGHT_MOVES.overlay(n)
}

#[inline]
pub fn king_attacks(k: BoardMask) -> BoardMask {
    KING_MOVES.overlay(k)
}

#[inline]
pub fn rook_attacks(r: BoardMask, total: BoardMask) -> BoardMask {
    simple_orthogonal_attacks(r, total)
}

#[inline]
pub fn bishop_attacks(b: BoardMask, total: BoardMask) -> BoardMask {
    simple_diagonal_attacks(b, total)
}

#[inline]
pub fn queen_attacks(q: BoardMask, total: BoardMask) -> BoardMask {
    simple_omnidirectional_attacks(q, total)
}
