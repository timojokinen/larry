use crate::util::Color;
use bitflags::bitflags;

bitflags! {
    pub struct PieceTypes: u8 {
        const PAWN          = 0b001;
        const KNIGHT        = 0b010;
        const BISHOP        = 0b011;
        const ROOK          = 0b100;
        const QUEEN         = 0b101;
        const KING          = 0b110;
    }
}

pub struct Piece(u8);

impl Piece {
    const WHITE_PAWN: u8 = 0;

    pub fn new(color: Color, piece_type: PieceTypes) -> Self {
        Piece((color as u8) << 4 | piece_type.bits())
    }

    pub fn get_color(&self) -> Color {
        match self.0 >> 4 {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!(),
        }
    }
}
