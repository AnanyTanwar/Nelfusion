use crate::board::bitboard::Bitboard;
use crate::board::bitboard::pop_count;
use std::sync::LazyLock;

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

#[inline(always)]
pub const fn next_subset(subset: Bitboard, mask: Bitboard) -> Bitboard {
    (subset.wrapping_sub(mask)) & mask
}

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn next_sparse_u64(&mut self) -> u64 {
        self.next_u64() & self.next_u64() & self.next_u64()
    }
}

pub fn find_magic(sq: i32, mask: Bitboard, is_rook: bool, rng: &mut Rng) -> u64 {
    let bits = pop_count(mask) as u32;
    let shift = 64 - bits;

    let mut subsets: Vec<Bitboard> = Vec::new();
    let mut attacks: Vec<Bitboard> = Vec::new();

    let mut subset: Bitboard = 0;
    loop {
        subsets.push(subset);
        let atk = if is_rook {
            rook_attacks_slow(sq, subset)
        } else {
            bishop_attacks_slow(sq, subset)
        };
        attacks.push(atk);

        subset = next_subset(subset, mask);
        if subset == 0 {
            break;
        }
    }

    let table_size = 1usize << bits;

    loop {
        let candidate = rng.next_sparse_u64();

        let mut used: Vec<Bitboard> = vec![0; table_size];
        let mut filled: Vec<bool> = vec![false; table_size];
        let mut ok = true;

        for i in 0..subsets.len() {
            let idx = ((subsets[i].wrapping_mul(candidate)) >> shift) as usize;

            if !filled[idx] {
                filled[idx] = true;
                used[idx] = attacks[i];
            } else if used[idx] != attacks[i] {
                ok = false;
                break;
            }
        }

        if ok {
            return candidate;
        }
    }
}

pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u32,
    pub offset: usize,
}

pub struct MagicTables {
    pub rook_magics: [MagicEntry; 64],
    pub bishop_magics: [MagicEntry; 64],
    pub rook_table: Vec<Bitboard>,
    pub bishop_table: Vec<Bitboard>,
}

impl MagicTables {
    pub fn generate() -> MagicTables {
        let mut rng = Rng::new(0x1234567890ABCDEF);

        let mut rook_magics: Vec<MagicEntry> = Vec::with_capacity(64);
        let mut bishop_magics: Vec<MagicEntry> = Vec::with_capacity(64);
        let mut rook_table: Vec<Bitboard> = Vec::new();
        let mut bishop_table: Vec<Bitboard> = Vec::new();

        for sq in 0..64 {
            let mask = rook_relevant_mask(sq);
            let bits = pop_count(mask);
            let shift = 64 - bits;
            let magic = find_magic(sq, mask, true, &mut rng);
            let offset = rook_table.len();
            let table_size = 1usize << bits;

            rook_table.resize(offset + table_size, 0);

            let mut subset: Bitboard = 0;
            loop {
                let idx = ((subset.wrapping_mul(magic)) >> shift) as usize;
                rook_table[offset + idx] = rook_attacks_slow(sq, subset);

                subset = next_subset(subset, mask);
                if subset == 0 {
                    break;
                }
            }

            rook_magics.push(MagicEntry {
                mask,
                magic,
                shift,
                offset,
            });
        }

        for sq in 0..64 {
            let mask = bishop_relevant_mask(sq);
            let bits = pop_count(mask);
            let shift = 64 - bits;
            let magic = find_magic(sq, mask, false, &mut rng);
            let offset = bishop_table.len();
            let table_size = 1usize << bits;

            bishop_table.resize(offset + table_size, 0);

            let mut subset: Bitboard = 0;
            loop {
                let idx = ((subset.wrapping_mul(magic)) >> shift) as usize;
                bishop_table[offset + idx] = bishop_attacks_slow(sq, subset);

                subset = next_subset(subset, mask);
                if subset == 0 {
                    break;
                }
            }

            bishop_magics.push(MagicEntry {
                mask,
                magic,
                shift,
                offset,
            });
        }

        MagicTables {
            rook_magics: rook_magics
                .try_into()
                .unwrap_or_else(|_| panic!("expected 64 rook magics")),
            bishop_magics: bishop_magics
                .try_into()
                .unwrap_or_else(|_| panic!("expected 64 bishop magics")),
            rook_table,
            bishop_table,
        }
    }

    #[inline(always)]
    pub fn rook_attacks(&self, sq: usize, occupied: Bitboard) -> Bitboard {
        let entry = &self.rook_magics[sq];
        let relevant = occupied & entry.mask;
        let idx = ((relevant.wrapping_mul(entry.magic)) >> entry.shift) as usize;
        self.rook_table[entry.offset + idx]
    }

    #[inline(always)]
    pub fn bishop_attacks(&self, sq: usize, occupied: Bitboard) -> Bitboard {
        let entry = &self.bishop_magics[sq];
        let relevant = occupied & entry.mask;
        let idx = ((relevant.wrapping_mul(entry.magic)) >> entry.shift) as usize;
        self.bishop_table[entry.offset + idx]
    }
}

pub static MAGIC_TABLES: LazyLock<MagicTables> = LazyLock::new(MagicTables::generate);

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_subset_enumeration_count() {
        let mask = rook_relevant_mask(0);
        let mut subset: Bitboard = 0;
        let mut count = 0;
        loop {
            count += 1;
            subset = next_subset(subset, mask);
            if subset == 0 {
                break;
            }
        }
        assert_eq!(count, 1 << 12);
    }

    #[test]
    fn test_find_magic_rook_corner() {
        let mut rng = Rng::new(0x1234567890ABCDEF);
        let mask = rook_relevant_mask(0);
        let magic = find_magic(0, mask, true, &mut rng);
        assert!(magic != 0);
    }

    #[test]
    #[ignore]
    fn test_magic_tables_match_slow_attacks() {
        let tables = MagicTables::generate();

        for sq in 0..64 {
            let rook_mask = rook_relevant_mask(sq);
            let mut subset: Bitboard = 0;
            loop {
                let fast = tables.rook_attacks(sq as usize, subset);
                let slow = rook_attacks_slow(sq, subset);
                assert_eq!(fast, slow, "rook mismatch at sq {} occ {:#x}", sq, subset);

                subset = next_subset(subset, rook_mask);
                if subset == 0 {
                    break;
                }
            }

            let bishop_mask = bishop_relevant_mask(sq);
            subset = 0;
            loop {
                let fast = tables.bishop_attacks(sq as usize, subset);
                let slow = bishop_attacks_slow(sq, subset);
                assert_eq!(fast, slow, "bishop mismatch at sq {} occ {:#x}", sq, subset);

                subset = next_subset(subset, bishop_mask);
                if subset == 0 {
                    break;
                }
            }
        }
    }
}
