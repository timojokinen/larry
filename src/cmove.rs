use bitflags::bitflags;

bitflags! {
    pub struct MoveFlags: u16 {
        const QUIET = 0;
        const DOUBLE_PAWN_PUSH = 0b0001;
        const KING_CASTLE = 0b0010;
        const QUEEN_CASTLE = 0b0011;
        const CAPTURE = 0b0100;
        const EP_CAPTURE = 0b0101;
        const KNIGHT_PROM = 0b1000;
        const BISHOP_PROM = 0b1001;
        const ROOK_PROM = 0b1010;
        const QUEEN_PROM = 0b1011;
        const KNIGHT_PROM_CAPTURE = 0b1100;
        const BISHOP_PROM_CAPTURE = 0b1101;
        const ROOK_PROM_CAPTURE = 0b1110;
        const QUEEN_PROM_CAPTURE = 0b1111;
    }
}

#[derive(Debug)]
pub struct Move(u16);

impl Move {
    pub fn new(from: u16, to: u16, flags: MoveFlags) -> Self {
        Move((from) | (to << 6) | (flags.bits() << 12))
    }

    pub fn get_from(&self) -> u8 {
        (self.0 & 0x3F) as u8
    }

    pub fn get_to(&self) -> u8 {
        ((self.0 >> 6) & 0x3F) as u8
    }

    pub fn get_flags(&self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }
}
