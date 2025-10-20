use crate::{
    arrays::ArrayBoard,
    bits::{BoardMask, Squares, mask, one_bit, show_mask},
    model::{BoardRank, Color, Direction, Square},
};

#[inline]
pub const fn obstruction_difference(
    neg_ray: BoardMask,
    pos_ray: BoardMask,
    occupied: BoardMask,
) -> BoardMask {
    let neg_hit = neg_ray & occupied;
    let pos_hit = pos_ray & occupied;
    let ms1b = 1u64 << (63 - (neg_hit & occupied | 1).leading_zeros());
    // let ms1b = 1u64 << neg_hit.checked_ilog2().unwrap_or(0);
    let diff = pos_hit ^ pos_hit.wrapping_sub(ms1b);
    return (neg_ray | pos_ray) & diff;
}

pub const RAYS_EAST: ArrayBoard<BoardMask> = build_slideboard(Direction::East);
pub const RAYS_NORTH: ArrayBoard<BoardMask> = build_slideboard(Direction::North);
pub const RAYS_WEST: ArrayBoard<BoardMask> = build_slideboard(Direction::West);
pub const RAYS_SOUTH: ArrayBoard<BoardMask> = build_slideboard(Direction::South);

pub const RAYS_NORTHEAST: ArrayBoard<BoardMask> = build_slideboard(Direction::NorthEast);
pub const RAYS_SOUTHEAST: ArrayBoard<BoardMask> = build_slideboard(Direction::SouthEast);

pub const RAYS_NORTHWEST: ArrayBoard<BoardMask> = build_slideboard(Direction::NorthWest);
pub const RAYS_SOUTHWEST: ArrayBoard<BoardMask> = build_slideboard(Direction::SouthWest);

pub const fn build_slideboard(dir: Direction) -> ArrayBoard<BoardMask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Some(Square::a1);

    while let Some(sq) = it {
        res.set(sq, build_slidemask(dir, sq));
        it = sq.next();
    }

    res
}

pub const fn build_slidemask(dir: Direction, sq: Square) -> BoardMask {
    let mut res = BoardMask::MIN;
    let mut n = 1;
    let mut sq = sq.go(&[dir]);
    while let Some(s) = sq {
        res |= s.bit();
        n += 1;
        sq = s.go(&[dir])
    }
    res
}

pub const WHITE_PAWN_MOVES: ArrayBoard<BoardMask> = build_pawnboard(Color::White);
pub const BLACK_PAWN_MOVES: ArrayBoard<BoardMask> = build_pawnboard(Color::Black);

pub const fn build_pawnboard(c: Color) -> ArrayBoard<BoardMask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Some(Square::a1);

    while let Some(sq) = it {
        res.set(sq, build_pawnmask(c, sq));
        it = sq.next();
    }

    res
}

pub const fn build_pawnmask(c: Color, sq: Square) -> BoardMask {
    let start_rank = match c {
        Color::White => BoardRank::_2,
        Color::Black => BoardRank::_7,
    };

    let dir = match c {
        Color::White => Direction::North,
        Color::Black => Direction::South,
    };

    if sq.file_rank().1 as i8 == start_rank as i8 {
        one_bit(sq.go(&[dir])) | one_bit(sq.go(&[dir, dir]))
    } else {
        one_bit(sq.go(&[dir]))
    }
}

#[inline]
pub fn simple_orthogonal_attack(sq: Square, total: BoardMask) -> BoardMask {
    obstruction_difference(RAYS_SOUTH.at(sq), RAYS_NORTH.at(sq), total)
        | obstruction_difference(RAYS_WEST.at(sq), RAYS_EAST.at(sq), total)
}

#[inline]
pub fn simple_orthogonal_attacks(r: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = 0;
    for sq in Squares(r) {
        res |= simple_orthogonal_attack(sq, total)
    }
    res
}

#[inline]
pub fn simple_omnidirectional_attack(sq: Square, total: BoardMask) -> BoardMask {
    obstruction_difference(RAYS_SOUTHWEST.at(sq), RAYS_NORTHEAST.at(sq), total)
        | obstruction_difference(RAYS_SOUTHEAST.at(sq), RAYS_NORTHWEST.at(sq), total)
        | obstruction_difference(RAYS_SOUTH.at(sq), RAYS_NORTH.at(sq), total)
        | obstruction_difference(RAYS_WEST.at(sq), RAYS_EAST.at(sq), total)
}

#[inline]
pub fn simple_omnidirectional_attacks(q: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = 0;
    for sq in Squares(q) {
        res |= simple_omnidirectional_attack(sq, total)
    }
    res
}

#[inline]
pub fn simple_diagonal_attack(sq: Square, total: BoardMask) -> BoardMask {
    obstruction_difference(RAYS_SOUTHWEST.at(sq), RAYS_NORTHEAST.at(sq), total)
        | obstruction_difference(RAYS_SOUTHEAST.at(sq), RAYS_NORTHWEST.at(sq), total)
}

#[inline]
pub fn simple_diagonal_attacks(b: BoardMask, total: BoardMask) -> BoardMask {
    let mut res = 0;
    for sq in Squares(b) {
        res |= simple_diagonal_attack(sq, total);
    }
    res
}
