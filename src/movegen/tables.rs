use crate::board::bitboard::{Bitboard, EMPTY};

pub const fn knight_attacks_table() -> [Bitboard; 64] {
    let mut table = [EMPTY; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = knight_attacks_from(sq as i32);
        sq += 1;
    }
    table
}

pub const fn king_attacks_table() -> [Bitboard; 64] {
    let mut table = [EMPTY; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = king_attacks_from(sq as i32);
        sq += 1;
    }
    table
}

pub const fn pawn_attacks_table_white() -> [Bitboard; 64] {
    let mut table = [EMPTY; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = pawn_attacks_from(sq as i32, true);
        sq += 1;
    }
    table
}

pub const fn pawn_attacks_table_black() -> [Bitboard; 64] {
    let mut table = [EMPTY; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = pawn_attacks_from(sq as i32, false);
        sq += 1;
    }
    table
}

const fn knight_attacks_from(sq: i32) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut attacks: Bitboard = 0;

    const OFFSETS: [(i32, i32); 8] = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    let mut i = 0;
    while i < 8 {
        let (df, dr) = OFFSETS[i];
        let new_file = file + df;
        let new_rank = rank + dr;
        if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
            let target_sq = new_rank * 8 + new_file;
            attacks |= 1u64 << target_sq;
        }
        i += 1;
    }

    attacks
}

const fn king_attacks_from(sq: i32) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut attacks: Bitboard = 0;

    const OFFSETS: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut i = 0;
    while i < 8 {
        let (df, dr) = OFFSETS[i];
        let new_file = file + df;
        let new_rank = rank + dr;
        if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
            let target_sq = new_rank * 8 + new_file;
            attacks |= 1u64 << target_sq;
        }
        i += 1;
    }

    attacks
}

const fn pawn_attacks_from(sq: i32, is_white: bool) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut attacks: Bitboard = 0;

    let dr: i32 = if is_white { 1 } else { -1 };
    let new_rank = rank + dr;

    let left_file = file - 1;
    if left_file >= 0 && new_rank >= 0 && new_rank < 8 {
        let target_sq = new_rank * 8 + left_file;
        attacks |= 1u64 << target_sq;
    }

    let right_file = file + 1;
    if right_file < 8 && new_rank >= 0 && new_rank < 8 {
        let target_sq = new_rank * 8 + right_file;
        attacks |= 1u64 << target_sq;
    }

    attacks
}

pub static KNIGHT_ATTACKS: [Bitboard; 64] = knight_attacks_table();
pub static KING_ATTACKS: [Bitboard; 64] = king_attacks_table();
pub static PAWN_ATTACKS_WHITE: [Bitboard; 64] = pawn_attacks_table_white();
pub static PAWN_ATTACKS_BLACK: [Bitboard; 64] = pawn_attacks_table_black();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::bitboard::pop_count;

    #[test]
    fn test_knight_corner_attacks() {
        let attacks = KNIGHT_ATTACKS[0];
        assert_eq!(pop_count(attacks), 2);
    }

    #[test]
    fn test_knight_center_attacks() {
        let attacks = KNIGHT_ATTACKS[28];
        assert_eq!(pop_count(attacks), 8);
    }

    #[test]
    fn test_king_corner_attacks() {
        let attacks = KING_ATTACKS[0];
        assert_eq!(pop_count(attacks), 3);
    }

    #[test]
    fn test_king_center_attacks() {
        let attacks = KING_ATTACKS[28];
        assert_eq!(pop_count(attacks), 8);
    }

    #[test]
    fn test_white_pawn_attacks_center() {
        let attacks = PAWN_ATTACKS_WHITE[28];
        assert_eq!(pop_count(attacks), 2);
        assert!(attacks & (1u64 << 35) != 0);
        assert!(attacks & (1u64 << 37) != 0);
    }

    #[test]
    fn test_black_pawn_attacks_center() {
        let attacks = PAWN_ATTACKS_BLACK[28];
        assert_eq!(pop_count(attacks), 2);
        assert!(attacks & (1u64 << 19) != 0);
        assert!(attacks & (1u64 << 21) != 0);
    }

    #[test]
    fn test_white_pawn_attacks_edge() {
        let attacks = PAWN_ATTACKS_WHITE[24];
        assert_eq!(pop_count(attacks), 1);
        assert!(attacks & (1u64 << 33) != 0);
    }

    #[test]
    fn test_white_pawn_attacks_rank8() {
        let attacks = PAWN_ATTACKS_WHITE[56];
        assert_eq!(attacks, EMPTY);
    }
}
