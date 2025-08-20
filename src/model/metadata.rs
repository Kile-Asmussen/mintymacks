use crate::model::{
    Color, Square,
    castling::{CastlingDetails, CastlingRights},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metadata {
    pub to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub castling_details: CastlingDetails,
}
