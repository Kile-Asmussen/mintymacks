use crate::{
    bits::board::BitBoard,
    zobrist::{self, ZOBHASHER, ZobristBoard},
};

#[test]
fn delta_hashing() {
    let mut board = BitBoard::startpos();
    let details = board.metadata.castling_details;

    let mut hash1 = ZOBHASHER.hash(&board);

    let mut moves = vec![];
    board.moves(&mut moves);

    let mv = *moves.first().unwrap();
    hash1 ^= ZOBHASHER.delta(mv, details);

    board.apply(mv);
    let hash2 = ZOBHASHER.hash(&board);

    assert_eq!(hash1, hash2);
}
