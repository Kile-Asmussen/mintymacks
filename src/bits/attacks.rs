use crate::{
    arrays::ArrayBoard,
    bits::{
        self, Bits, BoardMask, bit,
        board::HalfBitBoard,
        jumps::{self, BLACK_PAWN_CAPTURE, KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE},
        slide_move_attacks,
        slides::{
            self, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH,
            RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST,
        },
        two_bits,
    },
    model::{ChessPiece, Color, ColoredChessPiece, Square, moves::PseudoMove},
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
        cap: Option<(ChessPiece, Square)>,
    ) -> BoardMask {
        use ChessPiece::*;

        let enemy = enemy ^ mv.bits();
        let friendly = self.total ^ bit(cap.map(|(_, s)| s));
        let total = friendly | enemy;

        return pawn_attacks(self.pawns ^ is_cap(Pawn, cap), c)
            | knight_attacks(self.knights ^ is_cap(Knight, cap))
            | king_attacks(self.kings)
            | rook_attacks(self.rooks ^ is_cap(Rook, cap), total)
            | bishop_attacks(self.bishops ^ is_cap(Bishop, cap), total)
            | queen_attacks(self.queens ^ is_cap(Queen, cap), total);

        #[inline]
        fn is_cap(is: ChessPiece, cap: Option<(ChessPiece, Square)>) -> BoardMask {
            match cap {
                Some((p, sq)) if p == is => sq.bit(),
                _ => BoardMask::MIN,
            }
        }
    }

    pub fn attacks_after_friendly_move(
        &self,
        c: ColoredChessPiece,
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
        fn is_mv(c: ColoredChessPiece, is: ChessPiece, mv: PseudoMove) -> BoardMask {
            if c.piece() == is {
                mv.bits()
            } else {
                BoardMask::MIN
            }
        }
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
    slide_move_attacks(RAYS_SOUTH.at(sq), RAYS_NORTH.at(sq), total)
        | slide_move_attacks(RAYS_WEST.at(sq), RAYS_EAST.at(sq), total)
}

#[inline]
pub fn diag_rays(sq: Square, total: BoardMask) -> BoardMask {
    slide_move_attacks(RAYS_SOUTHWEST.at(sq), RAYS_NORTHEAST.at(sq), total)
        | slide_move_attacks(RAYS_SOUTHEAST.at(sq), RAYS_NORTHWEST.at(sq), total)
}

pub fn rook_attacks(r: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = BoardMask::MIN;
    for sq in Bits(r) {
        res |= ortho_rays(sq, total);
    }
    res
}

pub fn bishop_attacks(r: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = BoardMask::MIN;
    for sq in Bits(r) {
        res |= diag_rays(sq, total);
    }
    res
}

pub fn queen_attacks(r: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = BoardMask::MIN;
    for sq in Bits(r) {
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
