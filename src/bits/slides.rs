use crate::{
    arrays::ArrayBoard,
    bits::{BoardMask, bit, mask, show_mask},
    model::{Color, Dir, BoardRank, Square},
};

pub const RAYS_EAST: ArrayBoard<BoardMask> = build_slideboard(Dir::East);
pub const RAYS_NORTH: ArrayBoard<BoardMask> = build_slideboard(Dir::North);
pub const RAYS_WEST: ArrayBoard<BoardMask> = build_slideboard(Dir::West);
pub const RAYS_SOUTH: ArrayBoard<BoardMask> = build_slideboard(Dir::South);

pub const RAYS_NORTHEAST: ArrayBoard<BoardMask> =
    build_slideboard(Dir::NorthEast);
pub const RAYS_SOUTHEAST: ArrayBoard<BoardMask> =
    build_slideboard(Dir::SouthEast);

pub const RAYS_NORTHWEST: ArrayBoard<BoardMask> =
    build_slideboard(Dir::NorthWest);
pub const RAYS_SOUTHWEST: ArrayBoard<BoardMask> =
    build_slideboard(Dir::SouthWest);

pub const fn build_slideboard(dir: Dir) -> ArrayBoard<BoardMask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Some(Square::a1);

    while let Some(sq) = it {
        res.set(sq, build_slidemask(dir, sq));
        it = sq.next();
    }

    res
}

pub const fn build_slidemask(dir: Dir, sq: Square) -> BoardMask {
    let mut res = BoardMask::MIN;
    let mut n = 1;
    let mut sq = sq.go(&[dir]);
    while let Some(s) = sq
    {
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
        Color::White => Dir::North,
        Color::Black => Dir::South,
    };

    if sq.file_rank().1 as i8 == start_rank as i8 {
        bit(sq.go(&[dir])) | bit(sq.go(&[dir, dir]))
    } else {
        bit(sq.go(&[dir]))
    }
}

#[test]
fn slidemask_correct() {
    assert_eq!(
        RAYS_EAST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00001111,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        RAYS_NORTH.at(Square::d4),
        mask([
            0b_00010000,
            0b_00010000,
            0b_00010000,
            0b_00010000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        RAYS_WEST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_11100000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        RAYS_SOUTH.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00010000,
            0b_00010000,
            0b_00010000,
        ])
    );

    assert_eq!(
        RAYS_NORTHEAST.at(Square::d4),
        mask([
            0b_00000001,
            0b_00000010,
            0b_00000100,
            0b_00001000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        RAYS_NORTHWEST.at(Square::d4),
        mask([
            0b_00000000,
            0b_10000000,
            0b_01000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        RAYS_SOUTHEAST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00001000,
            0b_00000100,
            0b_00000010,
        ])
    );

    assert_eq!(
        RAYS_SOUTHWEST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_01000000,
            0b_10000000,
        ])
    );
}
