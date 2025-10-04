use std::ops::Index;

use crate::{
    arrays::ArrayBoard,
    bits::board::BitBoard,
    model::{
        castling::{CastlingDetails, CastlingRights, CLASSIC_CASTLING}, BoardFile, Color, ColoredChessPiece, Square
    }, regexp,
};

type Result<T> = std::result::Result<T, String>;

pub fn parse_fen(fen: &str) -> Result<(BitBoard, u16)> {
    let parts = regexp!(r"\s+").split(fen).collect::<Vec<&str>>();

    let n_parts = parts.len();
    if n_parts > 6 {
        return Err(format!("Invalid FEN: Too many components ({n_parts})"));
    } else if n_parts < 6 {
        return Err(format!("Invalid FEN: Too few components ({n_parts})"));
    }

    if let ([x], _) = &parts[..].as_chunks() {
        parse_fen_raw(&x)
    } else {
        Err(format!("Invalid FEN: this error shouldn't happen"))
    }
}

pub fn parse_fen_raw<S: AsRef<str>>(parts: &[S; 6]) -> Result<(BitBoard, u16)> {

    let board = parse_fen_board(parts[0].as_ref())?;
    let to_move = parse_fen_to_move(parts[1].as_ref())?;
    let castling_rights = parse_fen_castling_rights(parts[2].as_ref())?;
    let en_passant = parse_fen_en_passant_square(parts[3].as_ref())?;
    let halfmove = parse_fen_halfmove_clock(parts[4].as_ref())?;
    let turn = parse_fen_turn_counter(parts[5].as_ref())?;

    Ok((BitBoard::new(
        &board,
        to_move,
        turn,
        castling_rights,
        en_passant,
        CLASSIC_CASTLING,
    ), halfmove))
}

pub fn parse_fen_halfmove_clock(hmc: &str) -> Result<u16> {
    u16::from_str_radix(hmc, 10).map_err(|_| format!("Invalid FEN: Malformed halfmove clock `{hmc}'"))
}

pub fn parse_fen_turn_counter(tc: &str) -> Result<u16> {
    u16::from_str_radix(tc, 10).map_err(|_| format!("Invalid FEN: Malformed turn counter `{tc}'"))
}

pub fn parse_fen_en_passant_square(eps: &str) -> Result<Option<Square>> {
    if eps == "-" {
        return Ok(None);
    }

    if let Some(sq) = Square::parse(eps) {
        return Ok(Some(sq));
    }

    return Err(format!("Invalid FEN: Malformed en-passant square `{eps}'"));
}

pub fn parse_fen_castling_rights(cr: &str) -> Result<CastlingRights> {
    if cr != "-" && !cr.chars().all(|c| "KQkq".contains(c)) || cr.len() > 4 {
        return Err(format!(
            "Invalid FEN: Malformed castling rights string `{cr}'"
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

pub fn parse_fen_to_move(bw: &str) -> Result<Color> {
    Ok(match bw {
        "w" => Color::White,
        "b" => Color::Black,
        _ => {
            return Err(format!(
                "Invalid FEN: unrecognized color to move {bw}"
            ));
        }
    })
}

pub fn parse_fen_board(board: &str) -> Result<ArrayBoard<Option<ColoredChessPiece>>> {
    for c in board.chars() {
        if !"PNBRQKpnbrqk12345678/".contains(c) {
            return Err(format!(
                "Invalid FEN: unrecognized character `{c}'"
            ));
        }
    }

    let splits = board.chars().filter(|c| c == &'/').count();
    if splits != 7 {
        return Err(format!("Invalid FEN: number of ranks is not 8"));
    }

    let mut ranks = Vec::with_capacity(8);
    board.split('/').collect_into(&mut ranks);

    let mut expanded_ranks = Vec::with_capacity(8);

    for rank in ranks {
        if rank.len() > 8 {
            return Err(format!(
                "Invalid FEN: rank not 8 squares `{rank}`"
            ));
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
            return Err(format!(
                "Invalid FEN: rank not 8 squares `{rank}`"
            ));
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

pub fn color_piece_letter(c: char) -> Option<ColoredChessPiece> {
    use ColoredChessPiece::*;
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

pub fn render_fen(board: &BitBoard, halfmove: u16) -> String {
    format!(
        "{} {} {} {} {} {}",
        render_fen_board(&board.render()),
        board.metadata.to_move.letter(),
        board.metadata.castling_rights.to_string(board.metadata.castling_details),
        if let Some(sq) = board.metadata.en_passant {
            sq.to_str()
        } else {
            "-"
        },
        halfmove,
        board.metadata.turn
    )
}

pub fn render_fen_board(board: &ArrayBoard<Option<ColoredChessPiece>>) -> String {
    let mut res = Vec::with_capacity(8);

    let mut it = Some(Square::a1);
    let mut n = 0;
    let mut s = String::with_capacity(8);

    while let Some(sq) = it {
        if let Some(c) = board.at(sq) {
            if n != 0 {
                s += &n.to_string();
            }
            s.push(c.letter());
            n = 0;
        } else {
            n += 1;
        }

        if sq.file_rank().0 == BoardFile::H {
            if n != 0 {
                s += &n.to_string();
            }
            n = 0;
            res.push(s);
            s = String::with_capacity(8);
        }

        it = sq.next();
    }

    res.reverse();
    res.join("/")
}

impl ColoredChessPiece {
    pub fn letter(self) -> char {
        use ColoredChessPiece::*;
        match self {
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
}

impl Color {
    pub fn letter(self) -> char {
        match self {
            Self::White => 'w',
            Self::Black => 'b',
        }
    }
}

impl CastlingRights {
    pub fn to_string(self, deets: CastlingDetails) -> String {
        
        let mut res = String::with_capacity(4);

        // if deets.capture_own_rook {
        //
        // }

        if self.westward(Color::White) {
            res.push('K');
        }

        if self.eastward(Color::White) {
            res.push('Q');
        }

        if self.westward(Color::Black) {
            res.push('k');
        }
        
        if self.eastward(Color::Black) {
            res.push('q');
        }

        if res.is_empty() {
            res.push('-');
        }

        res
    }
}