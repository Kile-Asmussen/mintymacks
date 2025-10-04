use crate::{bits::{bit, board::HalfBitBoard, jumps::{KING_MOVES, KNIGHT_MOVES, WHITE_PAWN_CAPTURE}, mask, slide_move_attacks, slides::{RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST, RAYS_SOUTH, RAYS_SOUTHEAST, RAYS_SOUTHWEST, RAYS_WEST, WHITE_PAWN_MOVES}, Bits, BoardMask}, model::{metadata::Metadata, Color}};

pub fn count_pseudomoves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    count_pawn_pseudomoves(friendly, enemy, metadata) +
    count_pawn_pseudocaptures(friendly, enemy, metadata) +
    count_knight_pseudomoves(friendly, enemy, metadata) +
    count_bishop_pseudomoves(friendly, enemy, metadata) +
    count_rook_pseudomoves(friendly, enemy, metadata) +
    count_queen_pseudomoves(friendly, enemy, metadata) +
    count_king_pseudomoves(friendly, enemy, metadata)
}

#[inline]
pub fn count_knight_pseudomoves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    let mut res = 0;
    for from in Bits(friendly.knights) {
        res += (KNIGHT_MOVES.at(from) & !{
            let this = &friendly;
            this.total
        }).count_ones();
    }
    res
}

#[inline]
pub fn count_king_pseudomoves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    let mut res = 0;
    for from in Bits(friendly.knights) {
        res += (KING_MOVES.at(from) & !{
            let this = &friendly;
            this.total
        }).count_ones();
    }
    res += metadata.castling_rights.get(metadata.to_move).count_ones();

    res
}

#[inline]
pub fn count_pawn_pseudomoves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    let mut res = 0;
    for from in Bits(friendly.pawns) {
        let moves = match metadata.to_move {
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

        const END_ZONE: BoardMask = mask([0xFF, 0, 0, 0, 0, 0, 0, 0xFF]);

        res += (moves & END_ZONE).count_ones() * 4;
        res += (moves & !END_ZONE).count_ones();
    }

    res
}

#[inline]
pub fn count_pawn_pseudocaptures(
friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    let mut res = 0;
    for from in Bits(friendly.pawns) {
        let moves = match metadata.to_move {
            Color::White => {
                WHITE_PAWN_CAPTURE.at(from) 
            }
            Color::Black => {
                WHITE_PAWN_CAPTURE.at(from.swap()).swap_bytes()
            }
        } & (({
            let this = &enemy;
            this.total
        }) | bit(metadata.en_passant));

        const END_ZONE: BoardMask = mask([0xFF, 0, 0, 0, 0, 0, 0, 0xFF]);

        res += (moves & END_ZONE).count_ones() * 4;
        res += (moves & !END_ZONE).count_ones();
    }

    res
}

#[inline]
pub fn count_queen_pseudomoves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    let mut res = 0;
    for from in Bits(friendly.queens) {

        let attacks = slide_move_attacks(RAYS_SOUTH.at(from), RAYS_NORTH.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        })
        | slide_move_attacks(RAYS_WEST.at(from), RAYS_EAST.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        })
        | slide_move_attacks(RAYS_SOUTHWEST.at(from), RAYS_NORTHEAST.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        })
        | slide_move_attacks(RAYS_SOUTHEAST.at(from), RAYS_NORTHWEST.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        });

        let moves = attacks & !{
            let this = &friendly;
            this.total
        };

        res += moves.count_ones();
    }

    res
}

#[inline]
pub fn count_bishop_pseudomoves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    let mut res = 0;
    for from in Bits(friendly.queens) {

        let attacks = 
        slide_move_attacks(RAYS_SOUTHWEST.at(from), RAYS_NORTHEAST.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        })
        | slide_move_attacks(RAYS_SOUTHEAST.at(from), RAYS_NORTHWEST.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        });

        let moves = attacks & !{
            let this = &friendly;
            this.total
        };

        res += moves.count_ones();
    }

    res
}

#[inline]
pub fn count_rook_pseudomoves(
    friendly: &HalfBitBoard,
    enemy: &HalfBitBoard,
    metadata: Metadata,
) -> u32 {
    let mut res = 0;
    for from in Bits(friendly.queens) {

        let attacks = slide_move_attacks(RAYS_SOUTH.at(from), RAYS_NORTH.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        })
        | slide_move_attacks(RAYS_WEST.at(from), RAYS_EAST.at(from), ({
            let this = &friendly;
            this.total
        }) | {
            let this = &enemy;
            this.total
        });

        let moves = attacks & !{
            let this = &friendly;
            this.total
        };

        res += moves.count_ones();
    }

    res
}