use crate::{
    arrays::ArrayBoard,
    bits::board::BitBoard,
    ix_map,
    model::{
        BoardFile, BoardRank, ChessPiece, ColoredChessPiece, Square, Victory,
        castling::CastlingRights,
        moves::{ChessMove, PseudoMove, SpecialMove},
    },
    notation::{
        algebraic::AlgebraicMove,
        fen::{parse_fen, parse_fen_board, render_fen_board},
        pgn::{GameToken, MovePair, PGN, Tag},
    },
};

#[test]
fn fen_board_roundtrip() {
    assert_eq!(
        parse_fen_board("8/8/8/8/8/8/8/8").unwrap(),
        ArrayBoard::new(None)
    );

    assert_eq!(
        render_fen_board(&parse_fen_board("8/8/8/8/8/8/8/8").unwrap()),
        "8/8/8/8/8/8/8/8".to_string()
    );

    use ColoredChessPiece::*;

    let startpos = ArrayBoard::setup([
        [
            BlackRook,
            BlackKnight,
            BlackBishop,
            BlackQueen,
            BlackKing,
            BlackBishop,
            BlackKnight,
            BlackRook,
        ]
        .map(Some),
        [BlackPawn; 8].map(Some),
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [WhitePawn; 8].map(Some),
        [
            WhiteRook,
            WhiteKnight,
            WhiteBishop,
            WhiteQueen,
            WhiteKing,
            WhiteBishop,
            WhiteKnight,
            WhiteRook,
        ]
        .map(Some),
    ]);

    assert_eq!(
        parse_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap(),
        startpos
    );

    assert_eq!(
        render_fen_board(&startpos),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );

    assert_eq!(
        (BitBoard::startpos(), 0),
        parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(),
    );
}

#[test]
fn longalg_roundtrips() {
    assert_eq!(
        Some(Square::a7.to(Square::a8).q()),
        PseudoMove::parse(&Square::a7.to(Square::a8).longalg(Some(ChessPiece::Queen)))
    );

    assert_eq!("a7a8q", {
        let (pmv, pr) = PseudoMove::parse("a7a8q").unwrap();
        pmv.longalg(pr)
    })
}

#[test]
fn algebraic_roundtrip() {
    let board = BitBoard::startpos();
    let mut moves = vec![];
    board.moves(&mut moves);

    let mv = AlgebraicMove {
        piece: ChessPiece::Knight,
        destination: Square::c3,
        file_origin: None,
        rank_origin: None,
        capture: false,
        special: None,
        check_or_mate: None,
    };

    assert_eq!(moves.iter().filter(|m| mv.matches(**m)).count(), 1);

    let q = moves.iter().find(|m| mv.matches(**m)).map(|m| *m);

    assert_eq!(
        q,
        Some(ChessMove {
            piece: ColoredChessPiece::WhiteKnight,
            pmv: Square::b1.to(Square::c3),
            cap: None,
            special: None,
            rights: CastlingRights::full(),
            epc: None
        })
    );

    let q = q.unwrap();

    let mv2 = q.ambiguate(&board, &moves);

    assert_eq!(mv2, mv);

    assert_eq!(
        Some(AlgebraicMove {
            piece: ChessPiece::Pawn,
            file_origin: Some(BoardFile::E),
            rank_origin: None,
            destination: Square::f8,
            capture: true,
            special: Some(SpecialMove::Promotion(ChessPiece::Rook)),
            check_or_mate: Some(false)
        }),
        AlgebraicMove::parse("exf8=R+")
    );

    assert_eq!(
        Some(AlgebraicMove {
            piece: ChessPiece::Bishop,
            file_origin: Some(BoardFile::A),
            rank_origin: Some(BoardRank::_1),
            destination: Square::h8,
            capture: true,
            special: None,
            check_or_mate: Some(true)
        }),
        AlgebraicMove::parse("Ba1xh8#")
    );
}

#[test]
fn pgn_tag_pairs() {
    let (hash, rest) = PGN::parse_tag_pairs(
        r##"
    [Event "F/S Return Match"]
    [Site "Belgrade, Serbia JUG"]
    [Date "1992.11.04"]
    [Round "29"]
    [White "Fischer, Robert J."]
    [Black "Spassky, Boris V."]
    [Result "1/2-1/2"]

    1.e4 e5 2.Nf3 Nc6 3.Bb5 {This opening is called the Ruy Lopez.} 3...a6
    4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
    11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
    Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
    23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
    hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
    35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6
    Nf2 42.g4 Bd3 43.Re6 1/2-1/2
    "##,
    );

    let reference = ix_map! {
        "Event".to_string() => "F/S Return Match".to_string(),
        "Site".to_string() => "Belgrade, Serbia JUG".to_string(),
        "Date".to_string() => "1992.11.04".to_string(),
        "Round".to_string() => "29".to_string(),
        "White".to_string() => "Fischer, Robert J.".to_string(),
        "Black".to_string() => "Spassky, Boris V.".to_string(),
        "Result".to_string() => "1/2-1/2".to_string(),
    };

    assert_eq!(hash["Event"], reference["Event"]);
    assert_eq!(hash["Site"], reference["Site"]);
    assert_eq!(hash["Date"], reference["Date"]);
    assert_eq!(hash["Round"], reference["Round"]);
    assert_eq!(hash["White"], reference["White"]);
    assert_eq!(hash["Black"], reference["Black"]);
    assert_eq!(hash["Result"], reference["Result"]);

    assert_eq!(
        rest,
        r##"

    1.e4 e5 2.Nf3 Nc6 3.Bb5 {This opening is called the Ruy Lopez.} 3...a6
    4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
    11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
    Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
    23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
    hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
    35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6
    Nf2 42.g4 Bd3 43.Re6 1/2-1/2
    "##
    )
}

#[test]
fn pgn_game() {
    let (game, file) = PGN::parse_game(
        r##"

    1.e4 e5 2.Nf3 Nc6 3.Bb5 {This opening is called the Ruy Lopez.} 3...a6
    4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
    11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
    Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
    23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
    hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
    35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6
    Nf2 42.g4 Bd3 43.Re6 1/2-1/2
    "##,
    );

    assert_eq!(game[0].white.unwrap().to_string(), "e4");
    assert_eq!(game.len(), 43);
}

#[test]
fn pgn_full() {
    let (pgn, rest) = PGN::parse(
        r##"
    [Event "F/S Return Match"]
    [Site "Belgrade, Serbia JUG"]
    [Date "1992.11.04"]
    [Round "29"]
    [White "Fischer, Robert J."]
    [Black "Spassky, Boris V."]
    [Result "1/2-1/2"]

    1.e4 $1 e5 2.Nf3 Nc6 3.Bb5 {This opening is called the Ruy Lopez.} 3...a6
    4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
    11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
    Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
    23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
    hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
    35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6
    Nf2 42.g4 Bd3 43.Re6 1/2-1/2
    "##,
    );

    assert!(pgn.is_some());

    let pgn = pgn.unwrap();

    for m in &pgn.moves {
        println!("{}", m.to_string())
    }
    assert_eq!(pgn.end, Some(Victory::Draw));
    assert_eq!(
        pgn.headers.canon.get(&Tag::Event).map(|s| &s[..]),
        Some("F/S Return Match")
    );
    assert_eq!(pgn.moves.len(), 43);
}
