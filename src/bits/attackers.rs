use crate::{
    arrays::ArrayBoard,
    bits::{
        self, BoardMask, Squares,
        attacks::{bishop_attacks, queen_attacks, rook_attacks},
        board::HalfBitBoard,
        jumps::{self, BLACK_PAWN_CAPTURE, KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE},
        one_bit,
        slides::{
            self, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH,
            RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST, simple_diagonal_attack,
            simple_diagonal_attacks, simple_omnidirectional_attack, simple_omnidirectional_attacks,
            simple_orthogonal_attack, simple_orthogonal_attacks,
        },
        two_bits,
    },
    model::{
        BoardFile, ChessPiece, Color, ColoredChessPiece, ColoredChessPieceWithCapture, Square,
        moves::PseudoMove,
    },
};

impl HalfBitBoard {
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

pub fn count_pawn_attacker_materiel(p: BoardMask, c: Color, scale: i16, res: &mut ArrayBoard<i16>) {
    for sq in Squares(p) {
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
    for sq in Squares(p) {
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
    for sq in Squares(p) {
        res.add(KNIGHT_MOVES.at(sq), scale * ChessPiece::KNIGHT);
    }
}

pub fn count_knight_attackers(p: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Squares(p) {
        res.add(KNIGHT_MOVES.at(sq), amount);
    }
}

pub fn count_bishop_attacker_materiel(
    p: BoardMask,
    total: BoardMask,
    scale: i16,
    res: &mut ArrayBoard<i16>,
) {
    for sq in Squares(p) {
        res.add(
            simple_diagonal_attack(sq, total),
            scale * ChessPiece::BISHOP,
        );
    }
}

pub fn count_bishop_attackers(
    p: BoardMask,
    total: BoardMask,
    amount: i8,
    res: &mut ArrayBoard<i8>,
) {
    for sq in Squares(p) {
        res.add(simple_diagonal_attack(sq, total), amount);
    }
}

pub fn count_rook_attacker_materiel(
    p: BoardMask,
    total: BoardMask,
    scale: i16,
    res: &mut ArrayBoard<i16>,
) {
    for sq in Squares(p) {
        res.add(
            simple_orthogonal_attack(sq, total),
            scale * ChessPiece::ROOK,
        );
    }
}

pub fn count_rook_attackers(p: BoardMask, total: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Squares(p) {
        res.add(simple_orthogonal_attack(sq, total), amount);
    }
}

pub fn count_queen_attacker_materiel(
    p: BoardMask,
    total: BoardMask,
    scale: i16,
    res: &mut ArrayBoard<i16>,
) {
    for sq in Squares(p) {
        res.add(
            simple_omnidirectional_attack(sq, total),
            scale * ChessPiece::QUEEN,
        );
    }
}

pub fn count_queen_attackers(p: BoardMask, total: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Squares(p) {
        res.add(simple_omnidirectional_attack(sq, total), amount);
    }
}

pub fn count_king_attackers(p: BoardMask, amount: i8, res: &mut ArrayBoard<i8>) {
    for sq in Squares(p) {
        res.add(KING_MOVES.at(sq), amount);
    }
}
