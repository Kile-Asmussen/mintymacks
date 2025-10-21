use crate::{
    arrays::ArrayBoard,
    bits::{BoardMask, one_bit},
    model::{BoardRank, Color, Direction, Square},
};

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
    let dir = match c {
        Color::White => Direction::North,
        Color::Black => Direction::South,
    };

    one_bit(sq.go(&[dir, Direction::East])) | one_bit(sq.go(&[dir, Direction::West]))
}

pub const KNIGHT_MOVES: ArrayBoard<BoardMask> = build_knightboard();

pub const fn build_knightboard() -> ArrayBoard<BoardMask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Some(Square::a1);

    while let Some(sq) = it {
        res.set(sq, build_knightmask(sq));
        it = sq.next();
    }

    res
}

pub const fn build_knightmask(sq: Square) -> BoardMask {
    one_bit(sq.go(&[Direction::North, Direction::NorthEast]))
        | one_bit(sq.go(&[Direction::North, Direction::NorthWest]))
        | one_bit(sq.go(&[Direction::West, Direction::NorthWest]))
        | one_bit(sq.go(&[Direction::West, Direction::SouthWest]))
        | one_bit(sq.go(&[Direction::South, Direction::SouthEast]))
        | one_bit(sq.go(&[Direction::South, Direction::SouthWest]))
        | one_bit(sq.go(&[Direction::East, Direction::NorthEast]))
        | one_bit(sq.go(&[Direction::East, Direction::SouthEast]))
}

pub const KING_MOVES: ArrayBoard<BoardMask> = build_kingboard();

pub const fn build_kingboard() -> ArrayBoard<BoardMask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Some(Square::a1);

    while let Some(sq) = it {
        res.set(sq, build_kingmask(sq));
        it = sq.next();
    }

    res
}

pub const fn build_kingmask(sq: Square) -> BoardMask {
    one_bit(sq.go(&[Direction::North]))
        | one_bit(sq.go(&[Direction::NorthEast]))
        | one_bit(sq.go(&[Direction::East]))
        | one_bit(sq.go(&[Direction::SouthEast]))
        | one_bit(sq.go(&[Direction::South]))
        | one_bit(sq.go(&[Direction::SouthWest]))
        | one_bit(sq.go(&[Direction::West]))
        | one_bit(sq.go(&[Direction::NorthWest]))
}
