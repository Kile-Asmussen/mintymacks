use crate::{
    bits::{BoardMask, board::BitBoard},
    model::{
        BoardFile, BoardRank, ChessPiece, Color, Square,
        moves::{ChessMove, SpecialMove},
    },
    notation::regexp,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AlgebraicMove {
    pub piece: ChessPiece,
    pub file_origin: Option<BoardFile> = None,
    pub rank_origin: Option<BoardRank> = None,
    pub destination: Square,
    pub capture: bool = false,
    pub special: Option<SpecialMove> = None,
    pub check_or_mate: Option<bool> = None,
}

impl AlgebraicMove {
    pub fn matches(self, mv: ChessMove) -> bool {
        if let Some(SpecialMove::CastlingEastward) | Some(SpecialMove::CastlingWestward) =
            self.special
        {
            return mv.special == self.special;
        }

        self.piece == mv.piece.piece()
            && self.destination == mv.pmv.to
            && self.capture == mv.cap.is_some()
            && self.special == mv.special
            && (self.file_origin.is_none() || self.file_origin == Some(mv.pmv.from.file_rank().0))
            && (self.rank_origin.is_none() || self.rank_origin == Some(mv.pmv.from.file_rank().1))
    }

    pub fn to_string(self) -> String {
        let mut res = "".to_string();

        if self.special == Some(SpecialMove::CastlingEastward) {
            res = "O-O-O".to_string();
        } else if self.special == Some(SpecialMove::CastlingWestward) {
            res = "O-O".to_string();
        } else {
            if self.piece != ChessPiece::Pawn {
                res.push(self.piece.letter());
            }

            if let Some(f) = self.file_origin {
                res.push(f.letter());
            }

            if let Some(r) = self.rank_origin {
                res.push(r.digit());
            }

            if self.capture {
                res.push('x');
            }

            res += self.destination.to_str();

            if let Some(SpecialMove::Promotion(p)) = self.special {
                res.push('=');
                res.push(p.letter());
            }
        }

        if let Some(false) = self.check_or_mate {
            res.push('+');
        } else if let Some(true) = self.check_or_mate {
            res.push('#');
        }

        return res;
    }

    pub fn parse(s: &str) -> Option<AlgebraicMove> {
        let check_or_mate = if s.ends_with("+") {
            Some(false)
        } else if s.ends_with("#") {
            Some(true)
        } else {
            None
        };

        if regexp!("^O-O-O[+#]?$").is_match(s) {
            return Some(AlgebraicMove {
                piece: ChessPiece::King,
                file_origin: None,
                rank_origin: None,
                destination: Square::a1,
                capture: false,
                special: Some(SpecialMove::CastlingEastward),
                check_or_mate,
            });
        }

        if regexp!("^O-O[+#]?$").is_match(s) {
            return Some(AlgebraicMove {
                piece: ChessPiece::King,
                file_origin: None,
                rank_origin: None,
                destination: Square::a1,
                capture: false,
                special: Some(SpecialMove::CastlingWestward),
                check_or_mate,
            });
        }

        if let Some(c) = regexp!("^((?:[a-h]x)?)([a-h][1-8])((?:=[NBRQ])?)[+#]?$").captures(s) {
            let (_, [file_origin, destination, promotion]) = c.extract::<3>();
            return Some(AlgebraicMove {
                piece: ChessPiece::Pawn,
                file_origin: BoardFile::parse(file_origin.trim_end_matches('x')),
                destination: Square::parse(destination).unwrap(),
                check_or_mate,
                special: ChessPiece::parse(promotion.trim_start_matches('=')).map(SpecialMove::Promotion),
                rank_origin: None,
                capture: !file_origin.is_empty()
            })
        }

        if let Some(c) = regexp!("^([NBRQK])([a-h]?)([1-8]?)(x?)([a-h][1-8])[+#]?$").captures(s) {
            let (_, [piece, file_origin, rank_origin, capture, destination]) = c.extract::<5>();
            return Some(AlgebraicMove {
                piece: ChessPiece::parse(piece).unwrap(),
                file_origin: BoardFile::parse(file_origin),
                rank_origin: BoardRank::parse(rank_origin),
                destination: Square::parse(destination).unwrap(),
                check_or_mate,
                special: None,
                capture: !capture.is_empty()
            })
        }

        return None;
    }
}

impl ChessPiece {
    pub fn letter(self) -> char {
        match self {
            ChessPiece::Pawn => 'P',
            ChessPiece::Knight => 'N',
            ChessPiece::Bishop => 'B',
            ChessPiece::Rook => 'R',
            ChessPiece::Queen => 'Q',
            ChessPiece::King => 'K',
        }
    }

    pub fn parse(l: &str) -> Option<Self> {
        Some(match l {
            "P" | "p" => ChessPiece::Pawn,
            "N" | "n" => ChessPiece::Knight,
            "B" | "b" => ChessPiece::Bishop,
            "R" | "r" => ChessPiece::Rook,
            "Q" | "q" => ChessPiece::Queen,
            "K" | "k" => ChessPiece::King,
            _ => return None,
        })
    }
}

impl BoardFile {
    pub fn letter(self) -> char {
        match self {
            BoardFile::A => 'a',
            BoardFile::B => 'b',
            BoardFile::C => 'c',
            BoardFile::D => 'd',
            BoardFile::E => 'e',
            BoardFile::F => 'f',
            BoardFile::G => 'g',
            BoardFile::H => 'h',
        }
    }

    pub fn parse(f: &str) -> Option<Self> {
        Some(match f {
            "a" => BoardFile::A,
            "b" => BoardFile::B,
            "c" => BoardFile::C,
            "d" => BoardFile::D,
            "e" => BoardFile::E,
            "f" => BoardFile::F,
            "g" => BoardFile::G,
            "h" => BoardFile::H,
            _ => return None
        })
    }
}

impl BoardRank {
    pub fn digit(self) -> char {
        match self {
            BoardRank::_1 => '1',
            BoardRank::_2 => '2',
            BoardRank::_3 => '3',
            BoardRank::_4 => '4',
            BoardRank::_5 => '5',
            BoardRank::_6 => '6',
            BoardRank::_7 => '7',
            BoardRank::_8 => '8',
        }
    }

    pub fn parse(r: &str) -> Option<Self> {
        Some(match r {
            "1" => BoardRank::_1,
            "2" => BoardRank::_2,
            "3" => BoardRank::_3,
            "4" => BoardRank::_4,
            "5" => BoardRank::_5,
            "6" => BoardRank::_6,
            "7" => BoardRank::_7,
            "8" => BoardRank::_8,
            _ => return None
        })
    }
}

impl ChessMove {
    pub fn ambiguate(self, board: &BitBoard, moves: &[ChessMove]) -> AlgebraicMove {
        let mut guess = AlgebraicMove {
            piece: self.piece.piece(),
            rank_origin: Some(self.pmv.from.file_rank().1),
            file_origin: Some(self.pmv.from.file_rank().0),
            destination: self.pmv.to,
            capture: self.cap.is_some(),
            special: self.special,
            check_or_mate: None,
        };

        if self.special == Some(SpecialMove::CastlingEastward) {
            guess.destination = Square::a1;
            guess.rank_origin = None;
            guess.file_origin = None;
        }

        let (this, enemy) = board.active_passive(self.piece.color());
        let threats = this.threats(self.piece.color(), enemy.total(), Some(self.pmv), self.cap);
        if threats & enemy.kings != BoardMask::MIN {
            guess.check_or_mate = Some(false);
        }

        if guess.check_or_mate.is_some() {
            let mut board = board.clone();
            board.apply(self);
            let mut moves = vec![];
            board.moves(&mut moves);
            if moves.is_empty() {
                guess.check_or_mate = Some(true);
            }
        }

        if self.piece.piece() == ChessPiece::Pawn {
            if guess.capture {
                guess.file_origin = Some(self.pmv.to.file_rank().0);
                guess.rank_origin = None;
            }

            return guess;
        }

        if let Some(SpecialMove::CastlingEastward) | Some(SpecialMove::CastlingWestward) =
            guess.special
        {
            return guess;
        }

        let mut guess2 = guess;
        let mut guess3 = guess;
        let mut guess4 = guess;
        guess2.file_origin = None;
        guess3.rank_origin = None;
        guess4.file_origin = None;
        guess4.rank_origin = None;

        if unique(moves, guess4){
            return guess4;
        }

        if unique(moves, guess2) {
            return guess2;
        }

        if unique(moves, guess3) {
            return guess3;
        }

        return guess;

        fn unique(moves: &[ChessMove], guess: AlgebraicMove) -> bool {
            moves
                .into_iter()
                .filter(|mv| guess.matches(**mv)).count() == 1
        }
    }
}
