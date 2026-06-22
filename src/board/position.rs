use crate::board::bitboard::{Bitboard, EMPTY, get_bit, pop_count};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline(always)]
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const NUM_PIECE_TYPES: usize = 6;

pub const WK_CASTLE: u8 = 1;
pub const WQ_CASTLE: u8 = 2;
pub const BK_CASTLE: u8 = 4;
pub const BQ_CASTLE: u8 = 8;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub pieces: [[Bitboard; NUM_PIECE_TYPES]; 2],
    pub occupied_by: [Bitboard; 2],
    pub occupied: Bitboard,
    pub side_to_move: Color,
    pub castling_rights: u8,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
}

impl Position {
    pub fn empty() -> Position {
        Position {
            pieces: [[EMPTY; NUM_PIECE_TYPES]; 2],
            occupied_by: [EMPTY; 2],
            occupied: EMPTY,
            side_to_move: Color::White,
            castling_rights: WK_CASTLE | WQ_CASTLE | BK_CASTLE | BQ_CASTLE,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn startpos() -> Position {
        let mut pos = Position::empty();

        pos.pieces[Color::White as usize][PieceType::Pawn as usize] = 0x0000_0000_0000_FF00;
        pos.pieces[Color::White as usize][PieceType::Knight as usize] = 0x0000_0000_0000_0042;
        pos.pieces[Color::White as usize][PieceType::Bishop as usize] = 0x0000_0000_0000_0024;
        pos.pieces[Color::White as usize][PieceType::Rook as usize] = 0x0000_0000_0000_0081;
        pos.pieces[Color::White as usize][PieceType::Queen as usize] = 0x0000_0000_0000_0008;
        pos.pieces[Color::White as usize][PieceType::King as usize] = 0x0000_0000_0000_0010;

        pos.pieces[Color::Black as usize][PieceType::Pawn as usize] = 0x00FF_0000_0000_0000;
        pos.pieces[Color::Black as usize][PieceType::Knight as usize] = 0x4200_0000_0000_0000;
        pos.pieces[Color::Black as usize][PieceType::Bishop as usize] = 0x2400_0000_0000_0000;
        pos.pieces[Color::Black as usize][PieceType::Rook as usize] = 0x8100_0000_0000_0000;
        pos.pieces[Color::Black as usize][PieceType::Queen as usize] = 0x0800_0000_0000_0000;
        pos.pieces[Color::Black as usize][PieceType::King as usize] = 0x1000_0000_0000_0000;

        pos.update_occupancy();
        pos
    }

    pub fn update_occupancy(&mut self) {
        self.occupied_by[Color::White as usize] = self.pieces[Color::White as usize]
            .iter()
            .fold(EMPTY, |acc, &bb| acc | bb);
        self.occupied_by[Color::Black as usize] = self.pieces[Color::Black as usize]
            .iter()
            .fold(EMPTY, |acc, &bb| acc | bb);
        self.occupied =
            self.occupied_by[Color::White as usize] | self.occupied_by[Color::Black as usize];
    }

    pub fn piece_at(&self, sq: u8) -> Option<(Color, PieceType)> {
        for &color in &[Color::White, Color::Black] {
            for (i, &bb) in self.pieces[color as usize].iter().enumerate() {
                if get_bit(bb, sq) {
                    let piece_type = match i {
                        0 => PieceType::Pawn,
                        1 => PieceType::Knight,
                        2 => PieceType::Bishop,
                        3 => PieceType::Rook,
                        4 => PieceType::Queen,
                        5 => PieceType::King,
                        _ => unreachable!(),
                    };
                    return Some((color, piece_type));
                }
            }
        }
        None
    }

    /// Returns true if `sq` is attacked by any piece of `attacker` color.
    /// Uses `occupied` for sliding piece rays so pinned/blocking pieces are respected.
    pub fn is_square_attacked(&self, sq: u8, attacker: Color, occupied: Bitboard) -> bool {
        use crate::movegen::magic::MAGIC_TABLES;
        use crate::movegen::tables::{
            KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS_BLACK, PAWN_ATTACKS_WHITE,
        };

        let them = attacker as usize;

        // Pawn attacks — use the defender's pawn attack table to find attackers
        // e.g. if attacker is White, a white pawn on sq attacks from below,
        // so we check if any white pawn sits on a square that attacks sq from white's perspective.
        // Equivalently: does sq appear in PAWN_ATTACKS_WHITE[pawn_sq]?
        // Faster: check PAWN_ATTACKS_BLACK[sq] (attacks from sq as if black) & white pawns
        let pawn_attackers = if attacker == Color::White {
            PAWN_ATTACKS_BLACK[sq as usize]
        } else {
            PAWN_ATTACKS_WHITE[sq as usize]
        };
        if pawn_attackers & self.pieces[them][PieceType::Pawn as usize] != 0 {
            return true;
        }

        // Knights
        if KNIGHT_ATTACKS[sq as usize] & self.pieces[them][PieceType::Knight as usize] != 0 {
            return true;
        }

        // King
        if KING_ATTACKS[sq as usize] & self.pieces[them][PieceType::King as usize] != 0 {
            return true;
        }

        // Bishops + diagonal queens
        let diag_attackers = self.pieces[them][PieceType::Bishop as usize]
            | self.pieces[them][PieceType::Queen as usize];
        if MAGIC_TABLES.bishop_attacks(sq as usize, occupied) & diag_attackers != 0 {
            return true;
        }

        // Rooks + straight queens
        let straight_attackers = self.pieces[them][PieceType::Rook as usize]
            | self.pieces[them][PieceType::Queen as usize];
        if MAGIC_TABLES.rook_attacks(sq as usize, occupied) & straight_attackers != 0 {
            return true;
        }

        false
    }

    pub fn material_count(&self, color: Color) -> u32 {
        self.pieces[color as usize]
            .iter()
            .map(|bb| pop_count(*bb))
            .sum()
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let sq = rank * 8 + file;
                let symbol = match self.piece_at(sq) {
                    Some((Color::White, PieceType::Pawn)) => 'P',
                    Some((Color::White, PieceType::Knight)) => 'N',
                    Some((Color::White, PieceType::Bishop)) => 'B',
                    Some((Color::White, PieceType::Rook)) => 'R',
                    Some((Color::White, PieceType::Queen)) => 'Q',
                    Some((Color::White, PieceType::King)) => 'K',
                    Some((Color::Black, PieceType::Pawn)) => 'p',
                    Some((Color::Black, PieceType::Knight)) => 'n',
                    Some((Color::Black, PieceType::Bishop)) => 'b',
                    Some((Color::Black, PieceType::Rook)) => 'r',
                    Some((Color::Black, PieceType::Queen)) => 'q',
                    Some((Color::Black, PieceType::King)) => 'k',
                    None => '.',
                };
                print!("{} ", symbol);
            }
            println!();
        }
        println!("  a b c d e f g h");
        println!("Side to move: {:?}", self.side_to_move);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startpos_piece_count() {
        let pos = Position::startpos();
        assert_eq!(pos.material_count(Color::White), 16);
        assert_eq!(pos.material_count(Color::Black), 16);
        assert_eq!(pop_count(pos.occupied), 32);
    }

    #[test]
    fn test_piece_at() {
        let pos = Position::startpos();
        assert_eq!(pos.piece_at(4), Some((Color::White, PieceType::King)));
        assert_eq!(pos.piece_at(60), Some((Color::Black, PieceType::King)));
        assert_eq!(pos.piece_at(28), None);
    }

    #[test]
    fn test_is_square_attacked_startpos() {
        let pos = Position::startpos();
        // e2 (sq 12) is attacked by white knight on g1 and b1, and the king/queen can't reach
        // but most importantly e4 (28) should not be attacked by anyone at startpos
        assert!(!pos.is_square_attacked(28, Color::White, pos.occupied));
        assert!(!pos.is_square_attacked(28, Color::Black, pos.occupied));
        // e3 (sq 20) is attacked by white pawns on d2 and f2
        assert!(pos.is_square_attacked(20, Color::White, pos.occupied));
    }
}
