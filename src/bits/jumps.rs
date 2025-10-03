use crate::{
    arrays::ArrayBoard,
    bits::{BoardMask, bit},
    model::{BoardRank, Color, Dir, Square},
};

pub const WHITE_PAWN_CAPTURE: ArrayBoard<BoardMask> = build_pawnboard(Color::White);

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
        Color::White => Dir::North,
        Color::Black => Dir::South,
    };

    bit(sq.go(&[dir, Dir::East])) | bit(sq.go(&[dir, Dir::West]))
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
    bit(sq.go(&[Dir::North, Dir::NorthEast]))
        | bit(sq.go(&[Dir::North, Dir::NorthWest]))
        | bit(sq.go(&[Dir::West, Dir::NorthWest]))
        | bit(sq.go(&[Dir::West, Dir::SouthWest]))
        | bit(sq.go(&[Dir::South, Dir::SouthEast]))
        | bit(sq.go(&[Dir::South, Dir::SouthWest]))
        | bit(sq.go(&[Dir::East, Dir::NorthEast]))
        | bit(sq.go(&[Dir::East, Dir::SouthEast]))
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
    bit(sq.go(&[Dir::North]))
        | bit(sq.go(&[Dir::NorthEast]))
        | bit(sq.go(&[Dir::East]))
        | bit(sq.go(&[Dir::SouthEast]))
        | bit(sq.go(&[Dir::South]))
        | bit(sq.go(&[Dir::SouthWest]))
        | bit(sq.go(&[Dir::West]))
        | bit(sq.go(&[Dir::NorthWest]))
}
