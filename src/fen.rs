use crate::{castling_rights::CastlingRights, util::Color};

pub struct BoardState(
    [Option<Piece>; 64],
    Color,
    CastlingRights,
    Option<u8>,
    usize,
    usize,
);

impl From<usize> for Color {
    fn from(value: usize) -> Self {
        match value {
            0 => Color::White,
            1 => Color::Black,
            _ => panic!("Invalid integer value for Color"),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Piece {
    PawnWhite = 0,
    KnightWhite = 1,
    BishopWhite = 2,
    RookWhite = 3,
    QueenWhite = 4,
    KingWhite = 5,
    PawnBlack = 6,
    KnightBlack = 7,
    BishopBlack = 8,
    RookBlack = 9,
    QueenBlack = 10,
    KingBlack = 11,
}

impl Piece {
    const PIECE_CHARS: [char; 12] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'P' => Some(Piece::PawnWhite),
            'N' => Some(Piece::KnightWhite),
            'B' => Some(Piece::BishopWhite),
            'R' => Some(Piece::RookWhite),
            'Q' => Some(Piece::QueenWhite),
            'K' => Some(Piece::KingWhite),
            'p' => Some(Piece::PawnBlack),
            'n' => Some(Piece::KnightBlack),
            'b' => Some(Piece::BishopBlack),
            'r' => Some(Piece::RookBlack),
            'q' => Some(Piece::QueenBlack),
            'k' => Some(Piece::KingBlack),
            _ => None,
        }
    }

    pub fn to_int(self) -> u8 {
        self as u8
    }
}

pub fn san_to_int(san: &str) -> Option<u8> {
    if san.len() != 2 {
        return None;
    }

    let file = san.chars().nth(0)?;
    let rank = san.chars().nth(1)?;

    let file_value = (file as u8 - b'a') as u8;
    let rank_value = rank.to_digit(10)? as u8;

    if file_value >= 8 || rank_value < 1 || rank_value > 8 {
        return None;
    }

    Some((rank_value - 1) * 8 + file_value)
}

pub fn parse_fen(fen: &str) -> Result<BoardState, &str> {
    let mut parts = fen.splitn(6, " ");

    let mut mailbox: [Option<Piece>; 64] = [None; 64];
    let mut castling_rights = CastlingRights::empty();
    let piece_placements = parts.next().unwrap();
    let ranks = piece_placements.split("/");

    for (rank_idx, rank) in ranks.enumerate() {
        let pieces = rank.chars();
        let mut file_idx = 0;
        for (_, piece) in pieces.enumerate() {
            if file_idx > 7 {
                return Err("Invalid piece placement in FEN");
            }
            match piece.to_digit(10) {
                Some(number) => {
                    if number < 1 || number > 8 {
                        return Err("Invalid piece placement in FEN");
                    }
                    file_idx += number as usize;
                }
                None => {
                    if !&['p', 'k', 'q', 'n', 'b', 'r'].contains(&piece.to_ascii_lowercase()) {
                        return Err("Invalid piece placement in FEN");
                    }
                    mailbox[(8 - 1 - rank_idx) * 8 + file_idx] = Piece::from_char(piece);
                    file_idx += 1;
                }
            }
        }
    }

    let active_color = match parts.next().unwrap().chars().nth(0).unwrap() {
        'b' => Color::Black,
        'w' => Color::White,
        _ => return Err("Invalid active color in FEN"),
    };

    let castling_rights_str = parts.next().unwrap();

    for side in castling_rights_str.chars() {
        match side {
            'Q' => castling_rights.insert(CastlingRights::WHITE_QUEEN_SIDE),
            'K' => castling_rights.insert(CastlingRights::WHITE_KING_SIDE),
            'q' => castling_rights.insert(CastlingRights::BLACK_QUEEN_SIDE),
            'k' => castling_rights.insert(CastlingRights::BLACK_KING_SIDE),
            _ => return Err("Invalid castling rights in FEN"),
        }
    }

    let en_passant_square = match parts.next().unwrap() {
        "-" => None,
        san => san_to_int(san),
    };

    let halfmove_clock = match parts.next().unwrap().parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            return Err("Invalid halfmove clock in FEN");
        }
    };

    let fullmove_number = match parts.next().unwrap().parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            return Err("Invalid fullmove number in FEN");
        }
    };

    Ok(BoardState(
        mailbox,
        active_color,
        castling_rights,
        en_passant_square,
        halfmove_clock,
        fullmove_number,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fen_correctly() {
        let res = match parse_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2")
        {
            Ok(res) => res,
            Err(error) => panic!("{}", error),
        };

        let BoardState(
            mailbox,
            active_color,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
        ) = res;

        assert!(mailbox[0] == Some(Piece::RookWhite));
        assert!(mailbox[1] == Some(Piece::KnightWhite));
        assert!(mailbox[6] == None);
        assert!(mailbox[63] == Some(Piece::RookBlack));

        assert!(active_color == Color::Black);

        assert!(castling_rights.bits() == 0b1111);

        assert!(en_passant_square == None);

        assert!(halfmove_clock == 1);

        assert!(fullmove_number == 2);
    }

    #[test]
    fn parse_san_to_index_correctly() {
        match san_to_int("a1") {
            Some(number) => assert!(number == 0),
            None => panic!("SAN e3 parsing failed"),
        };

        match san_to_int("h8") {
            Some(number) => assert!(number == 63),
            None => panic!("SAN d6 parsing failed"),
        }
    }
}
