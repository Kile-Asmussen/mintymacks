use crate::{
    bits::{
        Bits, Mask, bit, board::HalfBitBoard, jumps, slide_move_stop_negative,
        slide_move_stop_positive, slides, two_bits,
    },
    model::{Color, Piece, Square, moves::PseudoMove},
};

pub fn pawn_threats(p: Mask, c: Color) -> Mask {
    let jumps = match c {
        Color::White => &jumps::WHITE_PAWN_CAPTURE,
        Color::Black => &jumps::BLACK_PAWN_CAPTURE,
    };

    jumps.overlay(p)
}

pub fn knight_threats(n: Mask) -> Mask {
    jumps::KNIGHT_MOVES.overlay(n)
}

pub fn king_threats(k: Mask) -> Mask {
    jumps::KING_MOVES.overlay(k)
}

pub fn rook_threats(r: Mask, total: Mask) -> Mask {
    let mut res = Mask::MIN;
    for sq in Bits(r) {
        res |= slide_move_stop_positive(slides::RAYS_NORTH.at(sq), Mask::MIN, total);
        res |= slide_move_stop_positive(slides::RAYS_EAST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_WEST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_SOUTH.at(sq), Mask::MIN, total);
    }
    res
}

pub fn bishop_threats(r: Mask, total: Mask) -> Mask {
    let mut res = Mask::MIN;
    for sq in Bits(r) {
        res |= slide_move_stop_positive(slides::RAYS_NORTHEAST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_positive(slides::RAYS_NORTHWEST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_SOUTHEAST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_SOUTHWEST.at(sq), Mask::MIN, total);
    }
    res
}

pub fn queen_threats(r: Mask, total: Mask) -> Mask {
    let mut res = Mask::MIN;
    for sq in Bits(r) {
        res |= slide_move_stop_positive(slides::RAYS_NORTH.at(sq), Mask::MIN, total);
        res |= slide_move_stop_positive(slides::RAYS_EAST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_WEST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_SOUTH.at(sq), Mask::MIN, total);
        res |= slide_move_stop_positive(slides::RAYS_NORTHEAST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_positive(slides::RAYS_NORTHWEST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_SOUTHEAST.at(sq), Mask::MIN, total);
        res |= slide_move_stop_negative(slides::RAYS_SOUTHWEST.at(sq), Mask::MIN, total);
    }
    res
}

impl HalfBitBoard {
    pub fn threats(
        &self,
        c: Color,
        enemy: Mask,
        mv: Option<PseudoMove>,
        cap: Option<(Piece, Square)>,
    ) -> Mask {
        let enemy = enemy ^ two_bits(mv);
        let friendly = self.total() ^ bit(cap.map(|(_, s)| s));
        let total = friendly | enemy;

        return pawn_threats(self.pawns ^ is_cap(Piece::Pawn, cap), c)
            | knight_threats(self.knights ^ is_cap(Piece::Knight, cap))
            | king_threats(self.kings)
            | rook_threats(self.rooks ^ is_cap(Piece::Rook, cap), total)
            | bishop_threats(self.bishops ^ is_cap(Piece::Bishop, cap), total)
            | queen_threats(self.queens ^ is_cap(Piece::Queen, cap), total);

        #[inline]
        fn is_cap(is: Piece, cap: Option<(Piece, Square)>) -> u64 {
            match cap {
                Some((p, sq)) if p == is => sq.bit(),
                _ => Mask::MIN,
            }
        }
    }
}
