use crate::{fen::Squares, piece::Piece};

pub type Bitboard = u64;

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

/// Rank bitboards masks by index
pub const RANKS: [u64; 8] = [
    0xff,
    0xff00,
    0xff0000,
    0xff000000,
    0xff00000000,
    0xff0000000000,
    0xff000000000000,
    0xff00000000000000,
];

/// File bitboards masks by index
pub const FILES: [u64; 8] = [
    0x0101010101010101,
    0x202020202020202,
    0x404040404040404,
    0x808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

pub const MAIN_DIAG: u64 = 0x8040201008040201;
pub const ANTI_DIAG: u64 = 0x0102040810204080;

/// Translate chess board coordinates to a square index (0 - 63)
pub fn coord_to_idx(file: u8, rank: u8) -> u8 {
    8 * rank + file
}

/// Translate the index of a square to the corresponding file index (0-7)
pub fn sq_to_file(square: u8) -> u8 {
    square & 7
}

/// Translate the index of a square to the corresponding rank index (0-7)
pub fn sq_to_rank(square: u8) -> u8 {
    square >> 3
}

/// Masks the rank of a given square
pub fn mask_rank(square: u8) -> Bitboard {
    RANKS[0] << (square & !7)
}

/// Masks the file of a given square
pub fn mask_file(square: u8) -> Bitboard {
    FILES[0] << (square & 7)
}

/// Masks the diagonal of a given square
pub fn mask_diag(square: u8) -> Bitboard {
    let mut bb = MAIN_DIAG;
    let rank = sq_to_rank(square);
    let file = sq_to_file(square);
    if rank > file {
        bb <<= (rank - file) * 8;
    } else {
        bb >>= (file - rank) * 8;
    }
    bb
}

/// Masks the anti-diagonal of a given square
pub fn mask_anti_diag(square: u8) -> Bitboard {
    let mut bb: u64 = ANTI_DIAG;
    let rank = sq_to_rank(square);
    let file = sq_to_file(square);
    let delta = (rank + file) as i8 - 7;
    if delta < 0 {
        bb >>= (-delta) as u64 * 8;
    } else {
        bb <<= (delta) as u64 * 8;
    }
    bb
}

/// Returns a Bitboard that represents the edges of the board, excluding the rank and file of the given square.
pub fn edges(square: u8) -> Bitboard {
    ((mask_file(0) | mask_file(63)) & !mask_file(square))
        | ((mask_rank(0) | mask_rank(63)) & !mask_rank(square))
}

// Returns the bitboard mask of a rank on the board relative to the given color
pub fn relative_rank(rank: u8, color: Color) -> Bitboard {
    if color == Color::White {
        RANKS[rank as usize]
    } else {
        RANKS[7 - rank as usize]
    }
}

/// Creates a bitboard from a square index
pub fn bb_from_square(square: u8) -> Bitboard {
    1u64.checked_shl(square.into()).unwrap_or(0)
}

/// Unsets the bit at the given index
pub fn pop_bit(n: &mut u64, idx: u8) {
    *n &= !(1 << idx);
}

/// Translates a mailbox representation to a bitboard for a given piece
pub fn mailbox_to_bb(mailbox: Squares, piece: Piece) -> Bitboard {
    let mut bitboard: Bitboard = 0;

    for (idx, p) in mailbox.iter().enumerate() {
        if *p == Some(piece) {
            bitboard |= 1 << idx;
        }
    }

    bitboard
}

/// Checks if a square is on the board
pub fn is_on_board(square: u8) -> bool {
    square < 64
}

/// Returns the opposite color
pub fn opp(color: Color) -> Color {
    (color as usize ^ 1).into()
}

/// Iterate over all bits set in the given bitboard
pub fn enumerate_bits<F>(bb: Bitboard, mut func: F)
where
    F: FnMut(u8),
{
    let _bb: &mut Bitboard = &mut bb.clone();

    loop {
        if *_bb == 0 {
            return;
        };
        let lsb1idx = _bb.trailing_zeros() as u8;
        func(lsb1idx);
        pop_bit(_bb, lsb1idx);
    }
}

/// Formats a bitboard in a pretty way for debugging
pub fn format_bitboard(bitboard: Bitboard) -> String {
    let mut board_str = String::new();
    for row in (0..8).rev() {
        for col in 0..8 {
            let pos = row * 8 + col;

            if (bitboard >> pos) & 1 == 1 {
                board_str.push('1');
            } else {
                board_str.push('0');
            }
            board_str.push(' ');
        }
        board_str.push('\n');
    }
    board_str
}

/// Prints pretty bitboard for debugging
pub fn print_bitboard(bitboard: Bitboard) {
    println!("{}", format_bitboard(bitboard));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coord_to_idx_correctly() {
        let idx = coord_to_idx(0, 0);
        assert!(idx == 0);

        let idx = coord_to_idx(4, 3);
        assert!(idx == 28);
    }

    #[test]
    fn parse_sq_to_file_correctly() {
        let file = sq_to_file(28);
        assert!(file == 4);
    }

    #[test]
    fn parse_sq_to_rank_correctly() {
        let rank = sq_to_rank(28);
        assert!(rank == 3)
    }

    #[test]
    fn mask_rank_correctly() {
        let rank_mask = mask_rank(28);
        assert!(rank_mask == 0xff000000);

        let rank_mask = mask_rank(44);
        assert!(rank_mask == 0xff0000000000);
    }

    #[test]
    fn mask_file_correctly() {
        let file_mask = mask_file(34);
        assert!(file_mask == 0x404040404040404);

        let file_mask = mask_file(12);
        assert!(file_mask == 0x1010101010101010);
    }

    #[test]
    fn pop_bit_correctly() {
        let mut bb = 0b1111;
        pop_bit(&mut bb, 2);
        assert!(bb == 0b1011);
        pop_bit(&mut bb, 0);
        assert!(bb == 0b1010);
    }

    #[test]
    fn calculate_opponent_color_correctly() {
        assert!(opp(Color::White) == Color::Black);
        assert!(opp(Color::Black) == Color::White);
    }
}
