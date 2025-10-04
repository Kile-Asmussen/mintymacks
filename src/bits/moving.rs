use crate::{
    bits::{
        BoardMask,
        board::{BitBoard, HalfBitBoard},
    },
    model::{
        BoardRank, ChessPiece, Color, ColoredChessPiece, Square,
        castling::{self, CastlingDetail, CastlingDetails, CastlingMove, CastlingRights},
        metadata::{self, Metadata},
        moves::{ChessMove, PseudoMove, SpecialMove},
    },
};

impl BitBoard {
    /// Calling this method on a Move value that
    /// does not come from the BitBoard::moves method
    /// is unspecified behavior
    pub fn apply(&mut self, mv: ChessMove) {
        self.apply_no_metadata(mv);
        self.metadata.apply(mv);
    }

    /// Calling this method with a Move value that was
    /// not used with the BitBoard::apply method immediately
    /// before this call, is unspecified behavior
    pub fn unapply(&mut self, mv: ChessMove) {
        self.apply_no_metadata(mv);
        self.metadata.unapply(mv);
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
        if let Some(mv) = buf.iter().find(|m| m.simplify() == mv) {
            self.apply(*mv);
            Some(*mv)
        } else {
            None
        }
    }

    #[inline]
    fn apply_no_metadata(&mut self, mv: ChessMove) {
        let cd = self.metadata.castling_details;
        let (act, pas) = self.active_passive_mut(mv.piece.color());
        act.apply_active(cd, mv);
        pas.apply_passive(mv);
    }

    fn active_passive_mut(&mut self, color: Color) -> (&mut HalfBitBoard, &mut HalfBitBoard) {
        match color {
            Color::White => (&mut self.white, &mut self.black),
            Color::Black => (&mut self.black, &mut self.white),
        }
    }

    pub fn active_passive(&self, color: Color) -> (&HalfBitBoard, &HalfBitBoard) {
        match color {
            Color::White => (&self.white, &self.black),
            Color::Black => (&self.black, &self.white),
        }
    }
}

impl HalfBitBoard {
    #[inline]
    pub fn apply_active(&mut self, castling: CastlingDetails, mv: ChessMove) {
        if let Some(sp) = mv.special {
            match sp {
                SpecialMove::CastlingWestward => {
                    castling_move(self, castling.westward, mv.piece.color())
                }
                SpecialMove::CastlingEastward => {
                    castling_move(self, castling.eastward, mv.piece.color())
                }
                SpecialMove::Promotion(p) => {
                    self.pawns ^= mv.pmv.from.bit();
                    *self.piece(p) ^= mv.pmv.to.bit();
                }
                _ => {}
            }
        } else {
            *self.piece(mv.piece.piece()) ^= mv.pmv.bits();
            self.total ^= mv.pmv.bits();
        }

        #[inline]
        fn castling_move(_self: &mut HalfBitBoard, cd: CastlingDetail, c: Color) {
            let cmv = cd.reify(c);
            _self.kings ^= cmv.king_move.bits();
            _self.rooks ^= cmv.rook_move.bits();
        }
    }

    #[inline]
    pub fn apply_passive(&mut self, mv: ChessMove) {
        if let Some((p, sq)) = mv.cap {
            *self.piece(p) ^= sq.bit();
            self.total ^= sq.bit();
        }
    }

    pub fn piece(&mut self, p: ChessPiece) -> &mut BoardMask {
        match p {
            ChessPiece::Pawn => &mut self.pawns,
            ChessPiece::Knight => &mut self.knights,
            ChessPiece::Bishop => &mut self.bishops,
            ChessPiece::Rook => &mut self.rooks,
            ChessPiece::Queen => &mut self.queens,
            ChessPiece::King => &mut self.kings,
        }
    }
}

impl Metadata {
    #[inline]
    pub fn apply(&mut self, mv: ChessMove) {
        self.castling_rights = mv.castling_change(self.castling_details);
        self.en_passant = mv.ep_opening();
        self.to_move = mv.piece.color().opposite();
        if mv.piece.color() == Color::Black {
            self.turn += 1;
        }
    }

    #[inline]
    fn unapply(&mut self, mv: ChessMove) {
        self.en_passant = mv.epc;
        self.castling_rights = mv.rights;
        self.to_move = mv.piece.color();
        if mv.piece.color() == Color::Black {
            self.turn -= 1;
        }
    }
}
