use crate::{
    arrays::ArrayBoard,
    bits::{Bits, BoardMask},
    model::{Dir, File, Rank, Square},
};

#[allow(non_upper_case_globals)]
impl Square {
    pub const a1: Square = File::A.by(Rank::_1);
    pub const b1: Square = File::B.by(Rank::_1);
    pub const c1: Square = File::C.by(Rank::_1);
    pub const d1: Square = File::D.by(Rank::_1);
    pub const e1: Square = File::E.by(Rank::_1);
    pub const f1: Square = File::F.by(Rank::_1);
    pub const g1: Square = File::G.by(Rank::_1);
    pub const h1: Square = File::H.by(Rank::_1);

    pub const a2: Square = File::A.by(Rank::_2);
    pub const b2: Square = File::B.by(Rank::_2);
    pub const c2: Square = File::C.by(Rank::_2);
    pub const d2: Square = File::D.by(Rank::_2);
    pub const e2: Square = File::E.by(Rank::_2);
    pub const f2: Square = File::F.by(Rank::_2);
    pub const g2: Square = File::G.by(Rank::_2);
    pub const h2: Square = File::H.by(Rank::_2);

    pub const a3: Square = File::A.by(Rank::_3);
    pub const b3: Square = File::B.by(Rank::_3);
    pub const c3: Square = File::C.by(Rank::_3);
    pub const d3: Square = File::D.by(Rank::_3);
    pub const e3: Square = File::E.by(Rank::_3);
    pub const f3: Square = File::F.by(Rank::_3);
    pub const g3: Square = File::G.by(Rank::_3);
    pub const h3: Square = File::H.by(Rank::_3);

    pub const a4: Square = File::A.by(Rank::_4);
    pub const b4: Square = File::B.by(Rank::_4);
    pub const c4: Square = File::C.by(Rank::_4);
    pub const d4: Square = File::D.by(Rank::_4);
    pub const e4: Square = File::E.by(Rank::_4);
    pub const f4: Square = File::F.by(Rank::_4);
    pub const g4: Square = File::G.by(Rank::_4);
    pub const h4: Square = File::H.by(Rank::_4);

    pub const a5: Square = File::A.by(Rank::_5);
    pub const b5: Square = File::B.by(Rank::_5);
    pub const c5: Square = File::C.by(Rank::_5);
    pub const d5: Square = File::D.by(Rank::_5);
    pub const e5: Square = File::E.by(Rank::_5);
    pub const f5: Square = File::F.by(Rank::_5);
    pub const g5: Square = File::G.by(Rank::_5);
    pub const h5: Square = File::H.by(Rank::_5);

    pub const a6: Square = File::A.by(Rank::_6);
    pub const b6: Square = File::B.by(Rank::_6);
    pub const c6: Square = File::C.by(Rank::_6);
    pub const d6: Square = File::D.by(Rank::_6);
    pub const e6: Square = File::E.by(Rank::_6);
    pub const f6: Square = File::F.by(Rank::_6);
    pub const g6: Square = File::G.by(Rank::_6);
    pub const h6: Square = File::H.by(Rank::_6);

    pub const a7: Square = File::A.by(Rank::_7);
    pub const b7: Square = File::B.by(Rank::_7);
    pub const c7: Square = File::C.by(Rank::_7);
    pub const d7: Square = File::D.by(Rank::_7);
    pub const e7: Square = File::E.by(Rank::_7);
    pub const f7: Square = File::F.by(Rank::_7);
    pub const g7: Square = File::G.by(Rank::_7);
    pub const h7: Square = File::H.by(Rank::_7);

    pub const a8: Square = File::A.by(Rank::_8);
    pub const b8: Square = File::B.by(Rank::_8);
    pub const c8: Square = File::C.by(Rank::_8);
    pub const d8: Square = File::D.by(Rank::_8);
    pub const e8: Square = File::E.by(Rank::_8);
    pub const f8: Square = File::F.by(Rank::_8);
    pub const g8: Square = File::G.by(Rank::_8);
    pub const h8: Square = File::H.by(Rank::_8);
}

impl Square {
    pub const WEST: ArrayBoard<i8> = ArrayBoard::setup([[0, 1, 2, 3, 4, 5, 6, 7]; 8]);

    pub const EAST: ArrayBoard<i8> = ArrayBoard::setup([[7, 6, 5, 4, 3, 2, 1, 0]; 8]);

    pub const NORTH: ArrayBoard<i8> =
        ArrayBoard::setup([[0; 8], [1; 8], [2; 8], [3; 8], [4; 8], [5; 8], [6; 8], [7; 8]]);

    pub const SOUTH: ArrayBoard<i8> =
        ArrayBoard::setup([[7; 8], [6; 8], [5; 8], [4; 8], [3; 8], [2; 8], [1; 8], [0; 8]]);

    pub const NORTHEAST: ArrayBoard<i8> = ArrayBoard::setup([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 0],
        [2, 2, 2, 2, 2, 2, 1, 0],
        [3, 3, 3, 3, 3, 2, 1, 0],
        [4, 4, 4, 4, 3, 2, 1, 0],
        [5, 5, 5, 4, 3, 2, 1, 0],
        [6, 6, 5, 4, 3, 2, 1, 0],
        [7, 6, 5, 4, 3, 2, 1, 0],
    ]);

    pub const NORTHWEST: ArrayBoard<i8> = ArrayBoard::setup([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 2, 2, 2, 2, 2, 2],
        [0, 1, 2, 3, 3, 3, 3, 3],
        [0, 1, 2, 3, 4, 4, 4, 4],
        [0, 1, 2, 3, 4, 5, 5, 5],
        [0, 1, 2, 3, 4, 5, 6, 6],
        [0, 1, 2, 3, 4, 5, 6, 7],
    ]);

    pub const SOUTHEAST: ArrayBoard<i8> = ArrayBoard::setup([
        [7, 6, 5, 4, 3, 2, 1, 0],
        [6, 6, 5, 4, 3, 2, 1, 0],
        [5, 5, 5, 4, 3, 2, 1, 0],
        [4, 4, 4, 4, 3, 2, 1, 0],
        [3, 3, 3, 3, 3, 2, 1, 0],
        [2, 2, 2, 2, 2, 2, 1, 0],
        [1, 1, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ]);

    pub const SOUTHWEST: ArrayBoard<i8> = ArrayBoard::setup([
        [0, 1, 2, 3, 4, 5, 6, 7],
        [0, 1, 2, 3, 4, 5, 6, 6],
        [0, 1, 2, 3, 4, 5, 5, 5],
        [0, 1, 2, 3, 4, 4, 4, 4],
        [0, 1, 2, 3, 3, 3, 3, 3],
        [0, 1, 2, 2, 2, 2, 2, 2],
        [0, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ]);

    pub const fn go(self, mut dirs: &[Dir]) -> Option<Self> {
        if dirs.is_empty() {
            Some(self)
        } else {
            let dir = dirs[0];
            let dirs = &dirs[1..];

            let n = (match dir {
                Dir::North => Square::NORTH,
                Dir::East => Square::EAST,
                Dir::South => Square::SOUTH,
                Dir::West => Square::WEST,
                Dir::NorthEast => Square::NORTHEAST,
                Dir::SouthEast => Square::SOUTHEAST,
                Dir::SouthWest => Square::SOUTHWEST,
                Dir::NorthWest => Square::NORTHWEST,
            })
            .at(self);

            if n != 0
                && let Some(sq) = Square::new(self.ix() + dir as i8)
            {
                sq.go(dirs)
            } else {
                None
            }
        }
    }
}

#[test]
fn square_names() {
    assert_eq!(Square::a1, Square::new(0).unwrap());
    assert_eq!(Square::b1, Square::new(1).unwrap());
    assert_eq!(Square::c1, Square::new(2).unwrap());

    assert_eq!(Square::a2, Square::new(8).unwrap());
    assert_eq!(Square::a3, Square::new(16).unwrap());

    assert_eq!(Square::h8, Square::new(63).unwrap());

    assert_eq!(Square::a1.file_rank(), (File::A, Rank::_1));
    assert_eq!(Square::h1.file_rank(), (File::H, Rank::_1));
    assert_eq!(Square::h8.file_rank(), (File::H, Rank::_8));
}

#[test]
fn square_go_correct() {
    assert_eq!(Square::a1.go(&[Dir::East]), Some(Square::b1));
    assert_eq!(Square::a1.go(&[Dir::North]), Some(Square::a2));
    assert_eq!(Square::a1.go(&[Dir::NorthEast]), Some(Square::b2));

    assert_eq!(Square::a1.go(&[Dir::South]), None);
    assert_eq!(Square::a1.go(&[Dir::West]), None);
    assert_eq!(Square::a1.go(&[Dir::SouthWest]), None);
    assert_eq!(Square::a1.go(&[Dir::SouthEast]), None);
    assert_eq!(Square::a1.go(&[Dir::NorthWest]), None);

    assert_eq!(
        Square::c1.go(&[Dir::West, Dir::NorthWest]),
        Some(Square::a2)
    );

    assert_eq!(
        Square::a1.go(&[Dir::North, Dir::East]),
        Square::a1.go(&[Dir::NorthEast])
    )
}
