use crate::{
    bits::{
        self, Bits, BoardMask, bit,
        board::{BitBoard, HalfBitBoard},
        jumps::{KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE},
        mask, show_mask, slide_move_attacks,
        slides::{
            BLACK_PAWN_MOVES, RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH,
            RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST, WHITE_PAWN_MOVES,
        },
    },
    model::{
        BoardRank, ChessPiece, Color, ColoredChessPiece, Square,
        castling::{self, CastlingDetail, CastlingDetails, CastlingMove, CastlingRights},
        metadata::Metadata,
        moves::{self, ChessMove, PseudoMove, SpecialMove},
    },
};

impl BitBoard {
    pub fn moves(&self, res: &mut Vec<ChessMove>) {
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
    for from in Bits(friendly.knights) {
        for dst in Bits(
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
    for from in Bits(friendly.rooks) {
        let attacks = slide_move_attacks(
            RAYS_SOUTH.at(from),
            RAYS_NORTH.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        ) | slide_move_attacks(
            RAYS_WEST.at(from),
            RAYS_EAST.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        );

        let mask = attacks
            & !{
                let this = &friendly;
                this.total
            };

        for dst in Bits(mask) {
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
    for from in Bits(friendly.bishops) {
        let attacks = slide_move_attacks(
            RAYS_SOUTHWEST.at(from),
            RAYS_NORTHEAST.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        ) | slide_move_attacks(
            RAYS_SOUTHEAST.at(from),
            RAYS_NORTHWEST.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        );

        let mask = attacks
            & !{
                let this = &friendly;
                this.total
            };

        for dst in Bits(mask) {
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
    for from in Bits(friendly.queens) {
        let attacks = slide_move_attacks(
            RAYS_SOUTH.at(from),
            RAYS_NORTH.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        ) | slide_move_attacks(
            RAYS_WEST.at(from),
            RAYS_EAST.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        ) | slide_move_attacks(
            RAYS_SOUTHWEST.at(from),
            RAYS_NORTHEAST.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        ) | slide_move_attacks(
            RAYS_SOUTHEAST.at(from),
            RAYS_NORTHWEST.at(from),
            ({
                let this = &friendly;
                this.total
            }) | {
                let this = &enemy;
                this.total
            },
        );

        let mask = attacks
            & !{
                let this = &friendly;
                this.total
            };

        for dst in Bits(mask) {
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
    for from in Bits(friendly.pawns) {
        let mask = match metadata.to_move {
            Color::White => slide_move_attacks(
                BoardMask::MIN,
                WHITE_PAWN_MOVES.at(from),
                ({
                    let this = &friendly;
                    this.total
                }) | {
                    let this = &enemy;
                    this.total
                },
            ),
            Color::Black => slide_move_attacks(
                WHITE_PAWN_MOVES.at(from.swap()).swap_bytes(),
                BoardMask::MIN,
                ({
                    let this = &friendly;
                    this.total
                }) | {
                    let this = &enemy;
                    this.total
                },
            ),
        } & !(({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        });

        for dst in Bits(mask) {
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
    for from in Bits(friendly.pawns) {
        let mask = match metadata.to_move {
            Color::White => WHITE_PAWN_CAPTURE.at(from),
            Color::Black => WHITE_PAWN_CAPTURE.at(from.swap()).swap_bytes(),
        } & (({
            let this = &enemy;
            this.total
        }) | bit(metadata.en_passant));

        for dst in Bits(mask) {
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

    for from in Bits(friendly.kings) {
        for dst in Bits(
            KING_MOVES.at(from)
                & !static_threats
                & !{
                    let this = &friendly;
                    this.total
                },
        ) {
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

    let hypothetical_threat = enemy.attacks_after_enemy_move(
        metadata.to_move.opposite(),
        friendly.total,
        mv,
        cap_sq,
        cap_p,
    );

    let kings = if piece == ChessPiece::King {
        friendly.kings ^ mv.bits()
    } else {
        friendly.kings
    };

    if (hypothetical_threat & kings) == 0 {
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

    let hypothetical_threat = enemy.attacks_after_enemy_move(
        metadata.to_move.opposite(),
        friendly.total,
        mv,
        cap_sq,
        cap_p,
    );

    if (hypothetical_threat & friendly.kings) == 0 {
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
