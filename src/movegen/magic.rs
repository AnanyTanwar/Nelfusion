use crate::board::bitboard::Bitboard;

pub const fn rook_relevant_mask(sq: i32) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut mask: Bitboard = 0;

    let mut f = 1;
    while f < 7 {
        if f != file {
            mask |= 1u64 << (rank * 8 + f);
        }
        f += 1;
    }

    let mut r = 1;
    while r < 7 {
        if r != rank {
            mask |= 1u64 << (r * 8 + file);
        }
        r += 1;
    }

    mask
}

pub const fn bishop_relevant_mask(sq: i32) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut mask: Bitboard = 0;

    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut d = 0;
    while d < 4 {
        let (df, dr) = directions[d];
        let mut f = file + df;
        let mut r = rank + dr;
        while f >= 1 && f <= 6 && r >= 1 && r <= 6 {
            mask |= 1u64 << (r * 8 + f);
            f += df;
            r += dr;
        }
        d += 1;
    }

    mask
}

pub const fn rook_attacks_slow(sq: i32, occupied: Bitboard) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut attacks: Bitboard = 0;

    let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut d = 0;
    while d < 4 {
        let (df, dr) = directions[d];
        let mut f = file + df;
        let mut r = rank + dr;
        while f >= 0 && f < 8 && r >= 0 && r < 8 {
            let target_sq = r * 8 + f;
            attacks |= 1u64 << target_sq;
            if (occupied >> target_sq) & 1 != 0 {
                break;
            }
            f += df;
            r += dr;
        }
        d += 1;
    }

    attacks
}

pub const fn bishop_attacks_slow(sq: i32, occupied: Bitboard) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut attacks: Bitboard = 0;

    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut d = 0;
    while d < 4 {
        let (df, dr) = directions[d];
        let mut f = file + df;
        let mut r = rank + dr;
        while f >= 0 && f < 8 && r >= 0 && r < 8 {
            let target_sq = r * 8 + f;
            attacks |= 1u64 << target_sq;
            if (occupied >> target_sq) & 1 != 0 {
                break;
            }
            f += df;
            r += dr;
        }
        d += 1;
    }

    attacks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::bitboard::pop_count;

    #[test]
    fn test_rook_mask_corner() {
        let mask = rook_relevant_mask(0);
        assert_eq!(pop_count(mask), 12);
    }

    #[test]
    fn test_rook_mask_center() {
        let mask = rook_relevant_mask(28);
        assert_eq!(pop_count(mask), 10);
    }

    #[test]
    fn test_bishop_mask_corner() {
        let mask = bishop_relevant_mask(0);
        assert_eq!(pop_count(mask), 6);
    }

    #[test]
    fn test_bishop_mask_center() {
        let mask = bishop_relevant_mask(28);
        assert_eq!(pop_count(mask), 9);
    }

    #[test]
    fn test_rook_attacks_open_board() {
        let attacks = rook_attacks_slow(0, 0);
        assert_eq!(pop_count(attacks), 14);
    }

    #[test]
    fn test_rook_attacks_blocked() {
        let occ = 1u64 << 24;
        let attacks = rook_attacks_slow(0, occ);
        assert!(attacks & (1u64 << 24) != 0);
        assert!(attacks & (1u64 << 32) == 0);
    }

    #[test]
    fn test_bishop_attacks_open_board() {
        let attacks = bishop_attacks_slow(28, 0);
        assert_eq!(pop_count(attacks), 13);
    }
}
