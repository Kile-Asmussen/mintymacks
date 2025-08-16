use crate::{
    bits::{Mask, bit, mask, show_mask},
    board::{Dir, Square, Squares},
    arrays::ArrayBoard,
};

pub const WEST: ArrayBoard<Mask> = build_slideboard(Dir::West, &Squares::WEST);
pub const EAST: ArrayBoard<Mask> = build_slideboard(Dir::East, &Squares::EAST);
pub const NORTH: ArrayBoard<Mask> = build_slideboard(Dir::North, &Squares::NORTH);
pub const SOUTH: ArrayBoard<Mask> = build_slideboard(Dir::South, &Squares::SOUTH);

pub const NORTHEAST: ArrayBoard<Mask> = build_slideboard(Dir::NorthEast, &Squares::NORTHEAST);
pub const NORTHWEST: ArrayBoard<Mask> = build_slideboard(Dir::NorthWest, &Squares::NORTHWEST);
pub const SOUTHEAST: ArrayBoard<Mask> = build_slideboard(Dir::SouthEast, &Squares::SOUTHEAST);
pub const SOUTHWEST: ArrayBoard<Mask> = build_slideboard(Dir::SouthWest, &Squares::SOUTHWEST);

pub const fn build_slideboard(dir: Dir, max: &ArrayBoard<i8>) -> ArrayBoard<Mask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Squares::all();

    while let Some(sq) = it.next() {
        res.set(sq, build_slidemask(dir, max.at(sq), sq));
    }

    res
}

pub const fn build_slidemask(dir: Dir, max: i8, sq: Square) -> Mask {
    let mut res = Mask::MIN;
    let mut n = 1;
    while let Some(s) = Square::new(sq.ix() + (dir as i8) * n)
        && n <= max
    {
        res |= bit(s);
        n += 1;
    }
    res
}

#[test]
fn slidemask_correct() {
    assert_eq!(
        EAST.at(Square::d4),
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
        NORTH.at(Square::d4),
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
        WEST.at(Square::d4),
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
        SOUTH.at(Square::d4),
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
        NORTHEAST.at(Square::d4),
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
        NORTHWEST.at(Square::d4),
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
        SOUTHEAST.at(Square::d4),
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
        SOUTHWEST.at(Square::d4),
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
