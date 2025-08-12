use std::num::NonZeroI8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Square(NonZeroI8);

impl Square {
    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=63 => {
                if let Some(nzi8) = NonZeroI8::new(ix + 1) {
                    return Some(Self(nzi8));
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    pub const fn ix(self) -> i8 {
        self.0.get() - 1
    }

    pub const fn at(f: File, r: Rank) -> Self {
        Self::new(f as i8 + r as i8).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl File {
    pub const fn x(self, r: Rank) -> Square {
        Square::at(self, r)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Rank {
    _1 = 0,
    _2 = 8,
    _3 = 16,
    _4 = 24,
    _5 = 32,
    _6 = 40,
    _7 = 48,
    _8 = 56,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Color {
    White = 1,
    Black = -1,
}

impl Color {
    pub const fn piece(self, p: Piece) -> ColorPiece {
        ColorPiece::new(self, p)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Piece {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl Piece {
    pub const fn color(self, c: Color) -> ColorPiece {
        ColorPiece::new(c, self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum ColorPiece {
    WhitePawn = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteRook = 4,
    WhiteQueen = 5,
    WhiteKing = 6,
    BlackPawn = -1,
    BlackKnight = -2,
    BlackBishop = -3,
    BlackRook = -4,
    BlackQueen = -5,
    BlackKing = -6,
}

impl ColorPiece {
    pub const fn new(c: Color, p: Piece) -> Self {
        use Color::*;
        use ColorPiece::*;
        use Piece::*;
        match (c, p) {
            (White, Pawn) => WhitePawn,
            (White, Knight) => WhiteKnight,
            (White, Bishop) => WhiteBishop,
            (White, Rook) => WhiteRook,
            (White, Queen) => WhiteQueen,
            (White, King) => WhiteKing,
            (Black, Pawn) => BlackPawn,
            (Black, Knight) => BlackKnight,
            (Black, Bishop) => BlackBishop,
            (Black, Rook) => BlackRook,
            (Black, Queen) => BlackQueen,
            (Black, King) => BlackKing,
        }
    }
}

#[cfg(test)]
fn null_optimization<T>() {
    assert_eq!(std::mem::size_of::<Option<T>>(), std::mem::size_of::<T>());
}

#[test]
fn square_nullopt() {
    null_optimization::<Square>();
    null_optimization::<Color>();
    null_optimization::<Piece>();
    null_optimization::<ColorPiece>();
    null_optimization::<Rank>();
    null_optimization::<File>();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum CastlingMove {
    OOO = 1,
    OO = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum CastlingRights {
    NoRights = 0,
    QueenSide = 1,
    KingSide = 2,
    Both = 3,
}

impl CastlingRights {}
