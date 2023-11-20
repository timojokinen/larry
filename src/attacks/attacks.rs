use crate::{util::*, Color};

/// Algorithm to calculate sliding piece attacks with given occupancy.
/// Formula: (((o&m)-2s) ^ ((o&m)`-2s`)`)&m
/// https://www.chessprogramming.org/Hyperbola_Quintessence
pub fn hyperbola_quintessence(piece: Bitboard, occupancy: Bitboard, mask: Bitboard) -> Bitboard {
    let occupancy_mask = occupancy & mask;
    let forward = occupancy_mask.wrapping_sub(piece.wrapping_mul(2));
    let reverse = occupancy_mask
        .reverse_bits()
        .wrapping_sub(piece.reverse_bits().wrapping_mul(2))
        .reverse_bits();

    (forward ^ reverse) & mask
}

pub fn bishop_att(square: u8, occupancy: Bitboard) -> Bitboard {
    let piece = 1 << square;
    hyperbola_quintessence(piece, occupancy, mask_diag(square))
        | hyperbola_quintessence(piece, occupancy, mask_anti_diag(square))
}

pub fn rook_att(square: u8, occupancy: Bitboard) -> Bitboard {
    let piece = 1 << square;
    hyperbola_quintessence(piece, occupancy, mask_file(square))
        ^ hyperbola_quintessence(piece, occupancy, mask_rank(square))
}

pub fn pawn_east_att(square: u8, color: Color) -> Bitboard {
    let occ = 1u64 << square;
    match color {
        Color::White => (occ << 9) & !mask_file(0),
        Color::Black => (occ >> 9) & !mask_file(63),
    }
}

pub fn pawn_west_att(square: u8, color: Color) -> Bitboard {
    let occ = 1u64 << square;
    match color {
        Color::White => (occ << 7) & !mask_file(63),
        Color::Black => (occ >> 7) & !mask_file(0),
    }
}

pub fn pawn_att(square: u8, color: Color) -> Bitboard {
    pawn_east_att(square, color) | pawn_west_att(square, color)
}

pub fn king_att(square: u8) -> Bitboard {
    let occupancy = 1u64 << square;
    let mut att = 0u64;

    let file = sq_to_file(square);
    let rank = sq_to_rank(square);

    att |= if file > 0 && rank < 7 {
        occupancy << 7
    } else {
        0
    };
    att |= if rank < 7 { occupancy << 8 } else { 0 };
    att |= if rank < 7 && file < 7 {
        occupancy << 9
    } else {
        0
    };
    att |= if file < 7 { occupancy << 1 } else { 0 };
    att |= if file > 0 { occupancy >> 1 } else { 0 };
    att |= if file < 7 && rank > 0 {
        occupancy >> 7
    } else {
        0
    };
    att |= if rank > 0 { occupancy >> 8 } else { 0 };
    att |= if rank > 0 && file > 0 {
        occupancy >> 9
    } else {
        0
    };

    att
}

pub fn knight_att(square: u8) -> Bitboard {
    let occupancy = 1u64 << square;

    let mut att = 0u64;

    let file = sq_to_file(square);
    let rank = sq_to_rank(square);

    att |= if file > 0 && rank < 6 {
        occupancy << 15
    } else {
        0
    };
    att |= if file < 7 && rank < 6 {
        occupancy << 17
    } else {
        0
    };

    att |= if file > 1 && rank < 7 {
        occupancy << 6
    } else {
        0
    };
    att |= if file < 6 && rank < 7 {
        occupancy << 10
    } else {
        0
    };

    att |= if rank > 0 && file < 6 {
        occupancy >> 6
    } else {
        0
    };
    att |= if rank > 0 && file > 1 {
        occupancy >> 10
    } else {
        0
    };

    att |= if rank > 1 && file < 7 {
        occupancy >> 15
    } else {
        0
    };
    att |= if rank > 1 && file > 0 {
        occupancy >> 17
    } else {
        0
    };

    att
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn use_hyperbola_quintessence_correctly() {
        let bb = hyperbola_quintessence(0b00010000, 0b11010011, 0b11111111);
        assert!(bb == 0b01101110);

        let bb = hyperbola_quintessence(0b00000001, 0b01000000, 0b111111111);
        assert!(bb == 0b01111110);

        let bb = hyperbola_quintessence(1 << 18, 1 << 9 | 1 << 45, mask_diag(18));
        print_bitboard(bb);
    }
}
