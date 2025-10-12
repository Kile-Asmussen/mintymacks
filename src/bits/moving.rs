#[cfg(test)]
use crate::notation::algebraic::AlgebraicMove;
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
    zobrist::ZOBHASHER,
};

impl BitBoard {
    /// Calling this method on a Move value that
    /// does not come from the BitBoard::moves method
    /// is unspecified behavior
    pub fn apply(&mut self, mv: ChessMove) {
        self.apply_no_metadata(mv);
        self.metadata.apply(mv);
        let (_, pas) = self.active_passive(mv.cpc.color());
        self.metadata.hash ^= ZOBHASHER.delta(mv, self.metadata.castling_details)
    }

    /// Calling this method with a Move value that was
    /// not used with the BitBoard::apply method immediately
    /// before this call, is unspecified behavior
    pub fn unapply(&mut self, mv: ChessMove) {
        self.apply_no_metadata(mv);
        self.metadata.unapply(mv);
        let (_, pas) = self.active_passive(mv.cpc.color());
        self.metadata.hash ^= ZOBHASHER.delta(mv, self.metadata.castling_details)
    }

    #[cfg(test)]
    pub fn apply_algebraic(&mut self, mv: AlgebraicMove) -> Option<ChessMove> {
        let mut buf = vec![];
        self.algebraic_internal(mv, &mut buf)
    }

    #[cfg(test)]
    pub fn apply_algebraics(&mut self, mvs: &[AlgebraicMove]) -> Vec<ChessMove> {
        let mut res = vec![];
        let mut buf = vec![];
        for mv in mvs {
            if let Some(mv) = self.algebraic_internal(*mv, &mut buf) {
                res.push(mv);
            } else {
                break;
            }
        }
        return res;
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
        let matches = buf
            .iter()
            .filter(|m| mv == m.simplify())
            .map(|mv| *mv)
            .collect::<Vec<_>>();

        if let [mv] = &matches[..] {
            self.apply(*mv);
            Some(*mv)
        } else {
            None
        }
    }

    #[cfg(test)]
    fn algebraic_internal(
        &mut self,
        mv: AlgebraicMove,
        buf: &mut Vec<ChessMove>,
    ) -> Option<ChessMove> {
        buf.clear();
        self.moves(buf);
        let matches = buf
            .iter()
            .filter(|m| mv.matches(**m))
            .map(|mv| *mv)
            .collect::<Vec<_>>();

        if let [mv] = &matches[..] {
            self.apply(*mv);
            Some(*mv)
        } else {
            return None;
        }
    }

    #[inline]
    fn apply_no_metadata(&mut self, mv: ChessMove) {
        let cd = self.metadata.castling_details;
        let (act, pas) = self.active_passive_mut(mv.cpc.color());
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
        if let Some(sp) = mv.spc {
            match sp {
                SpecialMove::CastlingWestward => {
                    castling_move(self, castling.westward, mv.cpc.color());
                }
                SpecialMove::CastlingEastward => {
                    castling_move(self, castling.eastward, mv.cpc.color());
                }
                SpecialMove::Promotion(p) => {
                    self.pawns ^= mv.pmv.from.bit();
                    *self.piece(p) ^= mv.pmv.to.bit();
                    self.total ^= mv.pmv.bits();
                }
                _ => {}
            }
        } else {
            *self.piece(mv.cpc.piece()) ^= mv.pmv.bits();
            self.total ^= mv.pmv.bits();
        }

        #[inline]
        fn castling_move(_self: &mut HalfBitBoard, cd: CastlingDetail, c: Color) {
            let cmv = cd.reify(c);
            _self.kings ^= cmv.king_move.bits();
            _self.rooks ^= cmv.rook_move.bits();
            _self.total ^= cmv.king_move.bits() ^ cmv.rook_move.bits();
        }
    }

    #[inline]
    pub fn apply_passive(&mut self, mv: ChessMove) {
        if let Some(sq) = mv.cap
            && let Some(pm) = self.piece_for(sq)
        {
            *pm ^= sq.bit();
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

    pub fn piece_for(&mut self, sq: Square) -> Option<&mut BoardMask> {
        self.at(sq).map(|p| self.piece(p))
    }
}

impl Metadata {
    #[inline]
    pub fn apply(&mut self, mv: ChessMove) {
        self.castling_rights = mv.castling_change(self.castling_details);
        self.en_passant = mv.ep_opening();
        self.to_move = mv.cpc.color().opposite();
        if mv.cpc.color() == Color::Black {
            self.turn += 1;
        }
        if mv.irreversible() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
    }

    #[inline]
    fn unapply(&mut self, mv: ChessMove) {
        self.en_passant = mv.epc;
        self.castling_rights = mv.cr;
        self.to_move = mv.cpc.color();
        if mv.cpc.color() == Color::Black {
            self.turn -= 1;
        }
        self.halfmove_clock = mv.hmc;
    }
}
