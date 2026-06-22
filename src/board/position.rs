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

// Castling rights as bit flags
pub const WK_CASTLE: u8 = 1; // white king-side
pub const WQ_CASTLE: u8 = 2; // white queen-side
pub const BK_CASTLE: u8 = 4; // black king-side
pub const BQ_CASTLE: u8 = 8; // black queen-side

#[derive(Copy, Clone, Debug)]
pub struct Position {
    // [color][piece_type] -> bitboard
    pub pieces: [[Bitboard; NUM_PIECE_TYPES]; 2],

    // occupancy helpers, recomputed from pieces but cached for speed
    pub occupied_by: [Bitboard; 2], // all white pieces, all black pieces
    pub occupied: Bitboard,         // all pieces combined

    pub side_to_move: Color,
    pub castling_rights: u8,
    pub en_passant: Option<u8>, // square index, if a double pawn push just happened
    pub halfmove_clock: u16,    // for 50-move rule
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

        // White pieces
        pos.pieces[Color::White as usize][PieceType::Pawn as usize] = 0x0000_0000_0000_FF00;
        pos.pieces[Color::White as usize][PieceType::Knight as usize] = 0x0000_0000_0000_0042;
        pos.pieces[Color::White as usize][PieceType::Bishop as usize] = 0x0000_0000_0000_0024;
        pos.pieces[Color::White as usize][PieceType::Rook as usize] = 0x0000_0000_0000_0081;
        pos.pieces[Color::White as usize][PieceType::Queen as usize] = 0x0000_0000_0000_0008;
        pos.pieces[Color::White as usize][PieceType::King as usize] = 0x0000_0000_0000_0010;

        // Black pieces (mirrored on ranks 7 and 8)
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

    /// Returns the piece type and color occupying a square, if any.
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
        // e1 = white king, index 4
        assert_eq!(pos.piece_at(4), Some((Color::White, PieceType::King)));
        // e8 = black king, index 60
        assert_eq!(pos.piece_at(60), Some((Color::Black, PieceType::King)));
        // e4 = empty, index 28
        assert_eq!(pos.piece_at(28), None);
    }
}
