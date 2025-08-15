mod square;
mod tests;

use std::num::NonZeroI8;

use crate::byteboard::ArrayBoard;

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

    pub const fn file_rank(self) -> (File, Rank) {
        (
            File::new(self.ix() & 0x7).unwrap(),
            Rank::new(self.ix() >> 3).unwrap(),
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Squares;

impl Squares {
    pub const fn all() -> AllSquares {
        AllSquares(unsafe { NonZeroI8::new_unchecked(1) })
    }
}

impl IntoIterator for Squares {
    type Item = Square;

    type IntoIter = AllSquares;

    fn into_iter(self) -> Self::IntoIter {
        Self::all()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct AllSquares(NonZeroI8);

impl AllSquares {
    pub const fn next(&mut self) -> Option<Square> {
        if self.0.get() < 65 {
            let res = Some(Square(self.0));
            self.0 = unsafe { NonZeroI8::new_unchecked(self.0.get() + 1) };
            res
        } else {
            None
        }
    }
}

impl Iterator for AllSquares {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (65 - self.0.get()) as usize;
        (n, Some(n))
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
    pub const fn by(self, r: Rank) -> Square {
        Square::at(self, r)
    }

    pub const fn ix(self) -> i8 {
        self as i8
    }

    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=7 => Some(unsafe { std::mem::transmute(ix) }),
            _ => None,
        }
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

impl Rank {
    pub const fn ix(self) -> i8 {
        self as i8
    }

    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=7 => Some(unsafe { std::mem::transmute(ix * 8) }),
            _ => None,
        }
    }
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

impl CastlingRights {
    pub fn can(self, c: CastlingMove) -> bool {
        (self as i8) & (c as i8) != 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Dir {
    North = 8,
    East = 1,
    South = -8,
    West = -1,
    NorthEast = 9,
    SouthEast = -7,
    SouthWest = -9,
    NorthWest = 7,
}
