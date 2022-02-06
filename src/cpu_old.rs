use std::ops::{Index, IndexMut};

use crate::util::Twiddling;
use anyhow::Result;

const BANK_SIZE: usize = 16 * 1024;
type Bank = [u8; BANK_SIZE];

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Result<Self> {
        // let data = Self::split_rom(rom)?;
        let data = rom;

        let cart = Self { data };
        Ok(cart)
    }

    // fn split_rom(rom: Vec<u8>) -> Result<Vec<Bank>> {
    //     //safety checks
    //     if rom.len() % BANK_SIZE != 0 {
    //         anyhow::bail!("len not alligned to bank size");
    //     }
    //     let mut split = Vec::with_capacity(rom.len());
    //     let chunks = rom.as_slice().chunks_exact(BANK_SIZE);
    //     for chunk in chunks {

    //     }

    //     Ok(chunks)
    // }
}

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

    pub fn set_both(&mut self, val: u16) {
        let [hi_val, lo_val] = val.to_be_bytes();
        self.hi = hi_val;
        self.lo = lo_val;
    }
}

struct Ram([u8; 65536]);

impl Index<u16> for Ram {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        let index = index as usize;
        &self.0[index]
    }
}

impl IndexMut<u16> for Ram {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let index = index as usize;
        &mut self.0[index]
    }
}

impl std::ops::Deref for Ram {
    type Target = [u8; 65536];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Flags {}

pub struct Cpu {
    //registers
    pub a: u8,
    pub bc: RegisterPair,
    pub de: RegisterPair,
    pub hl: RegisterPair,
    pub sp: u16,
    pub pc: u16,

    flags: Flags,

    ram: Ram,

    //interrupts enabled?
    ime: bool,

    cartridge: Cartridge,
}

impl Cpu {
    pub fn z_flag(&self) -> bool {
        self.af.lo.get_bit(7)
    }

    pub fn set_z_flag(&mut self, val: bool) {
        self.af.lo.set_bit(7, val)
    }

    pub fn n_flag(&self) -> bool {
        self.af.lo.get_bit(6)
    }

    pub fn set_n_flag(&mut self, val: bool) {
        self.af.lo.set_bit(6, val)
    }

    pub fn h_flag(&self) -> bool {
        self.af.lo.get_bit(5)
    }

    pub fn set_h_flag(&mut self, val: bool) {
        self.af.lo.set_bit(5, val)
    }

    pub fn c_flag(&self) -> bool {
        self.af.lo.get_bit(4)
    }

    pub fn set_c_flag(&mut self, val: bool) {
        self.af.lo.set_bit(4, val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[allow(clippy::bool_assert_comparison)]
    fn set_get() {
        let mut f_reg = 0b1101_0000;

        assert_eq!(f_reg.get_bit(0), false);
        assert_eq!(f_reg.get_bit(1), false);
        assert_eq!(f_reg.get_bit(2), false);
        assert_eq!(f_reg.get_bit(3), false);
        assert_eq!(f_reg.get_bit(4), true);
        assert_eq!(f_reg.get_bit(5), false);
        assert_eq!(f_reg.get_bit(6), true);
        assert_eq!(f_reg.get_bit(7), true);

        f_reg.set_bit(4, false);
        f_reg.set_bit(5, true);

        assert_eq!(f_reg.get_bit(4), false);
        assert_eq!(f_reg.get_bit(5), true);
    }
}
