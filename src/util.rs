use std::ops::{Add, AddAssign};

pub trait Twiddling {
    fn set_bit(&mut self, n: u8, val: bool);
    fn get_bit(&self, n: u8) -> bool;

    fn msb_and_lsb(&self) -> [u8; 2];

    fn swap_nibbles(&mut self);
}

impl Twiddling for u8 {
    fn set_bit(&mut self, n: u8, val: bool) {
        let mask = 1 << n;
        let negged = if val { 0b1111_1111 } else { 0 };
        *self ^= (negged ^ *self) & mask;
    }

    fn get_bit(&self, n: u8) -> bool {
        let mask = 1 << n;
        let bit = *self & mask;
        bit != 0
    }

    fn msb_and_lsb(&self) -> [u8; 2] {
        [(*self & 0b1111_0000) >> 4, *self & 0b0000_1111]
    }

    fn swap_nibbles(&mut self) {
        let swapped = (*self << 4) | (*self >> 4);
        *self = swapped;
    }
}

#[derive(Debug, Default)]
pub struct RegisterPair {
    pub hi: u8,
    pub lo: u8,
}

impl RegisterPair {
    pub fn as_both(&self) -> u16 {
        let hi = (self.hi as u16) << 8;
        let lo = self.lo as u16;
        hi | lo
    }

    pub fn set_both(&mut self, value: u16) {
        let [hi_val, lo_val] = value.to_be_bytes();
        self.hi = hi_val;
        self.lo = lo_val;
    }

    pub fn inc(&mut self) {
        *self += 1
    }

    pub fn dec(&mut self) {
        let after_dec = self.as_both().wrapping_add(-1_i8 as u16);
        self.set_both(after_dec);
    }
}

impl Add<u16> for RegisterPair {
    type Output = u16;

    fn add(self, rhs: u16) -> Self::Output {
        self.as_both().wrapping_add(rhs)
    }
}

impl AddAssign<u16> for RegisterPair {
    fn add_assign(&mut self, rhs: u16) {
        let sum = self.as_both() + rhs;
        self.set_both(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        let opcode: u8 = 0xA3;
        let [left, right] = opcode.msb_and_lsb();
        assert_eq!(left, 0xA);
        assert_eq!(right, 0x3);
    }

    #[test]
    fn reg_as_both() {
        let pair = RegisterPair { hi: 0x44, lo: 0x22 };
        assert_eq!(pair.as_both(), 0x4422);
    }

    #[test]
    fn reg_set() {
        let mut pair = RegisterPair { hi: 0xA, lo: 0xB };
        pair.set_both(0xCCDC);
        assert_eq!(pair.hi, 0xCC);
        assert_eq!(pair.lo, 0xDC);
    }

    #[test]
    fn swap_nib() {
        use Twiddling;
        let mut x: u8 = 0b1010_1110;
        x.swap_nibbles();
        assert_eq!(x, 0b1110_1010);
    }

    #[test]
    fn dec_me_pls() {
        let mut reg_pair = RegisterPair::default();
        reg_pair.set_both(932_u16);
        reg_pair.dec();
        assert_eq!(reg_pair.as_both(), 931);
    }
}
