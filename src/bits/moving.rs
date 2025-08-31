use crate::{
    bits::{
        Mask,
        board::{BitBoard, HalfBitBoard},
    },
    model::{
        Color, ColorPiece, Piece, Rank, Square,
        castling::{self, CastlingDetail, CastlingDetails, CastlingMove, CastlingRights},
        metadata::{self, Metadata},
        moves::{Move, PseudoMove, Special},
    },
};

impl BitBoard {
    /// Calling this method on a Move value that
    /// does not come from the BitBoard::moves method
    /// is unspecified behavior
    pub fn apply(&mut self, mv: Move) {
        self.apply_no_metadata(mv);
        self.metadata.apply(mv);
    }

    /// Calling this method with a Move value that was
    /// not used with the BitBoard::apply method immediately
    /// before this call, is unspecified behavior
    pub fn unapply(&mut self, mv: Move) {
        self.apply_no_metadata(mv);
        self.metadata.unapply(mv);
    }

    pub fn make_move(&mut self, mv: (PseudoMove, Option<Piece>)) -> Option<Move> {
        let mut buf = vec![];
        self.make_move_internal(mv, &mut buf)
    }

    pub fn make_moves(&mut self, mvs: &[(PseudoMove, Option<Piece>)]) -> Vec<Move> {
        let mut res = vec![];
        let mut buf = vec![];
        for mv in mvs {
            if let Some(mv) = self.make_move_internal(*mv, &mut buf) {
                res.push(mv);
            } else {
                break;
            }
        }
        return res;
    }

    fn make_move_internal(
        &mut self,
        mv: (PseudoMove, Option<Piece>),
        buf: &mut Vec<Move>,
    ) -> Option<Move> {
        buf.clear();
        self.moves(buf);
        if let Some(mv) = buf.iter().find(|m| m.matches(mv.0, mv.1)) {
            self.apply(*mv);
            Some(*mv)
        } else {
            None
        }
    }

    fn apply_no_metadata(&mut self, mv: Move) {
        let cd = self.metadata.castling_details;
        let (act, pas) = self.active_passive(mv.piece.color());
        act.apply_active(cd, mv);
        pas.apply_passive(mv);
    }

    fn active_passive(&mut self, color: Color) -> (&mut HalfBitBoard, &mut HalfBitBoard) {
        match color {
            Color::White => (&mut self.white, &mut self.black),
            Color::Black => (&mut self.black, &mut self.white),
        }
    }
}

impl HalfBitBoard {
    #[inline]
    pub fn apply_active(&mut self, castling: CastlingDetails, mv: Move) {
        if let Some(sp) = mv.special {
            match sp {
                Special::CastlingEastward => {
                    castling_move(self, castling.westward, mv.piece.color())
                }
                Special::CastlingWestward => {
                    castling_move(self, castling.eastward, mv.piece.color())
                }
                Special::Promotion(p) => {
                    self.pawns ^= mv.mv.from.bit();
                    *self.piece(p) ^= mv.mv.to.bit();
                }
                _ => {}
            }
        } else {
            *self.piece(mv.piece.piece()) ^= mv.mv.bits()
        }

        fn castling_move(_self: &mut HalfBitBoard, cd: CastlingDetail, c: Color) {
            let cmv = cd.reify(c);
            _self.kings ^= cmv.king_move.bits();
            _self.rooks ^= cmv.rook_move.bits();
        }
    }

    #[inline]
    pub fn apply_passive(&mut self, mv: Move) {
        if let Some((p, sq)) = mv.cap {
            *self.piece(p) ^= sq.bit();
        }
    }

    pub fn piece(&mut self, p: Piece) -> &mut Mask {
        match p {
            Piece::Pawn => &mut self.pawns,
            Piece::Knight => &mut self.knights,
            Piece::Bishop => &mut self.bishops,
            Piece::Rook => &mut self.rooks,
            Piece::Queen => &mut self.queens,
            Piece::King => &mut self.kings,
        }
    }
}

impl Metadata {
    #[inline]
    pub fn apply(&mut self, mv: Move) {
        self.castling_rights = mv.castling_change(self.castling_details);
        self.en_passant = mv.ep_opening();
        self.to_move = mv.piece.color().opposite();
    }

    fn unapply(&mut self, mv: Move) {
        self.en_passant = mv.epc;
        self.castling_rights = mv.rights;
        self.to_move = mv.piece.color();
    }
}
