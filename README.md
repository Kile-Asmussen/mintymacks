# Mintymacks

This is a chess library with the following features:

- Bit-board board state representation
- Legal move generation and a simple perft
- Array board representation for setup
- Zobrist hashing with move deltas
- Algebraic notation
- FEN parsing
- PGN parsing
- UCI parsing and printing
- UCI protocol engine loading and interaction through Tokio
- ECO opening database

It is extensively tested, including verifying the move generation correctness against Stockfish