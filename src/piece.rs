use crate::util::Color;
use bitflags::bitflags;

bitflags! {
    pub struct PieceTypes: u8 {
        const PAWN          = 0b000;
        const KNIGHT        = 0b001;
        const BISHOP        = 0b010;
        const ROOK          = 0b011;
        const QUEEN         = 0b100;
        const KING          = 0b101;
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Piece(u8);

impl Piece {
    pub const PIECE_CHARS: [char; 12] =
        ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

    pub fn new(color: Color, piece_type: PieceTypes) -> Self {
        Piece((color as u8) << 3 | piece_type.bits())
    }

    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'P' => Some(Piece::new(Color::White, PieceTypes::PAWN)),
            'N' => Some(Piece::new(Color::White, PieceTypes::KNIGHT)),
            'B' => Some(Piece::new(Color::White, PieceTypes::BISHOP)),
            'R' => Some(Piece::new(Color::White, PieceTypes::ROOK)),
            'Q' => Some(Piece::new(Color::White, PieceTypes::QUEEN)),
            'K' => Some(Piece::new(Color::White, PieceTypes::KING)),
            'p' => Some(Piece::new(Color::Black, PieceTypes::PAWN)),
            'n' => Some(Piece::new(Color::Black, PieceTypes::KNIGHT)),
            'b' => Some(Piece::new(Color::Black, PieceTypes::BISHOP)),
            'r' => Some(Piece::new(Color::Black, PieceTypes::ROOK)),
            'q' => Some(Piece::new(Color::Black, PieceTypes::QUEEN)),
            'k' => Some(Piece::new(Color::Black, PieceTypes::KING)),
            _ => None,
        }
    }

    pub fn get_color(&self) -> Color {
        match self.0 >> 3 {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!(),
        }
    }

    pub fn get_index(&self) -> usize {
        let piece_type = self.0 & 0b111;
        let color = self.get_color();

        match color {
            Color::White => piece_type.into(),
            Color::Black => (piece_type + 6).into(),
        }
    }
}
