use crate::{
    bits::{
        Bits, Mask, bit,
        board::HalfBitBoard,
        jumps::{BLACK_PAWN_CAPTURE, KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE},
        slide_move_stop_negative, slide_move_stop_positive,
        slides::{
            BLACK_PAWN_MOVES, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH,
            RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST, WHITE_PAWN_MOVES,
        },
    },
    model::{
        Color, ColorPiece, Piece, Rank, Square,
        castling::{CastlingDetails, CastlingMove, CastlingRights},
        moves::{self, Move, PseudoMove, Special},
    },
};

pub fn legal_moves(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    castling: CastlingDetails,
    res: &mut Vec<Move>,
) {
}

#[inline]
pub fn generic_move(
    mv: PseudoMove,
    piece: ColorPiece,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    let cap = enemy.at(mv.to).map(|p| (p, mv.to));

    let hypothetical_threat =
        enemy.threats(piece.color().opposite(), friendly.total(), Some(mv), cap);

    if (hypothetical_threat & friendly.kings) != 0 {
        return;
    }

    res.push(Move {
        piece,
        mv,
        cap,
        special: None,
        rights,
        eps,
    });
}

pub fn knight_moves(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.knights) {
        for dst in Bits(KNIGHT_MOVES.at(from) & !friendly.total()) {
            generic_move(
                from.to(dst),
                color.piece(Piece::Knight),
                friendly,
                enemy,
                rights,
                eps,
                res,
            )
        }
    }
}

pub fn rook_moves(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.rooks) {
        let mask = slide_move_stop_positive(RAYS_NORTH.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_EAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_WEST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTH.at(from), friendly.total(), enemy.total());

        for dst in Bits(mask) {
            generic_move(
                from.to(dst),
                color.piece(Piece::Rook),
                friendly,
                enemy,
                rights,
                eps,
                res,
            );
        }
    }
}

pub fn bishop_moves(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.rooks) {
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
            generic_move(
                from.to(dst),
                color.piece(Piece::Rook),
                friendly,
                enemy,
                rights,
                eps,
                res,
            );
        }
    }
}

pub fn queen_moves(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.rooks) {
        let mask = slide_move_stop_positive(RAYS_NORTH.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_EAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_WEST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTH.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_NORTHEAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_positive(RAYS_NORTHWEST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTHEAST.at(from), friendly.total(), enemy.total())
            | slide_move_stop_negative(RAYS_SOUTHWEST.at(from), friendly.total(), enemy.total());

        for dst in Bits(mask) {
            generic_move(
                from.to(dst),
                color.piece(Piece::Queen),
                friendly,
                enemy,
                rights,
                eps,
                res,
            );
        }
    }
}

pub fn pawn_move(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.pawns) {
        let mask = match color {
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
            let hypothetical_threat =
                enemy.threats(color.opposite(), friendly.total(), Some(mv), None);

            if (hypothetical_threat & friendly.kings) != 0 {
                return;
            }

            handle_pawn_promotion(color, mv, None, rights, eps, res);
        }
    }
}

pub fn pawn_capture(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    for from in Bits(friendly.pawns) {
        let mask = match color {
            Color::White => WHITE_PAWN_CAPTURE.at(from) & (enemy.total() | bit(eps)),
            Color::Black => BLACK_PAWN_CAPTURE.at(from) & (enemy.total() | bit(eps)),
        };

        for dst in Bits(mask) {
            let mv = from.to(dst);
            let mut cap = enemy.at(dst).map(|p| (p, dst));

            if cap == None {
                cap = eps
                    .and_then(|sq| Square::new(sq.ix() + 8 * (color as i8)))
                    .map(|sq| (Piece::Pawn, sq))
            }

            let hypothetical_threat =
                enemy.threats(color.opposite(), friendly.total(), Some(mv), cap);

            if (hypothetical_threat & friendly.kings) != 0 {
                return;
            }

            handle_pawn_promotion(color, mv, cap, rights, eps, res);
        }
    }
}

#[inline]
pub fn handle_pawn_promotion(
    color: Color,
    mv: PseudoMove,
    cap: Option<(Piece, Square)>,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    if let 0..=7 | 56..=63 = mv.to.ix() {
        for p in [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
            res.push(Move {
                piece: color.piece(Piece::Pawn),
                mv,
                cap,
                special: Some(Special::Promotion(p)),
                rights,
                eps,
            });
        }
    } else {
        res.push(Move {
            piece: color.piece(Piece::Pawn),
            mv,
            cap,
            special: None,
            rights,
            eps,
        });
    }
}

pub fn king_moves(
    color: Color,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    rights: CastlingRights,
    eps: Option<Square>,
    castling: CastlingDetails,
    res: &mut Vec<Move>,
) {
    let static_threats = enemy.threats(color.opposite(), friendly.total(), None, None);

    for from in Bits(friendly.kings) {
        for dst in Bits(KING_MOVES.at(from) & !static_threats & !friendly.total()) {
            generic_move(
                from.to(dst),
                color.piece(Piece::King),
                friendly,
                enemy,
                rights,
                eps,
                res,
            )
        }
    }

    if rights.westward(color) {
        handle_castling(
            color,
            castling.westward.reify(color),
            castling,
            static_threats,
            friendly.total() | enemy.total(),
            rights,
            eps,
            res,
        );
    }

    if rights.eastward(color) {
        handle_castling(
            color,
            castling.eastward.reify(color),
            castling,
            static_threats,
            friendly.total() | enemy.total(),
            rights,
            eps,
            res,
        );
    }
}

#[inline]
pub fn handle_castling(
    color: Color,
    cmv: CastlingMove,
    castling: CastlingDetails,
    static_threats: Mask,
    total: Mask,
    rights: CastlingRights,
    eps: Option<Square>,
    res: &mut Vec<Move>,
) {
    if (cmv.threat_mask & static_threats) == 0 && (cmv.clear_mask & total) == 0 {
        if castling.capture_own_rook {
            res.push(Move {
                piece: color.piece(Piece::King),
                mv: cmv.king_move.from.to(cmv.rook_move.from),
                cap: None,
                special: Some(Special::CastlingWestward),
                rights,
                eps,
            })
        } else {
            res.push(Move {
                piece: color.piece(Piece::King),
                mv: cmv.king_move,
                cap: None,
                special: Some(Special::CastlingWestward),
                rights,
                eps,
            })
        }
    }
}
