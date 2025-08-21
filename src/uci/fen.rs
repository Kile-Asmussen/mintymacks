use std::ops::Index;

use anyhow::{Error, anyhow};

use crate::{
    arrays::ArrayBoard,
    model::{Color, ColorPiece, File, Square, castling::CastlingRights},
    uci::longalg::parse_square,
};

pub fn parse_fen_halfmove_clock(hmc: &str) -> anyhow::Result<u16> {
    u16::from_str_radix(hmc, 10).map_err(|_| anyhow!("Invalid FEN: Malformed ply clock `{}'", hmc))
}

pub fn parse_fen_turn_counter(tc: &str) -> anyhow::Result<u16> {
    u16::from_str_radix(tc, 10).map_err(|_| anyhow!("Invalid FEN: Malformed turn counter `{}'", tc))
}

pub fn parse_fen_en_passant_square(eps: &str) -> anyhow::Result<Option<Square>> {
    if eps == "-" {
        return Ok(None);
    }

    if let Some(sq) = parse_square(eps) {
        return Ok(Some(sq));
    }

    return Err(anyhow!(
        "Invalid FEN: Malformed en-passant square `{}'",
        eps
    ));
}

pub fn parse_fen_castling_rights(cr: &str) -> anyhow::Result<CastlingRights> {
    if cr != "-" && !cr.chars().all(|c| "KQkq".contains(c)) || cr.len() > 4 {
        return Err(anyhow!(
            "Invalid FEN: Malformed castling rights string `{}'",
            cr
        ));
    }

    let mut res = CastlingRights::full();

    if !cr.contains('K') {
        res.move_west_rook(Color::White);
    }

    if !cr.contains('Q') {
        res.move_east_rook(Color::White);
    }

    if !cr.contains('k') {
        res.move_west_rook(Color::Black);
    }

    if !cr.contains('q') {
        res.move_east_rook(Color::Black);
    }

    Ok(res)
}

pub fn parse_fen_to_move(bw: &str) -> anyhow::Result<Color> {
    Ok(match bw {
        "w" => Color::White,
        "b" => Color::Black,
        _ => {
            return Err(Error::msg(format!(
                "Invalid FEN: unrecognized color to move {bw}"
            )));
        }
    })
}

pub fn parse_fen_board(board: &str) -> anyhow::Result<ArrayBoard<Option<ColorPiece>>> {
    for c in board.chars() {
        if !"PNBRQKpnbrqk12345678/".contains(c) {
            return Err(Error::msg(format!(
                "Invalid FEN: unrecognized character `{c}'"
            )));
        }
    }

    let splits = board.chars().filter(|c| c == &'/').count();
    if splits != 7 {
        return Err(Error::msg(format!("Invalid FEN: number of ranks is not 8")));
    }

    let mut ranks = Vec::with_capacity(8);
    board.split('/').collect_into(&mut ranks);

    let mut expanded_ranks = Vec::with_capacity(8);

    for rank in ranks {
        if rank.len() > 8 {
            return Err(Error::msg(format!(
                "Invalid FEN: rank not 8 squares `{rank}`"
            )));
        }

        let mut expanded_rank = String::with_capacity(8);

        for c in rank.chars() {
            match c {
                '1' => expanded_rank += "1",
                '2' => expanded_rank += "11",
                '3' => expanded_rank += "111",
                '4' => expanded_rank += "1111",
                '5' => expanded_rank += "11111",
                '6' => expanded_rank += "111111",
                '7' => expanded_rank += "1111111",
                '8' => expanded_rank += "11111111",
                c => expanded_rank.push(c),
            }
        }

        if expanded_rank.len() != 8 {
            return Err(Error::msg(format!(
                "Invalid FEN: rank not 8 squares `{rank}`"
            )));
        }

        expanded_ranks.push(expanded_rank);
    }

    if expanded_ranks.len() != 8 {
        panic!("expanded_ranks is the wrong size");
    }

    expanded_ranks.reverse();

    let chars = expanded_ranks.into_iter().collect::<String>();

    if chars.len() != 64 {
        panic!("more chars generated than expected");
    }

    let mut res = ArrayBoard::new(None);

    for (ix, c) in chars.char_indices() {
        res.set(Square::new(ix as i8).unwrap(), color_piece_letter(c))
    }

    Ok(res)
}

pub fn color_piece_letter(c: char) -> Option<ColorPiece> {
    use ColorPiece::*;
    Some(match c {
        'P' => WhitePawn,
        'N' => WhiteKnight,
        'B' => WhiteBishop,
        'R' => WhiteRook,
        'Q' => WhiteQueen,
        'K' => WhiteKing,
        'p' => BlackPawn,
        'n' => BlackKnight,
        'b' => BlackBishop,
        'r' => BlackRook,
        'q' => BlackQueen,
        'k' => BlackKing,
        _ => return None,
    })
}

pub fn render_fen_board(board: &ArrayBoard<Option<ColorPiece>>) -> String {
    let mut res = Vec::with_capacity(8);

    let mut it = Some(Square::a1);
    let mut n = 0;
    let mut s = String::with_capacity(8);

    while let Some(sq) = it {
        if let Some(c) = board.at(sq) {
            if n != 0 {
                s += &n.to_string();
                s.push(letter_color_piece(c));
                n = 0;
            }
        } else {
            n += 1;
        }

        if sq.file_rank().0 == File::H {
            if n != 0 {
                s += &n.to_string();
                n = 0;
            }
            res.push(s);
            s = String::with_capacity(8);
        }

        it = sq.next();
    }

    res.reverse();
    res.join("/")
}

pub fn letter_color_piece(c: ColorPiece) -> char {
    use ColorPiece::*;
    match c {
        WhitePawn => 'P',
        WhiteKnight => 'N',
        WhiteBishop => 'B',
        WhiteRook => 'R',
        WhiteQueen => 'Q',
        WhiteKing => 'K',
        BlackPawn => 'p',
        BlackKnight => 'n',
        BlackBishop => 'b',
        BlackRook => 'r',
        BlackQueen => 'q',
        BlackKing => 'k',
    }
}
