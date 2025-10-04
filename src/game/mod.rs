use crate::{arrays::ArrayBoard, bits::board::BitBoard, model::{moves::ChessMove, ColoredChessPiece, Victory}, notation::{algebraic::AlgebraicMove, pgn::{PGNHeaders, PGN}}, zobrist::ZobHash};


#[derive(Debug)]
struct GameState {
    pub move_sequence: Vec<(ZobHash, ChessMove, ZobHash)>,
    pub start: Option<[String; 6]>,
    pub board: BitBoard,
    pub next: Vec<ChessMove>,
    pub metadata: PGNHeaders,
}

