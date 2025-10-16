use crate::{
    arrays::ArrayBoard,
    bits::{
        BoardMask, Squares,
        attacks::{bishop_attacks, knight_attacks, rook_attacks},
        board::{BitBoard, HalfBitBoard},
        jumps::KNIGHT_MOVES,
        mask,
        movegen::{legal_moves, pawn_moves},
        one_bit, show_mask, slides,
    },
    fuzzing::stockfish_perft,
    model::{
        Color, ColoredChessPiece, Square,
        castling::{CLASSIC_CASTLING, CastlingRights},
        metadata::Metadata,
        moves::{ChessMove, PseudoMove},
    },
    notation::{
        algebraic,
        fen::{self, parse_fen, parse_fen_board, render_fen},
    },
};
#[cfg(test)]
use crate::{model::ChessPiece, notation::algebraic::AlgebraicMove};

impl BitBoard {
    #[cfg(test)]
    pub fn apply_algebraic(&mut self, mv: AlgebraicMove) -> Option<ChessMove> {
        let mut buf = vec![];
        self.algebraic_internal(mv, &mut buf)
    }

    #[cfg(test)]
    pub fn apply_algebraics(&mut self, mvs: &[AlgebraicMove]) -> Vec<ChessMove> {
        let mut res = vec![];
        let mut buf = vec![];
        for mv in mvs {
            if let Some(mv) = self.algebraic_internal(*mv, &mut buf) {
                res.push(mv);
            } else {
                break;
            }
        }
        return res;
    }

    #[cfg(test)]
    pub fn apply_pseudomove(&mut self, mv: (PseudoMove, Option<ChessPiece>)) -> Option<ChessMove> {
        let mut buf = vec![];
        self.pseudomove_internal(mv, &mut buf)
    }

    #[cfg(test)]
    pub fn apply_pseudomoves(
        &mut self,
        mvs: &[(PseudoMove, Option<ChessPiece>)],
    ) -> Vec<ChessMove> {
        let mut res = vec![];
        let mut buf = vec![];
        for mv in mvs {
            if let Some(mv) = self.pseudomove_internal(*mv, &mut buf) {
                res.push(mv);
            } else {
                break;
            }
        }
        return res;
    }

    #[cfg(test)]
    fn pseudomove_internal(
        &mut self,
        mv: (PseudoMove, Option<ChessPiece>),
        buf: &mut Vec<ChessMove>,
    ) -> Option<ChessMove> {
        buf.clear();
        self.moves(buf);
        let matches = buf
            .iter()
            .filter(|m| mv == m.simplify())
            .map(|mv| *mv)
            .collect::<Vec<_>>();

        if let [mv] = &matches[..] {
            self.apply(*mv);
            Some(*mv)
        } else {
            None
        }
    }

    #[cfg(test)]
    fn algebraic_internal(
        &mut self,
        mv: AlgebraicMove,
        buf: &mut Vec<ChessMove>,
    ) -> Option<ChessMove> {
        use crate::notation::MoveMatcher;

        buf.clear();
        self.moves(buf);
        let matches = buf
            .iter()
            .filter(|m| mv.matches(**m))
            .map(|mv| *mv)
            .collect::<Vec<_>>();

        if let [mv] = &matches[..] {
            self.apply(*mv);
            Some(*mv)
        } else {
            return None;
        }
    }
}

#[test]
fn knight_threat_masks() {
    let t = knight_attacks(mask([
        0b_00000000,
        0b_00000000,
        0b_00000100,
        0b_00000000,
        0b_00000000,
        0b_00100000,
        0b_00000000,
        0b_00000000,
    ]));

    assert_eq!(
        t,
        mask([
            0b_00001010,
            0b_00010001,
            0b_00000000,
            0b_01010001,
            0b_10001010,
            0b_00000000,
            0b_10001000,
            0b_01010000,
        ])
    );
}

#[test]
fn rook_threat_masks() {
    let t = rook_attacks(
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000100,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
        ]),
        mask([
            0b_00000000,
            0b_00000100,
            0b_00000110,
            0b_00000000,
            0b_00000000,
            0b_00101000,
            0b_00000000,
            0b_00000000,
        ]),
    );

    assert_eq!(
        t,
        mask([
            0b_00100000,
            0b_00100100,
            0b_11111010,
            0b_00100100,
            0b_00100100,
            0b_11011100,
            0b_00100100,
            0b_00100100,
        ])
    )
}

#[test]
fn bishop_threat_masks() {
    let t = bishop_attacks(
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000100,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
        ]),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000100,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
        ]),
    );

    assert_eq!(
        t,
        mask([
            0b_00010001,
            0b_00001010,
            0b_00000100,
            0b_10001010,
            0b_01010001,
            0b_00100000,
            0b_01010000,
            0b_10001000,
        ])
    )
}

fn test_move_numbers(fen: &str, c: Color, cr: CastlingRights, epc: Option<Square>, num: usize) {
    let board = fen::parse_fen_board(fen).unwrap();

    let white = HalfBitBoard::new(Color::White, &board);
    let black = HalfBitBoard::new(Color::Black, &board);
    let metadata = Metadata {
        to_move: c,
        castling_rights: cr,
        turn: 1,
        halfmove_clock: 0,
        en_passant: epc,
        castling_details: CLASSIC_CASTLING,
        hash: 0,
    };

    let mut moves = vec![];
    legal_moves(&white, &black, metadata, &mut moves);
    println!("FEN: {}", fen);
    println!(
        "moves: {}",
        moves
            .iter()
            .map(|x| x.longalg())
            .collect::<Vec<_>>()
            .join(" ")
    );
    assert_eq!(moves.len(), num);
}

#[test]
fn test_movegen() {
    test_move_numbers(
        "8/8/8/8/8/8/8/8",
        Color::White,
        CastlingRights::nil(),
        None,
        0,
    );

    test_move_numbers(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
        Color::White,
        CastlingRights::full(),
        None,
        20,
    );

    test_move_numbers(
        "R7/8/8/8/8/8/8/8",
        Color::White,
        CastlingRights::nil(),
        None,
        14,
    );

    test_move_numbers(
        "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1",
        Color::White,
        CastlingRights::nil(),
        None,
        218,
    );
}

#[test]
fn test_moving() {
    let mut board = BitBoard::startpos();
    board.apply(ChessMove {
        cpc: ColoredChessPiece::WhitePawn.with_cap(None),
        pmv: Square::d2.to(Square::d4),
        cap: None,
        spc: None,
        hmc: 0,
        cr: CastlingRights::full(),
        epc: None,
    });

    assert_eq!(
        board.render(),
        fen::parse_fen_board("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR").unwrap()
    );
}

#[test]
fn test_knight_move_corner_case() {
    assert_eq!(
        KNIGHT_MOVES.at(Square::c1),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_01010000,
            0b_10001000,
            0b_00000000,
        ])
    );

    assert_eq!(
        KNIGHT_MOVES.at(Square::d1),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00101000,
            0b_01000100,
            0b_00000000,
        ])
    );

    assert_eq!(
        KNIGHT_MOVES.at(Square::g1),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000101,
            0b_00001000,
            0b_00000000,
        ])
    );
}

#[test]
fn en_passant_pawn_capture() {
    let mut board = ArrayBoard::<Option<ColoredChessPiece>>::new(None);

    board.set(Square::a7, Some(ColoredChessPiece::BlackPawn));
    board.set(Square::b5, Some(ColoredChessPiece::WhitePawn));
    board.set(Square::h1, Some(ColoredChessPiece::WhiteKing));
    board.set(Square::h8, Some(ColoredChessPiece::BlackKing));

    let mut board = BitBoard::new(
        &board,
        Color::Black,
        1,
        0,
        CastlingRights::nil(),
        None,
        CLASSIC_CASTLING,
    );

    let mv = board
        .apply_pseudomove(Square::a7.to(Square::a5).p())
        .unwrap();

    println!("({:?}).epc_opening() == {:?}", mv, mv.ep_opening());
    println!();

    let mut res = vec![];
    board.moves(&mut res);

    for mv in &res {
        println!("{:?}", mv)
    }
}
