#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Move(u16);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveType {
    Normal,
    Promotion,
    EnPassant,
    Castling,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PromoPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

const FROM_MASK: u16 = 0b0000_0000_0011_1111;
const TO_SHIFT: u16 = 6;
const TO_MASK: u16 = 0b0000_1111_1100_0000;
const PROMO_SHIFT: u16 = 12;
const PROMO_MASK: u16 = 0b0011_0000_0000_0000;
const TYPE_SHIFT: u16 = 14;
const TYPE_MASK: u16 = 0b1100_0000_0000_0000;

impl Move {
    pub const NULL: Move = Move(0);

    #[inline(always)]
    pub fn new(from: u8, to: u8) -> Move {
        Move((from as u16) | ((to as u16) << TO_SHIFT))
    }

    #[inline(always)]
    pub fn new_promotion(from: u8, to: u8, promo: PromoPiece) -> Move {
        let promo_bits = promo as u16;
        let type_bits = MoveType::Promotion as u16;
        Move(
            (from as u16)
                | ((to as u16) << TO_SHIFT)
                | (promo_bits << PROMO_SHIFT)
                | (type_bits << TYPE_SHIFT),
        )
    }

    #[inline(always)]
    pub fn new_en_passant(from: u8, to: u8) -> Move {
        let type_bits = MoveType::EnPassant as u16;
        Move((from as u16) | ((to as u16) << TO_SHIFT) | (type_bits << TYPE_SHIFT))
    }

    #[inline(always)]
    pub fn new_castling(from: u8, to: u8) -> Move {
        let type_bits = MoveType::Castling as u16;
        Move((from as u16) | ((to as u16) << TO_SHIFT) | (type_bits << TYPE_SHIFT))
    }

    #[inline(always)]
    pub fn from(self) -> u8 {
        (self.0 & FROM_MASK) as u8
    }

    #[inline(always)]
    pub fn to(self) -> u8 {
        ((self.0 & TO_MASK) >> TO_SHIFT) as u8
    }

    #[inline(always)]
    pub fn move_type(self) -> MoveType {
        match (self.0 & TYPE_MASK) >> TYPE_SHIFT {
            0 => MoveType::Normal,
            1 => MoveType::Promotion,
            2 => MoveType::EnPassant,
            3 => MoveType::Castling,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn promo_piece(self) -> PromoPiece {
        match (self.0 & PROMO_MASK) >> PROMO_SHIFT {
            0 => PromoPiece::Knight,
            1 => PromoPiece::Bishop,
            2 => PromoPiece::Rook,
            3 => PromoPiece::Queen,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn is_null(self) -> bool {
        self.0 == 0
    }

    pub fn to_uci_string(self) -> String {
        let from_str = square_to_uci(self.from());
        let to_str = square_to_uci(self.to());
        let mut s = format!("{}{}", from_str, to_str);
        if self.move_type() == MoveType::Promotion {
            let c = match self.promo_piece() {
                PromoPiece::Knight => 'n',
                PromoPiece::Bishop => 'b',
                PromoPiece::Rook => 'r',
                PromoPiece::Queen => 'q',
            };
            s.push(c);
        }
        s
    }
}

fn square_to_uci(sq: u8) -> String {
    let file = (sq % 8) as u8;
    let rank = (sq / 8) as u8;
    format!("{}{}", (b'a' + file) as char, (b'1' + rank) as char)
}

pub const MAX_MOVES: usize = 256;

pub struct MoveList {
    moves: [Move; MAX_MOVES],
    count: usize,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            moves: [Move::NULL; MAX_MOVES],
            count: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, m: Move) {
        debug_assert!(self.count < MAX_MOVES, "MoveList overflow");
        self.moves[self.count] = m;
        self.count += 1;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.count
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.count].iter()
    }

    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.count]
    }
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_move_roundtrip() {
        let m = Move::new(12, 28);
        assert_eq!(m.from(), 12);
        assert_eq!(m.to(), 28);
        assert_eq!(m.move_type(), MoveType::Normal);
    }

    #[test]
    fn test_promotion_roundtrip() {
        let m = Move::new_promotion(52, 60, PromoPiece::Queen);
        assert_eq!(m.from(), 52);
        assert_eq!(m.to(), 60);
        assert_eq!(m.move_type(), MoveType::Promotion);
        assert_eq!(m.promo_piece(), PromoPiece::Queen);
    }

    #[test]
    fn test_en_passant_roundtrip() {
        let m = Move::new_en_passant(35, 44);
        assert_eq!(m.move_type(), MoveType::EnPassant);
    }

    #[test]
    fn test_castling_roundtrip() {
        let m = Move::new_castling(4, 6);
        assert_eq!(m.move_type(), MoveType::Castling);
    }

    #[test]
    fn test_uci_string() {
        let m = Move::new(12, 28); 
        assert_eq!(m.to_uci_string(), "e2e4");
    }

    #[test]
    fn test_move_list_push() {
        let mut list = MoveList::new();
        list.push(Move::new(0, 1));
        list.push(Move::new(2, 3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.as_slice()[0].from(), 0);
    }
}