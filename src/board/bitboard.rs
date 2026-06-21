pub type Bitboard = u64;

pub const EMPTY: Bitboard = 0;
pub const FULL: Bitboard = !0;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    #[inline(always)]
    pub fn from_index(i: u8) -> Square {
        unsafe { std::mem::transmute(i) }
    }

    #[inline(always)]
    pub fn index(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub fn file(self) -> u8 {
        self.index() % 8
    }

    #[inline(always)]
    pub fn rank(self) -> u8 {
        self.index() / 8
    }
}

#[inline(always)]
pub fn set_bit(bb: Bitboard, sq: u8) -> Bitboard {
    bb | (1u64 << sq)
}

#[inline(always)]
pub fn clear_bit(bb: Bitboard, sq: u8) -> Bitboard {
    bb & !(1u64 << sq)
}

#[inline(always)]
pub fn get_bit(bb: Bitboard, sq: u8) -> bool {
    (bb >> sq) & 1 == 1
}

#[inline(always)]
pub fn pop_count(bb: Bitboard) -> u32 {
    bb.count_ones()
}

#[inline(always)]
pub fn lsb(bb: Bitboard) -> u8 {
    bb.trailing_zeros() as u8
}

#[inline(always)]
pub fn pop_lsb(bb: &mut Bitboard) -> u8 {
    let sq = lsb(*bb);
    *bb &= *bb - 1;
    sq
}

pub fn print_bitboard(bb: Bitboard) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let sq = rank * 8 + file;
            print!("{} ", if get_bit(bb, sq) { '1' } else { '.' });
        }
        println!();
    }
    println!("  a b c d e f g h");
    println!("Bitboard value: {}", bb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_clear_get() {
        let mut bb = EMPTY;
        bb = set_bit(bb, Square::E4.index());
        assert!(get_bit(bb, Square::E4.index()));
        bb = clear_bit(bb, Square::E4.index());
        assert!(!get_bit(bb, Square::E4.index()));
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = set_bit(set_bit(EMPTY, 5), 10);
        let first = pop_lsb(&mut bb);
        assert_eq!(first, 5);
        let second = pop_lsb(&mut bb);
        assert_eq!(second, 10);
        assert_eq!(bb, EMPTY);
    }
}