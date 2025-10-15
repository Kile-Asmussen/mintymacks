use crate::{
    arrays::ArrayBoard,
    bits::{
        self, Bits, BoardMask, bit,
        board::HalfBitBoard,
        fills::{fill_bishop2, fill_bishop4, fill_queen2, fill_queen4, fill_rook2, fill_rook4},
        jumps::{self, BLACK_PAWN_CAPTURE, KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE},
        obstruction_difference,
        slides::{
            self, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH,
            RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST,
        },
        two_bits,
    },
    model::{
        ChessPiece, Color, ColoredChessPiece, ColoredChessPieceWithCapture, Square,
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
        let friendly = self.total ^ bit(cap_sq);
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

        let enemy = enemy ^ bit(cap);
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

    pub fn pieces(&self, amount: i8, res: &mut ArrayBoard<i8>) {
        res.add(self.pawns, amount);
        res.add(self.knights, amount);
        res.add(self.bishops, amount);
        res.add(self.rooks, amount);
        res.add(self.queens, amount);
        res.add(self.kings, amount);
    }

    pub fn materiel(&self, scale: i16, res: &mut ArrayBoard<i16>) {
        res.add(self.pawns, scale * ChessPiece::PAWN);
        res.add(self.knights, scale * ChessPiece::KNIGHT);
        res.add(self.bishops, scale * ChessPiece::BISHOP);
        res.add(self.rooks, scale * ChessPiece::ROOK);
        res.add(self.queens, scale * ChessPiece::QUEEN);
    }

    pub fn count_attackers(
        &self,
        c: Color,
        amount: i8,
        enemy: BoardMask,
        res: &mut ArrayBoard<i8>,
    ) {
        let total = self.total | enemy;
        count_pawn_attackers(self.pawns, c, amount, res);
        count_knight_attackers(self.knights, amount, res);
        count_bishop_attackers(self.bishops, total, amount, res);
        count_rook_attackers(self.rooks, total, amount, res);
        count_queen_attackers(self.queens, total, amount, res);
        count_king_attackers(self.kings, amount, res);
    }

    pub fn count_attacker_materiel(
        &self,
        c: Color,
        enemy: BoardMask,
        scale: i16,
        res: &mut ArrayBoard<i16>,
    ) {
        let total = self.total | enemy;
        count_pawn_attacker_materiel(self.pawns, c, scale, res);
        count_knight_attacker_materiel(self.knights, scale, res);
        count_bishop_attacker_materiel(self.bishops, total, scale, res);
        count_rook_attacker_materiel(self.rooks, total, scale, res);
        count_queen_attacker_materiel(self.queens, total, scale, res);
    }
}

pub fn pawn_attacks(p: BoardMask, c: Color) -> BoardMask {
    match c {
        Color::White => WHITE_PAWN_CAPTURE.overlay(p),
        Color::Black => BLACK_PAWN_CAPTURE.overlay(p),
    }
}

pub fn knight_attacks(n: BoardMask) -> BoardMask {
    KNIGHT_MOVES.overlay(n)
}

pub fn king_attacks(k: BoardMask) -> BoardMask {
    KING_MOVES.overlay(k)
}

#[inline]
pub fn ortho_rays(sq: Square, total: BoardMask) -> BoardMask {
    obstruction_difference(RAYS_SOUTH.at(sq), RAYS_NORTH.at(sq), total)
        | obstruction_difference(RAYS_WEST.at(sq), RAYS_EAST.at(sq), total)
}

#[inline]
pub fn diag_rays(sq: Square, total: BoardMask) -> BoardMask {
    obstruction_difference(RAYS_SOUTHWEST.at(sq), RAYS_NORTHEAST.at(sq), total)
        | obstruction_difference(RAYS_SOUTHEAST.at(sq), RAYS_NORTHWEST.at(sq), total)
}

pub fn rook_attacks(r: BoardMask, total: BoardMask) -> BoardMask {
    fill_rook4(r, total)
    // let mut res = BoardMask::MIN;
    // for sq in Bits(r) {
    //     res |= ortho_rays(sq, total);
    // }
    // res
}

pub fn bishop_attacks(b: BoardMask, total: BoardMask) -> BoardMask {
    fill_bishop4(b, total)
    // let mut res = BoardMask::MIN;
    // for sq in Bits(b) {
    //     res |= diag_rays(sq, total);
    // }
    // res
}

pub fn queen_attacks(q: BoardMask, total: BoardMask) -> BoardMask {
    // fill_bishop4(q, total) | fill_rook4(q, total)
    let mut res = BoardMask::MIN;
    for sq in Bits(q) {
        res |= ortho_rays(sq, total);
        res |= diag_rays(sq, total);
    }
    res
}

pub fn count_pawn_attacker_materiel(p: BoardMask, c: Color, scale: i16, res: &mut ArrayBoard<i16>) {
    for sq in Bits(p) {
        res.add(
            match c {
                Color::White => WHITE_PAWN_CAPTURE,
                Color::Black => BLACK_PAWN_CAPTURE,
            }
            .at(sq),
            scale * ChessPiece::PAWN,
        );
    }
}

pub fn count_pawn_attackers(p: BoardMask, c: Color, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Bits(p) {
        res.add(
            match c {
                Color::White => WHITE_PAWN_CAPTURE,
                Color::Black => BLACK_PAWN_CAPTURE,
            }
            .at(sq),
            amount,
        );
    }
}

pub fn count_knight_attacker_materiel(p: BoardMask, scale: i16, res: &mut ArrayBoard<i16>) {
    for sq in Bits(p) {
        res.add(KNIGHT_MOVES.at(sq), scale * ChessPiece::KNIGHT);
    }
}

pub fn count_knight_attackers(p: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Bits(p) {
        res.add(KNIGHT_MOVES.at(sq), amount);
    }
}

pub fn count_bishop_attacker_materiel(
    p: BoardMask,
    total: BoardMask,
    scale: i16,
    res: &mut ArrayBoard<i16>,
) {
    for sq in Bits(p) {
        res.add(diag_rays(sq, total), scale * ChessPiece::BISHOP);
    }
}

pub fn count_bishop_attackers(
    p: BoardMask,
    total: BoardMask,
    amount: i8,
    res: &mut ArrayBoard<i8>,
) {
    for sq in Bits(p) {
        res.add(diag_rays(sq, total), amount);
    }
}

pub fn count_rook_attacker_materiel(
    p: BoardMask,
    total: BoardMask,
    scale: i16,
    res: &mut ArrayBoard<i16>,
) {
    for sq in Bits(p) {
        res.add(ortho_rays(sq, total), scale * ChessPiece::ROOK);
    }
}

pub fn count_rook_attackers(p: BoardMask, total: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Bits(p) {
        res.add(ortho_rays(sq, total), amount);
    }
}

pub fn count_queen_attacker_materiel(
    p: BoardMask,
    total: BoardMask,
    scale: i16,
    res: &mut ArrayBoard<i16>,
) {
    for sq in Bits(p) {
        res.add(
            ortho_rays(sq, total) | diag_rays(sq, total),
            scale * ChessPiece::QUEEN,
        );
    }
}

pub fn count_queen_attackers(p: BoardMask, total: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Bits(p) {
        res.add(ortho_rays(sq, total) | diag_rays(sq, total), amount);
    }
}

pub fn count_king_attackers(p: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Bits(p) {
        res.add(KING_MOVES.at(sq), amount);
    }
}
