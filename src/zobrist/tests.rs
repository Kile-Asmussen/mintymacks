use crate::{
    bits::board::BitBoard,
    zobrist::{self, ZobristBoard},
};

#[test]
fn delta_hashing() {
    let zobrist = ZobristBoard::new();

    let mut board = BitBoard::startpos();
    let details = board.metadata.castling_details;

    let mut hash1 = zobrist.hash(&board);

    let mut moves = vec![];
    board.moves(&mut moves);

    let mv = *moves.first().unwrap();
    hash1 ^= zobrist.delta(mv, details);

    board.apply(mv);
    let hash2 = zobrist.hash(&board);

    assert_eq!(hash1, hash2);
}
