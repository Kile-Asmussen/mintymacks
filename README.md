# Mintymacks

This is a chess library with the following features:

- Bit-board board state representation
- Legal move generation
- Array board representation for setup
- Zobrist hashing with move deltas
- Algebraic notation
- FEN parsing
- PGN parsing
- UCI parsing and printing

It is extensively tested, including verifying the move generation correctness against Stockfish

# See also

- mintymacks_tui --- a terminal user interface for analyzing games, playing games against engines, or playing engines against each other
- mintymacks_engine --- a minimax-based engine using this library