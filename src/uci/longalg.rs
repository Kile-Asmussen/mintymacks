use std::str::Chars;

use crate::model::{File, Piece, Rank, Square, moves::PseudoMove};

pub fn parse_long_alg(s: &str) -> Option<(PseudoMove, Option<Piece>)> {
    let mut it = s.chars();

    let res = PseudoMove {
        from: parse_square_raw(&mut it)?,
        to: parse_square_raw(&mut it)?,
    };

    if let Some(p) = it.next() {
        if let None = it.next() {
            Some((res, Some(piece_letter(p)?)))
        } else {
            None
        }
    } else {
        Some((res, None))
    }
}

pub fn parse_square(s: &str) -> Option<Square> {
    let mut it = s.chars();

    let res = parse_square_raw(&mut it)?;

    if let None = it.next() {
        Some(res)
    } else {
        None
    }
}

fn parse_square_raw(it: &mut Chars) -> Option<Square> {
    let file = file_letter(it.next()?)?;
    let rank = rank_letter(it.next()?)?;
    Some(file.by(rank))
}

pub fn piece_letter(c: char) -> Option<Piece> {
    Some(match c {
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'r' => Piece::Rook,
        'q' => Piece::Queen,
        _ => return None,
    })
}

pub fn rank_letter(c: char) -> Option<Rank> {
    Some(match c {
        '1' => Rank::_1,
        '2' => Rank::_2,
        '3' => Rank::_3,
        '4' => Rank::_4,
        '5' => Rank::_5,
        '6' => Rank::_6,
        '7' => Rank::_7,
        '8' => Rank::_8,
        _ => return None,
    })
}

pub fn file_letter(c: char) -> Option<File> {
    Some(match c {
        'a' => File::A,
        'b' => File::B,
        'c' => File::C,
        'd' => File::D,
        'e' => File::E,
        'f' => File::F,
        'g' => File::G,
        'h' => File::H,
        _ => return None,
    })
}
