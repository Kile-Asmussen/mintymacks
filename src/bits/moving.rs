use crate::{
    bits::{
        Mask,
        board::{BitBoard, HalfBitBoard},
    },
    model::{
        Color, ColorPiece, Piece, Rank, Square,
        castling::{self, CastlingDetail, CastlingDetails, CastlingMove, CastlingRights},
        metadata::{self, Metadata},
        moves::{Move, Special},
    },
};

impl BitBoard {
    pub fn apply(&mut self, mv: Move) {
        self.apply_no_metadata(mv);
        self.metadata.apply(mv);
    }

    pub fn unapply(&mut self, mv: Move) {
        self.apply_no_metadata(mv);
        self.metadata.unapply(mv);
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
        use ColorPiece::*;
        self.castling_rights = match mv.piece {
            WhiteKing | BlackKing => self.castling_rights.move_king(mv.piece.color()),
            WhiteRook => move_rook(
                mv.mv.from,
                Color::White,
                self.castling_details,
                self.castling_rights,
            ),
            BlackRook => move_rook(
                mv.mv.from,
                Color::Black,
                self.castling_details,
                self.castling_rights,
            ),
            _ => self.castling_rights,
        };

        self.castling_rights = if let Some((Piece::Rook, sq)) = mv.cap {
            move_rook(
                sq,
                mv.piece.color().opposite(),
                self.castling_details,
                self.castling_rights,
            )
        } else {
            self.castling_rights
        };

        self.en_passant = mv.ep_opening();
        self.to_move = mv.piece.color().opposite();

        fn move_rook(
            from: Square,
            color: Color,
            details: CastlingDetails,
            rights: CastlingRights,
        ) -> CastlingRights {
            if from == details.eastward.rook_from.by(color.rank()) {
                rights.move_east_rook(color)
            } else if from == details.westward.rook_from.by(color.rank()) {
                rights.move_west_rook(color)
            } else {
                rights
            }
        }
    }

    fn unapply(&mut self, mv: Move) {
        self.en_passant = mv.epc;
        self.castling_rights = mv.rights;
        self.to_move = mv.piece.color();
    }
}
