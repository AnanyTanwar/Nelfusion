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

pub static KNIGHT_ATTACKS: [Bitboard; 64] = knight_attacks_table();
pub static KING_ATTACKS: [Bitboard; 64] = king_attacks_table();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::bitboard::pop_count;

    #[test]
    fn test_knight_corner_attacks() {
        // Knight on a1 (square 0) should have exactly 2 attack squares.
        let attacks = KNIGHT_ATTACKS[0];
        assert_eq!(pop_count(attacks), 2);
    }

    #[test]
    fn test_knight_center_attacks() {
        // Knight on e4 (square 28) should have 8 attack squares (fully surrounded).
        let attacks = KNIGHT_ATTACKS[28];
        assert_eq!(pop_count(attacks), 8);
    }

    #[test]
    fn test_king_corner_attacks() {
        // King on a1 (square 0) should have exactly 3 attack squares.
        let attacks = KING_ATTACKS[0];
        assert_eq!(pop_count(attacks), 3);
    }

    #[test]
    fn test_king_center_attacks() {
        // King on e4 (square 28) should have 8 attack squares.
        let attacks = KING_ATTACKS[28];
        assert_eq!(pop_count(attacks), 8);
    }
}
