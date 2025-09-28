use crate::{
    bits::{BoardMask, board::BitBoard},
    model::{
        Color, File, Piece, Rank, Square,
        moves::{ChessMove, Special},
    },
    regex,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AlgebraicMove {
    pub piece: Piece,
    pub file_origin: Option<File>,
    pub rank_origin: Option<Rank>,
    pub destination: Square,
    pub capture: bool,
    pub special: Option<Special>,
    pub check_or_mate: Option<bool>,
}

impl AlgebraicMove {
    pub fn matches(self, mv: ChessMove) -> bool {
        if let Some(Special::CastlingEastward) | Some(Special::CastlingWestward) = self.special {
            return mv.special == self.special;
        }

        self.piece == mv.piece.piece()
            && self.destination == mv.pmv.to
            && self.capture == mv.cap.is_some()
            && self.special == mv.special
            && self
                .file_origin
                .map(|r| r == mv.pmv.from.file_rank().0)
                .unwrap_or(true)
            && self
                .rank_origin
                .map(|f| f == mv.pmv.from.file_rank().1)
                .unwrap_or(true)
    }

    pub fn to_string(self) -> String {
        let mut res = "".to_string();

        if self.special == Some(Special::CastlingEastward) {
            res = "O-O-O".to_string();
        } else if self.special == Some(Special::CastlingWestward) {
            res = "O-O".to_string();
        } else {
            if self.piece != Piece::Pawn {
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

            if let Some(Special::Promotion(p)) = self.special {
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

        if regex!("^O-O-O[+#]?$").is_match(s) {
            return Some(AlgebraicMove {
                piece: Piece::King,
                file_origin: None,
                rank_origin: None,
                destination: Square::a1,
                capture: false,
                special: Some(Special::CastlingEastward),
                check_or_mate,
            });
        }

        if regex!("^O-O[+#]?$").is_match(s) {
            return Some(AlgebraicMove {
                piece: Piece::King,
                file_origin: None,
                rank_origin: None,
                destination: Square::a1,
                capture: false,
                special: Some(Special::CastlingWestward),
                check_or_mate,
            });
        }

        if let Some(c) = regex!("^(?:([a-h])x)?([a-h][1-8])(?:=([NBRQ]?))[+#]?$").captures(s) {
            if let Some(b) = c.get(1) {
                b.as_str();
            }
        }

        if let Some(c) = regex!("^([NBRQK])([a-h])?([1-8])?(x)?([a-h][1-8])[+#]?$").captures(s) {}

        return None;
    }
}

impl Piece {
    fn letter(self) -> char {
        match self {
            Piece::Pawn => 'P',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }

    fn parse(l: &str) -> Option<Self> {
        Some(match l {
            "P" | "p" => Piece::Pawn,
            "K" | "k" => Piece::Knight,
            "B" | "b" => Piece::Bishop,
            "R" | "r" => Piece::Rook,
            "Q" | "q" => Piece::Queen,
            "K" | "k" => Piece::King,
            _ => return None,
        })
    }
}

impl File {
    fn letter(self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

impl Rank {
    fn digit(self) -> char {
        match self {
            Rank::_1 => '1',
            Rank::_2 => '2',
            Rank::_3 => '3',
            Rank::_4 => '4',
            Rank::_5 => '5',
            Rank::_6 => '6',
            Rank::_7 => '7',
            Rank::_8 => '8',
        }
    }
}

impl ChessMove {
    pub fn ambiguate(self, board: &BitBoard, moves: &[ChessMove]) -> AlgebraicMove {
        let mut guess = AlgebraicMove {
            piece: self.piece.piece(),
            rank_origin: None,
            file_origin: None,
            destination: self.pmv.to,
            capture: self.cap.is_some(),
            special: self.special,
            check_or_mate: None,
        };

        let (this, enemy) = board.active_passive(self.piece.color());
        let threats = this.threats(self.piece.color(), enemy.total(), Some(self.pmv), self.cap);
        if threats & enemy.kings != BoardMask::MIN {
            guess.check_or_mate = Some(false);
        }

        if self.piece.piece() == Piece::Pawn {
            if guess.capture {
                guess.file_origin = Some(self.pmv.to.file_rank().0)
            }

            return guess;
        }

        if let Some(Special::CastlingEastward) | Some(Special::CastlingWestward) = guess.special {
            return guess;
        }

        if !unique(self, moves, guess) {
            guess.file_origin = Some(self.pmv.from.file_rank().0)
        }

        if !unique(self, moves, guess) {
            guess.file_origin = None;
            guess.rank_origin = Some(self.pmv.from.file_rank().1);
        }

        if !unique(self, moves, guess) {
            guess.file_origin = Some(self.pmv.from.file_rank().0);
        }

        return guess;

        fn unique(current: ChessMove, moves: &[ChessMove], guess: AlgebraicMove) -> bool {
            moves
                .into_iter()
                .filter(|mv| **mv != current && guess.matches(**mv))
                .count()
                == 1
        }
    }
}
