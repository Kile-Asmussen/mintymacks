
1. Fuzz Zobrist hashing
2. Fuzz move undo
3. Fuzz move gen count against stockfish
4. Implement perft and compare to stockfish
5. Implement state tracking
    * History of irreversible moves
    * History of hashed board positions sans active player
6. Fuzz hashed boards
7. Determine draw conditions
    * Insufficient checkmating material
    * 75 move forced draw
    * Threefold repetition
8. Expand to full UCI spec
9. Minimax algorithm