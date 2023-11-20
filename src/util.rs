use crate::fen::Piece;

pub type Bitboard = u64;

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    White = 0,
    Black = 1,
}

pub const RANK: u64 = 0xff;
pub const FILE: u64 = 0x0101010101010101;
pub const MAIN_DIAG: u64 = 0x8040201008040201;
pub const ANTI_DIAG: u64 = 0x0102040810204080;

pub fn coord_to_idx(file: u8, rank: u8) -> u8 {
    8 * rank + file
}

pub fn sq_to_file(square: u8) -> u8 {
    square & 7
}

pub fn sq_to_rank(square: u8) -> u8 {
    square >> 3
}

pub fn mask_rank(square: u8) -> Bitboard {
    RANK << (square & !7)
}

pub fn mask_file(square: u8) -> Bitboard {
    FILE << (square & 7)
}

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

pub fn edges(square: u8) -> Bitboard {
    ((mask_file(0) | mask_file(63)) & !mask_file(square))
        | ((mask_rank(0) | mask_rank(63)) & !mask_rank(square))
}

pub fn pop_bit(n: &mut u64, bit: u32) {
    *n &= !(1 << bit);
}

pub fn mailbox_to_bitboard(mailbox: [Option<Piece>; 64], piece: Piece) -> Bitboard {
    let mut bitboard: Bitboard = 0;

    for (idx, p) in mailbox.iter().enumerate() {
        if *p == Some(piece) {
            bitboard |= 1 << idx;
        }
    }

    bitboard
}

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
}
