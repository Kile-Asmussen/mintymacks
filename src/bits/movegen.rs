use crate::{
    bits::{
        BoardMask, Squares,
        attacks::superpiece_attacks,
        board::{BitBoard, HalfBitBoard},
        fills::{
            black_pawn_attack_fill, black_pawn_move_fill, white_pawn_attack_fill,
            white_pawn_move_fill,
        },
        jumps::{KING_MOVES, KNIGHT_MOVES},
        one_bit,
        slides::{
            obstruction_difference, simple_diagonal_attack, simple_omnidirectional_attack,
            simple_orthogonal_attack,
        },
    },
    model::{
        BoardRank, ChessPiece, Color, Square,
        castling::CastlingDetail,
        metadata::Metadata,
        moves::{ChessMove, PseudoMove, SpecialMove},
    },
};

impl BitBoard {
    pub fn moves(&self, res: &mut Vec<ChessMove>) {
        let (act, pas) = self.active_passive(self.metadata.to_move);
        legal_moves(act, pas, self.metadata, res);
    }
}

pub fn legal_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
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
    res: &mut Vec<ChessMove>,
) {
    for from in Squares(friendly.knights) {
        for dst in Squares(
            KNIGHT_MOVES.at(from)
                & !{
                    let this = &friendly;
                    this.total
                },
        ) {
            encode_piece_move(
                from.to(dst),
                ChessPiece::Knight,
                friendly,
                enemy,
                metadata,
                res,
            )
        }
    }
}

#[inline]
pub fn rook_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    for from in Squares(friendly.rooks) {
        let attacks = simple_orthogonal_attack(from, friendly.total | enemy.total);

        let mask = attacks & !friendly.total;

        for dst in Squares(mask) {
            encode_piece_move(
                from.to(dst),
                ChessPiece::Rook,
                friendly,
                enemy,
                metadata,
                res,
            );
        }
    }
}

#[inline]
pub fn bishop_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    for from in Squares(friendly.bishops) {
        let attacks = simple_diagonal_attack(from, friendly.total | enemy.total);

        let mask = attacks & !friendly.total;

        for dst in Squares(mask) {
            encode_piece_move(
                from.to(dst),
                ChessPiece::Bishop,
                friendly,
                enemy,
                metadata,
                res,
            );
        }
    }
}

#[inline]
pub fn queen_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    for from in Squares(friendly.queens) {
        let attacks = simple_omnidirectional_attack(from, friendly.total | enemy.total);

        let mask = attacks & !friendly.total;

        for dst in Squares(mask) {
            encode_piece_move(
                from.to(dst),
                ChessPiece::Queen,
                friendly,
                enemy,
                metadata,
                res,
            );
        }
    }
}

#[inline]
pub fn pawn_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    let move_fill = match metadata.to_move {
        Color::White => white_pawn_move_fill,
        Color::Black => black_pawn_move_fill,
    };

    let empty = !(friendly.total | enemy.total);

    for from in Squares(friendly.pawns) {
        let mask = move_fill(from.bit(), empty);

        for dst in Squares(mask) {
            let mv = from.to(dst);

            encode_pawn_move(mv, None, friendly, enemy, metadata, res);
        }
    }
}

#[inline]
pub fn pawn_captures(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    let attack_fill = match metadata.to_move {
        Color::White => white_pawn_attack_fill,
        Color::Black => black_pawn_attack_fill,
    };

    for from in Squares(friendly.pawns) {
        let mask = attack_fill(from.bit()) & (enemy.total | one_bit(metadata.en_passant));

        for dst in Squares(mask) {
            let mv = from.to(dst);
            let mut cap = if enemy.total & mv.to.bit() != 0 {
                Some(mv.to)
            } else {
                None
            };

            if cap == None {
                cap = metadata
                    .en_passant
                    .and_then(|sq| Square::new(sq.ix() - 8 * (metadata.to_move as i8)));
            }

            encode_pawn_move(mv, cap, friendly, enemy, metadata, res);
        }
    }
}

#[inline]
pub fn king_moves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    let static_threats = enemy.attacks(metadata.to_move.opposite(), friendly.total);

    for from in Squares(friendly.kings) {
        for dst in Squares(KING_MOVES.at(from) & !static_threats & !friendly.total) {
            encode_piece_move(
                from.to(dst),
                ChessPiece::King,
                friendly,
                enemy,
                metadata,
                res,
            )
        }
    }

    if metadata.castling_rights.westward(metadata.to_move) {
        encode_castling_move(
            metadata.castling_details.westward,
            SpecialMove::CastlingWestward,
            metadata,
            static_threats,
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
            res,
        );
    }

    if metadata.castling_rights.eastward(metadata.to_move) {
        encode_castling_move(
            metadata.castling_details.eastward,
            SpecialMove::CastlingEastward,
            metadata,
            static_threats,
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
            res,
        );
    }
}

#[inline]
pub fn encode_castling_move(
    castling: CastlingDetail,
    special: SpecialMove,
    metadata: Metadata,
    static_threats: BoardMask,
    total: BoardMask,
    res: &mut Vec<ChessMove>,
) {
    let cmv = castling.reify(metadata.to_move);
    if (cmv.threat_mask & static_threats) == 0 && (cmv.clear_mask & total) == 0 {
        if metadata.castling_details.capture_own_rook {
            res.push(ChessMove {
                cpc: metadata.to_move.piece(ChessPiece::King).with_cap(None),
                pmv: cmv.king_move.from.to(cmv.rook_move.from),
                cap: None,
                hmc: metadata.halfmove_clock,
                spc: Some(special),
                cr: metadata.castling_rights,
                epc: metadata.en_passant,
            })
        } else {
            res.push(ChessMove {
                cpc: metadata.to_move.piece(ChessPiece::King).with_cap(None),
                pmv: cmv.king_move,
                cap: None,
                hmc: metadata.halfmove_clock,
                spc: Some(special),
                cr: metadata.castling_rights,
                epc: metadata.en_passant,
            })
        }
    }
}

#[inline]
pub fn encode_piece_move(
    mv: PseudoMove,
    piece: ChessPiece,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    let (cap_sq, cap_p) = if enemy.total & mv.to.bit() != 0 {
        (Some(mv.to), enemy.at(mv.to))
    } else {
        (None, None)
    };

    let kings = if piece == ChessPiece::King {
        friendly.kings ^ mv.bits()
    } else {
        friendly.kings
    };

    let hypothetical_check = enemy.checks_after_enemy_move(
        metadata.to_move.opposite(),
        friendly.total,
        mv,
        cap_sq,
        cap_p,
        kings,
    );

    if !hypothetical_check {
        res.push(ChessMove {
            cpc: metadata.to_move.piece(piece).with_cap(cap_p),
            pmv: mv,
            cap: cap_sq,
            hmc: metadata.halfmove_clock,
            spc: None,
            cr: metadata.castling_rights,
            epc: metadata.en_passant,
        });
    }
}

#[inline]
pub fn encode_pawn_move(
    mv: PseudoMove,
    cap_sq: Option<Square>,
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
    res: &mut Vec<ChessMove>,
) {
    let cap_p = if let Some(sq) = cap_sq {
        enemy.at(sq)
    } else {
        None
    };

    let hypothetical_check = enemy.checks_after_enemy_move(
        metadata.to_move.opposite(),
        friendly.total,
        mv,
        cap_sq,
        cap_p,
        friendly.kings,
    );

    if !hypothetical_check {
        let promotions = if let BoardRank::_1 | BoardRank::_8 = mv.to.file_rank().1 {
            &[ChessPiece::Knight, ChessPiece::Bishop, ChessPiece::Rook, ChessPiece::Queen]
                .map(|p| Some(SpecialMove::Promotion(p)))[..]
        } else {
            &[None][..]
        };

        for special in promotions {
            let special = *special;
            res.push(ChessMove {
                cpc: metadata.to_move.piece(ChessPiece::Pawn).with_cap(cap_p),
                pmv: mv,
                cap: cap_sq,
                hmc: metadata.halfmove_clock,
                spc: special,
                cr: metadata.castling_rights,
                epc: metadata.en_passant,
            });
        }
    }
}
