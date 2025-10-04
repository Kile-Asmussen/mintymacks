use crate::{
    bits::{
        bit, board::HalfBitBoard, jumps::{self, KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE}, slide_move_attacks, slides::{self, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH, RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST}, two_bits, Bits, BoardMask
    },
    model::{moves::PseudoMove, ChessPiece, Color, Square},
};

pub fn pawn_threats(p: BoardMask, c: Color) -> BoardMask {
    match c {
        Color::White => WHITE_PAWN_CAPTURE.overlay(p),
        Color::Black => WHITE_PAWN_CAPTURE.overlay(p.swap_bytes()).swap_bytes(),
    }
}

pub fn knight_threats(n: BoardMask) -> BoardMask {
    KNIGHT_MOVES.overlay(n)
}

pub fn king_threats(k: BoardMask) -> BoardMask {
    KING_MOVES.overlay(k)
}

pub fn rook_threats(r: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = BoardMask::MIN;
    for sq in Bits(r) {
        res |= slide_move_attacks(RAYS_SOUTH.at(sq), RAYS_NORTH.at(sq), total);
        res |= slide_move_attacks(RAYS_WEST.at(sq), RAYS_EAST.at(sq), total);

        // res |= slide_move_stop_positive(slides::RAYS_NORTH.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_positive(slides::RAYS_EAST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_WEST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_SOUTH.at(sq), BoardMask::MIN, total);
    }
    res
}

pub fn bishop_threats(r: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = BoardMask::MIN;
    for sq in Bits(r) {
        res |= slide_move_attacks(RAYS_SOUTHWEST.at(sq), RAYS_NORTHEAST.at(sq), total);
        res |= slide_move_attacks(RAYS_SOUTHEAST.at(sq), RAYS_NORTHWEST.at(sq), total);

        // res |= slide_move_stop_positive(slides::RAYS_NORTHEAST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_positive(slides::RAYS_NORTHWEST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_SOUTHEAST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_SOUTHWEST.at(sq), BoardMask::MIN, total);
    }
    res
}

pub fn queen_threats(r: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = BoardMask::MIN;
    for sq in Bits(r) {
        res |= slide_move_attacks(RAYS_SOUTH.at(sq), RAYS_NORTH.at(sq), total);
        res |= slide_move_attacks(RAYS_WEST.at(sq), RAYS_EAST.at(sq), total);
        res |= slide_move_attacks(RAYS_SOUTHWEST.at(sq), RAYS_NORTHEAST.at(sq), total);
        res |= slide_move_attacks(RAYS_SOUTHEAST.at(sq), RAYS_NORTHWEST.at(sq), total);

        // res |= slide_move_stop_positive(slides::RAYS_NORTH.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_positive(slides::RAYS_EAST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_WEST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_SOUTH.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_positive(slides::RAYS_NORTHEAST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_positive(slides::RAYS_NORTHWEST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_SOUTHEAST.at(sq), BoardMask::MIN, total);
        // res |= slide_move_stop_negative(slides::RAYS_SOUTHWEST.at(sq), BoardMask::MIN, total);
    }
    res
}

impl HalfBitBoard {
    pub fn threats(
        &self,
        c: Color,
        enemy: BoardMask,
        mv: Option<PseudoMove>,
        cap: Option<(ChessPiece, Square)>,
    ) -> BoardMask {
        let enemy = enemy ^ two_bits(mv);
        let friendly = ({
            let this = &self;
            this.total
        }) ^ bit(cap.map(|(_, s)| s));
        let total = friendly | enemy;

        return pawn_threats(self.pawns ^ is_cap(ChessPiece::Pawn, cap), c)
            | knight_threats(self.knights ^ is_cap(ChessPiece::Knight, cap))
            | king_threats(self.kings)
            | rook_threats(self.rooks ^ is_cap(ChessPiece::Rook, cap), total)
            | bishop_threats(self.bishops ^ is_cap(ChessPiece::Bishop, cap), total)
            | queen_threats(self.queens ^ is_cap(ChessPiece::Queen, cap), total);

        #[inline]
        fn is_cap(is: ChessPiece, cap: Option<(ChessPiece, Square)>) -> u64 {
            match cap {
                Some((p, sq)) if p == is => sq.bit(),
                _ => BoardMask::MIN,
            }
        }
    }
}
