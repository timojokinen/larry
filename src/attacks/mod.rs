use crate::util::{Bitboard, Color};

use self::{
    attacks::{king_att, knight_att, pawn_att},
    magic_numbers::{calculate_hash_index, init_magic, Magic, SLIDER_TABLE_SIZE},
};

pub mod attacks;
pub mod magic_numbers;

pub static mut SLIDER_ATTACKS: [Bitboard; SLIDER_TABLE_SIZE] = [0; SLIDER_TABLE_SIZE];

pub static mut BISHOP_TABLE: [Magic; 64] = [Magic {
    mask: 0,
    magic: 0,
    shift: 0,
    offset: 0,
}; 64];
pub static mut ROOK_TABLE: [Magic; 64] = [Magic {
    mask: 0,
    magic: 0,
    shift: 0,
    offset: 0,
}; 64];
pub static mut KNIGHT_TABLE: [Bitboard; 64] = [0; 64];
pub static mut KING_TABLE: [Bitboard; 64] = [0; 64];
pub static mut PAWN_WHITE_TABLE: [Bitboard; 64] = [0; 64];
pub static mut PAWN_BLACK_TABLE: [Bitboard; 64] = [0; 64];

pub fn init_tables() {
    let mut offset = 0;

    for idx in 0..64 {
        let magic = unsafe { init_magic(idx, true, &mut offset, &mut SLIDER_ATTACKS) };
        unsafe { BISHOP_TABLE[idx as usize] = magic };

        let magic = unsafe { init_magic(idx, false, &mut offset, &mut SLIDER_ATTACKS) };
        unsafe { ROOK_TABLE[idx as usize] = magic };

        unsafe { KNIGHT_TABLE[idx as usize] = knight_att(idx) }
        unsafe { KING_TABLE[idx as usize] = king_att(idx) }
        unsafe { PAWN_WHITE_TABLE[idx as usize] = pawn_att(idx, Color::White) }
        unsafe { PAWN_BLACK_TABLE[idx as usize] = pawn_att(idx, Color::Black) }
    }
}

pub fn lookup_slider_att(square: u8, occupancy: Bitboard, bishop: bool) -> Bitboard {
    if square > 63 {
        return 0;
    }
    let table = unsafe {
        if bishop {
            BISHOP_TABLE
        } else {
            ROOK_TABLE
        }
    };

    let Magic {
        magic,
        mask,
        shift,
        offset,
    } = table[square as usize];

    let hash_index = calculate_hash_index(magic, occupancy & mask, shift);
    unsafe { SLIDER_ATTACKS[offset + hash_index] }
}

pub fn lookup_bishop_att(square: u8, occupancy: Bitboard) -> Bitboard {
    lookup_slider_att(square, occupancy, true)
}

pub fn lookup_rook_att(square: u8, occupancy: Bitboard) -> Bitboard {
    lookup_slider_att(square, occupancy, false)
}

pub fn lookup_queen_att(square: u8, occupancy: Bitboard) -> Bitboard {
    lookup_bishop_att(square, occupancy) | lookup_rook_att(square, occupancy)
}

pub fn lookup_king_att(square: u8) -> Bitboard {
    unsafe { KING_TABLE[square as usize] }
}

pub fn lookup_knight_att(square: u8) -> Bitboard {
    unsafe { KNIGHT_TABLE[square as usize] }
}

pub fn lookup_pawn_att(square: u8, color: Color) -> Bitboard {
    let table = unsafe {
        match color {
            Color::Black => PAWN_BLACK_TABLE,
            Color::White => PAWN_WHITE_TABLE,
        }
    };
    table[square as usize]
}
