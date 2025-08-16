use crate::{
    arrays::ArrayBoard,
    bits::{Mask, bit, mask, show_mask},
    board::{Dir, Square},
};

pub const WEST: ArrayBoard<Mask> = build_slideboard(Dir::West, &Square::WEST);
pub const EAST: ArrayBoard<Mask> = build_slideboard(Dir::East, &Square::EAST);
pub const NORTH: ArrayBoard<Mask> = build_slideboard(Dir::North, &Square::NORTH);
pub const SOUTH: ArrayBoard<Mask> = build_slideboard(Dir::South, &Square::SOUTH);

pub const NORTHEAST: ArrayBoard<Mask> = build_slideboard(Dir::NorthEast, &Square::NORTHEAST);
pub const NORTHWEST: ArrayBoard<Mask> = build_slideboard(Dir::NorthWest, &Square::NORTHWEST);
pub const SOUTHEAST: ArrayBoard<Mask> = build_slideboard(Dir::SouthEast, &Square::SOUTHEAST);
pub const SOUTHWEST: ArrayBoard<Mask> = build_slideboard(Dir::SouthWest, &Square::SOUTHWEST);

pub const fn build_slideboard(dir: Dir, max: &ArrayBoard<i8>) -> ArrayBoard<Mask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Some(Square::a1);

    while let Some(sq) = it {
        res.set(sq, build_slidemask(dir, max.at(sq), sq));
        it = sq.next();
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
