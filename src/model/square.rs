use crate::{
    arrays::ArrayBoard,
    bits::{BoardMask, Squares, mask},
    model::{BoardFile, BoardRank, Direction, Square},
};

#[allow(non_upper_case_globals)]
impl Square {
    pub const a1: Square = BoardFile::A.by(BoardRank::_1);
    pub const b1: Square = BoardFile::B.by(BoardRank::_1);
    pub const c1: Square = BoardFile::C.by(BoardRank::_1);
    pub const d1: Square = BoardFile::D.by(BoardRank::_1);
    pub const e1: Square = BoardFile::E.by(BoardRank::_1);
    pub const f1: Square = BoardFile::F.by(BoardRank::_1);
    pub const g1: Square = BoardFile::G.by(BoardRank::_1);
    pub const h1: Square = BoardFile::H.by(BoardRank::_1);

    pub const a2: Square = BoardFile::A.by(BoardRank::_2);
    pub const b2: Square = BoardFile::B.by(BoardRank::_2);
    pub const c2: Square = BoardFile::C.by(BoardRank::_2);
    pub const d2: Square = BoardFile::D.by(BoardRank::_2);
    pub const e2: Square = BoardFile::E.by(BoardRank::_2);
    pub const f2: Square = BoardFile::F.by(BoardRank::_2);
    pub const g2: Square = BoardFile::G.by(BoardRank::_2);
    pub const h2: Square = BoardFile::H.by(BoardRank::_2);

    pub const a3: Square = BoardFile::A.by(BoardRank::_3);
    pub const b3: Square = BoardFile::B.by(BoardRank::_3);
    pub const c3: Square = BoardFile::C.by(BoardRank::_3);
    pub const d3: Square = BoardFile::D.by(BoardRank::_3);
    pub const e3: Square = BoardFile::E.by(BoardRank::_3);
    pub const f3: Square = BoardFile::F.by(BoardRank::_3);
    pub const g3: Square = BoardFile::G.by(BoardRank::_3);
    pub const h3: Square = BoardFile::H.by(BoardRank::_3);

    pub const a4: Square = BoardFile::A.by(BoardRank::_4);
    pub const b4: Square = BoardFile::B.by(BoardRank::_4);
    pub const c4: Square = BoardFile::C.by(BoardRank::_4);
    pub const d4: Square = BoardFile::D.by(BoardRank::_4);
    pub const e4: Square = BoardFile::E.by(BoardRank::_4);
    pub const f4: Square = BoardFile::F.by(BoardRank::_4);
    pub const g4: Square = BoardFile::G.by(BoardRank::_4);
    pub const h4: Square = BoardFile::H.by(BoardRank::_4);

    pub const a5: Square = BoardFile::A.by(BoardRank::_5);
    pub const b5: Square = BoardFile::B.by(BoardRank::_5);
    pub const c5: Square = BoardFile::C.by(BoardRank::_5);
    pub const d5: Square = BoardFile::D.by(BoardRank::_5);
    pub const e5: Square = BoardFile::E.by(BoardRank::_5);
    pub const f5: Square = BoardFile::F.by(BoardRank::_5);
    pub const g5: Square = BoardFile::G.by(BoardRank::_5);
    pub const h5: Square = BoardFile::H.by(BoardRank::_5);

    pub const a6: Square = BoardFile::A.by(BoardRank::_6);
    pub const b6: Square = BoardFile::B.by(BoardRank::_6);
    pub const c6: Square = BoardFile::C.by(BoardRank::_6);
    pub const d6: Square = BoardFile::D.by(BoardRank::_6);
    pub const e6: Square = BoardFile::E.by(BoardRank::_6);
    pub const f6: Square = BoardFile::F.by(BoardRank::_6);
    pub const g6: Square = BoardFile::G.by(BoardRank::_6);
    pub const h6: Square = BoardFile::H.by(BoardRank::_6);

    pub const a7: Square = BoardFile::A.by(BoardRank::_7);
    pub const b7: Square = BoardFile::B.by(BoardRank::_7);
    pub const c7: Square = BoardFile::C.by(BoardRank::_7);
    pub const d7: Square = BoardFile::D.by(BoardRank::_7);
    pub const e7: Square = BoardFile::E.by(BoardRank::_7);
    pub const f7: Square = BoardFile::F.by(BoardRank::_7);
    pub const g7: Square = BoardFile::G.by(BoardRank::_7);
    pub const h7: Square = BoardFile::H.by(BoardRank::_7);

    pub const a8: Square = BoardFile::A.by(BoardRank::_8);
    pub const b8: Square = BoardFile::B.by(BoardRank::_8);
    pub const c8: Square = BoardFile::C.by(BoardRank::_8);
    pub const d8: Square = BoardFile::D.by(BoardRank::_8);
    pub const e8: Square = BoardFile::E.by(BoardRank::_8);
    pub const f8: Square = BoardFile::F.by(BoardRank::_8);
    pub const g8: Square = BoardFile::G.by(BoardRank::_8);
    pub const h8: Square = BoardFile::H.by(BoardRank::_8);
}

#[test]
fn afgaerg() {
    assert_eq!(Square::EAST_EDGE, 0x7F7F_7F7F_7F7F_7F7F);
    assert_eq!(Square::WEST_EDGE, 0xFEFE_FEFE_FEFE_FEFE);
}

impl Square {
    pub const EAST_EDGE: u64 = mask([0b11111110; 8]);

    pub const NORTH_EDGE: u64 = mask([0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    pub const SOUTH_EDGE: u64 = mask([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0]);

    pub const WEST_EDGE: u64 = mask([0b01111111; 8]);

    pub const fn go(self, mut dirs: &[Direction]) -> Option<Self> {
        if dirs.is_empty() {
            Some(self)
        } else {
            let dir = dirs[0];
            let dirs = &dirs[1..];

            let edge = match dir {
                Direction::North => Square::NORTH_EDGE,
                Direction::East => Square::EAST_EDGE,
                Direction::South => Square::SOUTH_EDGE,
                Direction::West => Square::WEST_EDGE,
                Direction::NorthEast => Square::NORTH_EDGE & Square::EAST_EDGE,
                Direction::SouthEast => Square::SOUTH_EDGE & Square::EAST_EDGE,
                Direction::SouthWest => Square::SOUTH_EDGE & Square::WEST_EDGE,
                Direction::NorthWest => Square::NORTH_EDGE & Square::WEST_EDGE,
            };

            let n = edge & self.bit();

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

    assert_eq!(Square::a1.file_rank(), (BoardFile::A, BoardRank::_1));
    assert_eq!(Square::h1.file_rank(), (BoardFile::H, BoardRank::_1));
    assert_eq!(Square::h8.file_rank(), (BoardFile::H, BoardRank::_8));
}

#[test]
fn square_go_correct() {
    assert_eq!(Square::a1.go(&[Direction::East]), Some(Square::b1));
    assert_eq!(Square::a1.go(&[Direction::North]), Some(Square::a2));
    assert_eq!(Square::a1.go(&[Direction::NorthEast]), Some(Square::b2));

    assert_eq!(Square::a1.go(&[Direction::South]), None);
    assert_eq!(Square::a1.go(&[Direction::West]), None);
    assert_eq!(Square::a1.go(&[Direction::SouthWest]), None);
    assert_eq!(Square::a1.go(&[Direction::SouthEast]), None);
    assert_eq!(Square::a1.go(&[Direction::NorthWest]), None);

    assert_eq!(
        Square::c1.go(&[Direction::West, Direction::NorthWest]),
        Some(Square::a2)
    );

    assert_eq!(
        Square::a1.go(&[Direction::North, Direction::East]),
        Square::a1.go(&[Direction::NorthEast])
    )
}
