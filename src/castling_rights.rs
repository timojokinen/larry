use bitflags::bitflags;

bitflags! {
    pub struct CastlingRights: u8 {
        const WHITE_KING_SIDE = 0b1000;
        const WHITE_QUEEN_SIDE = 0b0100;
        const BLACK_KING_SIDE = 0b0010;
        const BLACK_QUEEN_SIDE = 0b0001;
    }
}