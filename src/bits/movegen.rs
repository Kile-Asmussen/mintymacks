use crate::{
    bits::{
        Bits, Mask, bit,
        board::{BitBoard, HalfBitBoard},
        jumps::{BLACK_PAWN_CAPTURE, KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE},
        slide_move_stop_negative, slide_move_stop_positive,
        slides::{
            BLACK_PAWN_MOVES, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH,
            RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST, WHITE_PAWN_MOVES,
        },
    },
    model::{
        Color, ColorPiece, Piece, Rank, Square,
        castling::{self, CastlingDetail, CastlingDetails, CastlingMove, CastlingRights},
        metadata::Metadata,
        moves::{self, Move, PseudoMove, Special},
    },
};

impl BitBoard {
    pub fn moves(&self, res: &mut Vec<Move>) {
        match self.metadata.to_move {
            Color::White => legal_moves(&self.white, &self.black, self.metadata, res),
            Color::Black => legal_moves(&self.black, &self.white, self.metadata, res),
        }
    }
}

pub fn legal_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    pawn_moves(friendly, enemy, metadata, res);
    pawn_captures(friendly, enemy, metadata, res);
    knight_moves(friendly, enemy, metadata, res);
    bishop_moves(friendly, enemy, metadata, res);
    rook_moves(friendly, enemy, metadata, res);
    queen_moves(friendly, enemy, metadata, res);
    king_moves(friendly, enemy, metadata, res);
}

#[inline]
pub fn knight_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.knights) {
        for dst in Bits(KNIGHT_MOVES.at(from) & !friendly.total()) {
            generic_move(from.to(dst), Piece::Knight, friendly, enemy, metadata, res)
        }
    }
}

#[inline]
pub fn rook_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.rooks) {
        let mask = slide_move_stop_positive(RAYS_NORTH.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_EAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_WEST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTH.at(from), friendly.total(), enemy.total());

        for dst in Bits(mask) {
            generic_move(from.to(dst), Piece::Rook, friendly, enemy, metadata, res);
        }
    }
}

#[inline]
pub fn bishop_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.bishops) {
        let mask =
            slide_move_stop_positive(RAYS_NORTHEAST.at(from), friendly.total(), enemy.total())
                | slide_move_stop_positive(
                    RAYS_NORTHWEST.at(from),
                    friendly.total(),
                    enemy.total(),
                )
                | slide_move_stop_negative(
                    RAYS_SOUTHEAST.at(from),
                    friendly.total(),
                    enemy.total(),
                )
                | slide_move_stop_negative(
                    RAYS_SOUTHWEST.at(from),
                    friendly.total(),
                    enemy.total(),
                );

        for dst in Bits(mask) {
            generic_move(from.to(dst), Piece::Rook, friendly, enemy, metadata, res);
        }
    }
}

#[inline]
pub fn queen_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.queens) {
        let mask = slide_move_stop_positive(RAYS_NORTH.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_EAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_WEST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTH.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_NORTHEAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_NORTHWEST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTHEAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTHWEST.at(from), friendly.total(), enemy.total());

        for dst in Bits(mask) {
            generic_move(from.to(dst), Piece::Queen, friendly, enemy, metadata, res);
        }
    }
}

#[inline]
pub fn pawn_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.pawns) {
        let mask = match metadata.to_move {
            Color::White => slide_move_stop_positive(
                WHITE_PAWN_MOVES.at(from),
                friendly.total() | enemy.total(),
                Mask::MIN,
            ),
            Color::Black => slide_move_stop_negative(
                BLACK_PAWN_MOVES.at(from),
                friendly.total() | enemy.total(),
                Mask::MIN,
            ),
        };

        for dst in Bits(mask) {
            let mv = from.to(dst);
            let hypothetical_threat = enemy.threats(
                metadata.to_move.opposite(),
                friendly.total(),
                Some(mv),
                None,
            );

            if (hypothetical_threat & friendly.kings) != 0 {
                return;
            }

            handle_pawn_promotion(mv, None, metadata, res);
        }
    }
}

#[inline]
pub fn pawn_captures(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.pawns) {
        let mask = match metadata.to_move {
            Color::White => {
                WHITE_PAWN_CAPTURE.at(from) & (enemy.total() | bit(metadata.en_passant))
            }
            Color::Black => {
                BLACK_PAWN_CAPTURE.at(from) & (enemy.total() | bit(metadata.en_passant))
            }
        };

        for dst in Bits(mask) {
            let mv = from.to(dst);
            let mut cap = enemy.at(dst).map(|p| (p, dst));

            if cap == None {
                cap = metadata
                    .en_passant
                    .and_then(|sq| Square::new(sq.ix() + 8 * (metadata.to_move as i8)))
                    .map(|sq| (Piece::Pawn, sq))
            }

            let hypothetical_threat =
                enemy.threats(metadata.to_move.opposite(), friendly.total(), Some(mv), cap);

            if (hypothetical_threat & friendly.kings) != 0 {
                return;
            }

            handle_pawn_promotion(mv, cap, metadata, res);
        }
    }
}

#[inline]
pub fn handle_pawn_promotion(
    mv: PseudoMove,
    cap: Option<(Piece, Square)>,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    if let 0..=7 | 56..=63 = mv.to.ix() {
        for p in [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
            res.push(Move {
                piece: metadata.to_move.piece(Piece::Pawn),
                mv,
                cap,
                special: Some(Special::Promotion(p)),
                rights: metadata.castling_rights,
                epc: metadata.en_passant,
            });
        }
    } else {
        res.push(Move {
            piece: metadata.to_move.piece(Piece::Pawn),
            mv,
            cap,
            special: None,
            rights: metadata.castling_rights,
            epc: metadata.en_passant,
        });
    }
}

#[inline]
pub fn king_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    let static_threats = enemy.threats(metadata.to_move.opposite(), friendly.total(), None, None);

    for from in Bits(friendly.kings) {
        for dst in Bits(KING_MOVES.at(from) & !static_threats & !friendly.total()) {
            generic_move(from.to(dst), Piece::King, friendly, enemy, metadata, res)
        }
    }

    if metadata.castling_rights.westward(metadata.to_move) {
        handle_castling(
            metadata.castling_details.westward,
            metadata,
            static_threats,
            friendly.total() | enemy.total(),
            res,
        );
    }

    if metadata.castling_rights.eastward(metadata.to_move) {
        handle_castling(
            metadata.castling_details.eastward,
            metadata,
            static_threats,
            friendly.total() | enemy.total(),
            res,
        );
    }
}

#[inline]
pub fn handle_castling(
    castling: CastlingDetail,
    metadata: Metadata,
    static_threats: Mask,
    total: Mask,
    res: &mut Vec<Move>,
) {
    let cmv = castling.reify(metadata.to_move);
    if (cmv.threat_mask & static_threats) == 0 && (cmv.clear_mask & total) == 0 {
        if metadata.castling_details.capture_own_rook {
            res.push(Move {
                piece: metadata.to_move.piece(Piece::King),
                mv: cmv.king_move.from.to(cmv.rook_move.from),
                cap: None,
                special: Some(Special::CastlingWestward),
                rights: metadata.castling_rights,
                epc: metadata.en_passant,
            })
        } else {
            res.push(Move {
                piece: metadata.to_move.piece(Piece::King),
                mv: cmv.king_move,
                cap: None,
                special: Some(Special::CastlingWestward),
                rights: metadata.castling_rights,
                epc: metadata.en_passant,
            })
        }
    }
}

#[inline]
pub fn generic_move(
    mv: PseudoMove,
    piece: Piece,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<Move>,
) {
    let cap = enemy.at(mv.to).map(|p| (p, mv.to));

    let hypothetical_threat =
        enemy.threats(metadata.to_move.opposite(), friendly.total(), Some(mv), cap);

    if (hypothetical_threat & friendly.kings) != 0 {
        return;
    }

    res.push(Move {
        piece: metadata.to_move.piece(piece),
        mv,
        cap,
        special: None,
        rights: metadata.castling_rights,
        epc: metadata.en_passant,
    });
}
