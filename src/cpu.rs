use std::ops::{BitAnd, BitOr, Index, IndexMut, Not, Shl, ShlAssign, Shr, ShrAssign};
use thiserror::Error;

use crate::cartridge::Cartridge;
use crate::util::RegisterPair;
use crate::util::Twiddling;
use anyhow::{bail, Result};

use crate::opcode;

const M: usize = 4;

const START_ADDR: u16 = 0x100;

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("illegal opcode {0} called")]
    IllegalOpcode(u8),
    #[error("opcode called with bad argument")]
    BadArgument,
}

struct CpuRam([u8; 65536]);

impl Index<u16> for CpuRam {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        let index = index as usize;
        &self.0[index]
    }
}

impl IndexMut<u16> for CpuRam {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let index = index as usize;
        &mut self.0[index]
    }
}

impl std::ops::Deref for CpuRam {
    type Target = [u8; 65536];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct Flags {
    pub zero: bool,       //bit 7 of F register
    pub neg: bool,        //bit 6 of F register
    pub half_carry: bool, //bit 5 of F register
    pub carry: bool,      //bit 4 of F register
}

fn set_if(value: &mut bool, predicate: bool) {
    *value = predicate;
}

impl Flags {
    pub fn f_reg(&self) -> u8 {
        (self.zero as u8).shl(7)
            | (self.neg as u8).shl(6)
            | (self.half_carry as u8).shl(5)
            | (self.carry as u8).shl(4)
    }

    #[inline(always)]
    pub fn set_from_byte(&mut self, value: u8) {
        self.zero = value.get_bit(4);
        self.neg = value.get_bit(5);
        self.half_carry = value.get_bit(6);
        self.carry = value.get_bit(7);
    }
}

#[derive(Default, Debug)]
pub struct Registers {
    pub a: u8,
    pub bc: RegisterPair,
    pub de: RegisterPair,
    pub hl: RegisterPair,
    pub sp: u16,
    pub pc: u16,
    pub flags: Flags,
}

impl Registers {
    pub fn with_boot_values() -> Self {
        let mut registers = Registers::default();

        registers.set_af(0x01B0);
        registers.bc.set_both(0x0013);
        registers.de.set_both(0x00D8);
        registers.hl.set_both(0x014D);
        registers.pc = 0x0100;
        registers.sp = 0xFFFE;

        registers
    }

    pub fn af(&self) -> u16 {
        let f = self.flags.f_reg() as u16;
        let a = self.a as u16;
        (a << 8) | f
    }

    pub fn set_af(&mut self, value: u16) {
        let [msb, lsb] = value.to_be_bytes();
        self.a = msb;
        self.flags.set_from_byte(lsb);
    }

    pub fn decimal_adjust_a(&mut self) {
        //adapted from https://ehaskins.com/2018-01-30%20Z80%20DAA/ and https://github.com/mvdnes/rboy/blob/master/src/cpu.rs

        let mut adjust = if self.flags.carry { 0x60 } else { 0x00 };

        if self.flags.half_carry {
            adjust |= 0x06;
        }

        if !self.flags.neg {
            if self.a & 0x0F > 0x09 {
                adjust |= 0x06;
            }
            if self.a > 0x99 {
                adjust |= 0x60;
            }
            self.a = self.a.wrapping_add(adjust);
        } else {
            self.a = self.a.wrapping_sub(adjust);
        }

        if self.a == 0 {
            self.flags.zero = true;
        }

        if adjust >= 0x60 {
            self.flags.carry = true;
        }
    }
}

pub struct Cpu {
    registers: Registers,

    // ram: CpuRam,
    cartridge: Cartridge,

    ime: bool,
    stopped: bool,

    cycles_passed: usize,
}

impl Cpu {
    pub fn debug_header(&self) {
        println!("{:?}", self.cartridge.header);
    }

    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            registers: Registers::with_boot_values(),
            // ram,
            cartridge,
            ime: false,
            stopped: false,
            cycles_passed: 0,
        }
    }

    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cartridge.bank0()[usize::from(addr)], //16 KiB ROM bank 00
            0x4000..=0x7FFF => {
                let offset = addr - 0x4000;
                self.cartridge.cur_bank()[usize::from(offset)]
            } //16 KiB ROM Bank 01~NN
            0x8000..=0x9FFF => todo!(),                                   //8 KiB Video RAM (VRAM)
            0xA000..=0xBFFF => todo!(),                                   //8 KiB External RAM
            0xC000..=0xCFFF => todo!(),                                   //4 KiB Work RAM (WRAM)
            0xD000..=0xDFFF => todo!(),                                   //4 KiB Work RAM (WRAM)
            0xE000..=0xFDFF => todo!(), //Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => todo!(), //Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => todo!(), //Not Usable
            0xFF00..=0xFF7F => todo!(), //I/O Registers
            0xFF80..=0xFFFE => todo!(), //High RAM (HRAM)
            0xFFFF => todo!(),          //Interrupt Enable register (IE)
        }
    }

    fn read_next_u8(&mut self) -> u8 {
        let next_byte = self.read_byte(self.registers.pc);
        self.registers.pc += 1;
        next_byte
    }

    fn read_next_u16(&mut self) -> u16 {
        let low_bytes = self.read_next_u8() as u16;
        let high_bytes = self.read_next_u8() as u16;
        (high_bytes << 8) | low_bytes
    }

    fn write_byte(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x3FFF => todo!(), //16 KiB ROM bank 00
            0x4000..=0x7FFF => todo!(), //16 KiB ROM Bank 01~NN
            0x8000..=0x9FFF => todo!(), //8 KiB Video RAM (VRAM)
            0xA000..=0xBFFF => todo!(), //8 KiB External RAM
            0xC000..=0xCFFF => todo!(), //4 KiB Work RAM (WRAM)
            0xD000..=0xDFFF => todo!(), //4 KiB Work RAM (WRAM)
            0xE000..=0xFDFF => todo!(), //Mirror of C000~DDFF (ECHO RAM)
            0xFE00..=0xFE9F => todo!(), //Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => todo!(), //Not Usable
            0xFF00..=0xFF7F => todo!(), //I/O Registers
            0xFF80..=0xFFFE => todo!(), //High RAM (HRAM)
            0xFFFF => todo!(),          //Interrupt Enable register (IE)
        }
    }

    fn push(&mut self, value: u16) {
        let [msb, lsb] = value.to_be_bytes();

        self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, msb);
        self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, lsb);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.read_byte(self.registers.sp);
        self.registers.sp.wrapping_add(1);
        let msb = self.read_byte(self.registers.sp);
        self.registers.sp.wrapping_add(1);

        u16::from_be_bytes([msb, lsb])
    }

    pub fn exec_next(&mut self) {
        use opcode::{Op16, Op8, OpcodePrefixed, OpcodeUnprefixed};

        let opcode = self.read_next_u8();
        let opcode_info = &opcode::LOOKUP[opcode as usize];
        // self.cycles_passed += opcode_info.cycles;

        match opcode_info.inst {
            OpcodeUnprefixed::Nop => {}
            OpcodeUnprefixed::Ld16(target, source) => {
                let val = match source {
                    Op16::U16 => self.read_next_u16(),
                    Op16::HL => self.registers.hl.as_both(),
                    Op16::SPPlusI8 => {
                        let signed_imm = self.read_next_u8();
                        self.registers.sp.wrapping_add(signed_imm as u16)
                    }
                    _ => unimplemented!(),
                };

                match target {
                    Op16::SP => self.registers.sp = val,
                    Op16::BC => self.registers.bc.set_both(val),
                    Op16::DE => self.registers.de.set_both(val),
                    Op16::HL => self.registers.hl.set_both(val),
                    _ => unimplemented!(),
                };
            }
            OpcodeUnprefixed::Ld8(target, source) => {
                let data = match source {
                    Op8::U8 => self.read_next_u8(),
                    Op8::AddrU16 => {
                        let addr = self.read_next_u16();
                        self.read_byte(addr)
                    }
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrBC => self.read_byte(self.registers.bc.as_both()),
                    Op8::AddrDE => self.read_byte(self.registers.de.as_both()),
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::AddrHLInc | Op8::AddrHLDec => {
                        let val = self.read_byte(self.registers.hl.as_both());
                        if source == Op8::AddrHLInc {
                            self.registers.hl.inc();
                        } else {
                            self.registers.hl.dec();
                        }
                        val
                    }
                    Op8::LowAddrC | Op8::LowAddrU8 => {
                        let low_bits = if source == Op8::LowAddrC {
                            self.registers.bc.lo
                        } else {
                            self.read_next_u8()
                        };
                        let addr = (low_bits as u16) + 0xFF00;
                        self.read_byte(addr)
                    }
                    _ => unimplemented!(),
                };

                match target {
                    Op8::A => self.registers.a = data,
                    Op8::B => self.registers.bc.hi = data,
                    Op8::C => self.registers.bc.lo = data,
                    Op8::D => self.registers.de.hi = data,
                    Op8::E => self.registers.de.lo = data,
                    Op8::H => self.registers.hl.hi = data,
                    Op8::L => self.registers.hl.lo = data,
                    Op8::AddrBC => self.write_byte(self.registers.bc.as_both(), data),
                    Op8::AddrDE => self.write_byte(self.registers.de.as_both(), data),
                    Op8::AddrHL => self.write_byte(self.registers.hl.as_both(), data),
                    Op8::AddrHLInc | Op8::AddrHLDec => {
                        self.write_byte(self.registers.hl.as_both(), data);
                        if target == Op8::AddrHLInc {
                            self.registers.hl.inc();
                        } else {
                            self.registers.hl.dec();
                        }
                    }
                    Op8::AddrU16 => {
                        let addr = self.read_next_u16();
                        self.write_byte(addr, data);
                    }
                    Op8::LowAddrC | Op8::LowAddrU8 => {
                        let low_bits = if source == Op8::LowAddrC {
                            self.registers.bc.lo
                        } else {
                            self.read_next_u8()
                        };
                        let addr = (low_bits as u16) + 0xFF00;
                        self.write_byte(addr, data);
                    }
                    _ => unimplemented!(),
                };
            }
            OpcodeUnprefixed::LdU16StackAddr => {
                let addr = self.read_next_u16();
                let data = self.read_byte(addr);
                self.write_byte(addr, data);
            }
            OpcodeUnprefixed::Inc8(target) => {
                match target {
                    Op8::A => self.registers.a = self.registers.a.wrapping_add(1),
                    Op8::B => self.registers.bc.hi = self.registers.bc.hi.wrapping_add(1),
                    Op8::C => self.registers.bc.lo = self.registers.bc.lo.wrapping_add(1),
                    Op8::D => self.registers.de.hi = self.registers.de.hi.wrapping_add(1),
                    Op8::E => self.registers.de.lo = self.registers.de.lo.wrapping_add(1),
                    Op8::H => self.registers.hl.hi = self.registers.hl.hi.wrapping_add(1),
                    Op8::L => self.registers.hl.lo = self.registers.hl.lo.wrapping_add(1),
                    Op8::AddrHL => {
                        let addr = self.registers.hl.as_both();
                        let data = self.read_byte(addr).wrapping_add(1);
                        self.write_byte(addr, data);
                    }
                    _ => unimplemented!(),
                };
            }
            OpcodeUnprefixed::Inc16(target) => {
                match target {
                    Op16::SP => self.registers.sp = self.registers.sp.wrapping_add(1),
                    Op16::BC => self.registers.bc.inc(),
                    Op16::DE => self.registers.de.inc(),
                    Op16::HL => self.registers.hl.inc(),
                    _ => unimplemented!(),
                };
            }
            OpcodeUnprefixed::Dec8(target) => {
                match target {
                    Op8::A => self.registers.a = self.registers.a.wrapping_sub(1),
                    Op8::B => self.registers.bc.hi = self.registers.bc.hi.wrapping_sub(1),
                    Op8::C => self.registers.bc.lo = self.registers.bc.lo.wrapping_sub(1),
                    Op8::D => self.registers.de.hi = self.registers.de.hi.wrapping_sub(1),
                    Op8::E => self.registers.de.lo = self.registers.de.lo.wrapping_sub(1),
                    Op8::H => self.registers.hl.hi = self.registers.hl.hi.wrapping_sub(1),
                    Op8::L => self.registers.hl.lo = self.registers.hl.lo.wrapping_sub(1),
                    Op8::AddrHL => {
                        let subr = self.registers.hl.as_both();
                        let data = self.read_byte(subr).wrapping_sub(1);
                        self.write_byte(subr, data);
                    }
                    _ => unimplemented!(),
                };
            }
            OpcodeUnprefixed::Dec16(target) => {
                match target {
                    Op16::SP => self.registers.sp = self.registers.sp.wrapping_sub(1),
                    Op16::BC => self.registers.bc.dec(),
                    Op16::DE => self.registers.de.dec(),
                    Op16::HL => self.registers.hl.dec(),
                    _ => unimplemented!(),
                };
            }
            OpcodeUnprefixed::Add16(rhs) => {
                let data = match rhs {
                    Op16::SP => self.registers.sp,
                    Op16::BC => self.registers.bc.as_both(),
                    Op16::DE => self.registers.de.as_both(),
                    Op16::HL => self.registers.hl.as_both(),
                    _ => unimplemented!(),
                };
                self.registers.hl += data;
            }
            OpcodeUnprefixed::Add8(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };
                self.registers.a.wrapping_add(data);
            }
            OpcodeUnprefixed::AddI8SP => {
                let signed = self.read_next_u8() as u16;
                self.registers.sp.wrapping_add(signed);
            }
            OpcodeUnprefixed::Stop => self.stopped = true,

            OpcodeUnprefixed::Jr(cond) => {
                let cond_satisfied = match cond {
                    opcode::Condition::Zero => self.registers.flags.zero,
                    opcode::Condition::NotZero => !self.registers.flags.zero,
                    opcode::Condition::Carry => self.registers.flags.carry,
                    opcode::Condition::NotCarry => !self.registers.flags.carry,
                    opcode::Condition::Unconditional => true,
                };
                let rel_addr = self.read_next_u8() as u16;
                if cond_satisfied {
                    self.registers.pc.wrapping_add(rel_addr);
                }
            }
            OpcodeUnprefixed::Daa => self.registers.decimal_adjust_a(),
            OpcodeUnprefixed::Cpl => {
                self.registers.a = self.registers.a.not();
            }
            OpcodeUnprefixed::Scf => {
                self.registers.flags.carry = true;
            }
            OpcodeUnprefixed::Ccf => {
                self.registers.flags.carry = self.registers.flags.carry.not();
            }
            OpcodeUnprefixed::Halt => todo!(),
            OpcodeUnprefixed::Adc(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };
                self.registers.a.wrapping_add(data).wrapping_add(1);
            }
            OpcodeUnprefixed::Sub(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };

                self.registers.a.wrapping_sub(data);
            }
            OpcodeUnprefixed::Sbc(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };
                let carry_bit = self.registers.flags.carry as u8;
                self.registers.a = self.registers.a.wrapping_sub(data).wrapping_sub(carry_bit);
            }
            OpcodeUnprefixed::And(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };
                self.registers.a &= data;
            }
            OpcodeUnprefixed::Xor(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };
                self.registers.a ^= data;
            }
            OpcodeUnprefixed::Or(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };
                self.registers.a |= data;
            }
            OpcodeUnprefixed::Cp(rhs) => {
                let data = match rhs {
                    Op8::A => self.registers.a,
                    Op8::B => self.registers.bc.hi,
                    Op8::C => self.registers.bc.lo,
                    Op8::D => self.registers.de.hi,
                    Op8::E => self.registers.de.lo,
                    Op8::H => self.registers.hl.hi,
                    Op8::L => self.registers.hl.lo,
                    Op8::AddrHL => self.read_byte(self.registers.hl.as_both()),
                    Op8::U8 => self.read_next_u8(),
                    _ => unimplemented!(),
                };
            }
            OpcodeUnprefixed::Ret(cond) => {
                if self.test_condition(cond) {
                    self.registers.sp = self.pop();
                }
            }
            OpcodeUnprefixed::Pop(target) => {
                let value = self.pop();
                match target {
                    Op16::BC => self.registers.bc.set_both(value),
                    Op16::DE => self.registers.de.set_both(value),
                    Op16::HL => self.registers.hl.set_both(value),
                    Op16::AF => self.registers.set_af(value),
                    _ => unimplemented!(),
                }
            }
            OpcodeUnprefixed::Jp(cond, source) => {
                let jump_addr = match source {
                    Op16::U16 => self.read_next_u16(),
                    Op16::HL => self.registers.hl.as_both(),
                    _ => unimplemented!(),
                };

                if self.test_condition(cond) {
                    self.jump(jump_addr);
                }
            }
            OpcodeUnprefixed::Call(cond) => {
                let jump_addr = self.read_next_u16();
                if self.test_condition(cond) {
                    self.push(self.registers.pc);
                    self.jump(jump_addr);
                }
            }
            OpcodeUnprefixed::Push(target) => {
                let value = self.pop();

                match target {
                    Op16::BC => self.registers.bc.set_both(value),
                    Op16::DE => self.registers.de.set_both(value),
                    Op16::HL => self.registers.hl.set_both(value),
                    Op16::AF => self.registers.set_af(value),
                    _ => unimplemented!(),
                }
            }
            OpcodeUnprefixed::Rst(interrupt_addr) => {
                self.push(self.registers.pc);
                self.jump(interrupt_addr);
            }
            OpcodeUnprefixed::Prefix => {
                let opcode = self.read_next_u8();
                let opcode_info = &opcode::SECONDARY[opcode as usize];
                match opcode_info.inst {
                    OpcodePrefixed::Swap(target) => match target {
                        Op8::A => self.registers.a.swap_nibbles(),
                        Op8::B => self.registers.bc.hi.swap_nibbles(),
                        Op8::C => self.registers.bc.lo.swap_nibbles(),
                        Op8::D => self.registers.de.hi.swap_nibbles(),
                        Op8::E => self.registers.de.lo.swap_nibbles(),
                        Op8::H => self.registers.hl.hi.swap_nibbles(),
                        Op8::L => self.registers.hl.lo.swap_nibbles(),
                        Op8::AddrHL => {
                            let addr = self.registers.hl.as_both();
                            let mut data = self.read_byte(addr);
                            data.swap_nibbles();
                            self.write_byte(addr, data);
                        }
                        _ => unimplemented!(),
                    },
                    OpcodePrefixed::Sra(target) => match target {
                        Op8::A => self.registers.a = self.registers.a.rotate_right(1),
                        Op8::B => self.registers.bc.hi = self.registers.bc.hi.rotate_right(1),
                        Op8::C => self.registers.bc.lo = self.registers.bc.lo.rotate_right(1),
                        Op8::D => self.registers.de.hi = self.registers.de.hi.rotate_right(1),
                        Op8::E => self.registers.de.lo = self.registers.de.lo.rotate_right(1),
                        Op8::H => self.registers.hl.hi = self.registers.hl.hi.rotate_right(1),
                        Op8::L => self.registers.hl.lo = self.registers.hl.lo.rotate_right(1),
                        Op8::AddrHL => {
                            let addr = self.registers.hl.as_both();
                            let data = self.read_byte(addr).rotate_right(1);
                            self.write_byte(addr, data);
                        }
                        _ => unimplemented!(),
                    },
                    OpcodePrefixed::Sla(target) => match target {
                        Op8::A => self.registers.a = self.registers.a.rotate_left(1),
                        Op8::B => self.registers.bc.hi = self.registers.bc.hi.rotate_left(1),
                        Op8::C => self.registers.bc.lo = self.registers.bc.lo.rotate_left(1),
                        Op8::D => self.registers.de.hi = self.registers.de.hi.rotate_left(1),
                        Op8::E => self.registers.de.lo = self.registers.de.lo.rotate_left(1),
                        Op8::H => self.registers.hl.hi = self.registers.hl.hi.rotate_left(1),
                        Op8::L => self.registers.hl.lo = self.registers.hl.lo.rotate_left(1),
                        Op8::AddrHL => {
                            let addr = self.registers.hl.as_both();
                            let data = self.read_byte(addr).rotate_left(1);
                            self.write_byte(addr, data);
                        }
                        _ => unimplemented!(),
                    },
                    OpcodePrefixed::Bit(bit, target) => {
                        let test = match target {
                            Op8::A => self.registers.a.get_bit(bit),
                            Op8::B => self.registers.bc.hi.get_bit(bit),
                            Op8::C => self.registers.bc.lo.get_bit(bit),
                            Op8::D => self.registers.de.hi.get_bit(bit),
                            Op8::E => self.registers.de.lo.get_bit(bit),
                            Op8::H => self.registers.hl.hi.get_bit(bit),
                            Op8::L => self.registers.hl.lo.get_bit(bit),
                            Op8::AddrHL => {
                                let addr = self.registers.hl.as_both();
                                self.read_byte(addr).get_bit(bit)
                            }
                            _ => unimplemented!(),
                        };
                    }
                    OpcodePrefixed::Set(bit, target) | OpcodePrefixed::Res(bit, target) => {
                        //true for SET, false for RES
                        let val_to_set = matches!(opcode_info.inst, OpcodePrefixed::Set(_, _));

                        match target {
                            Op8::A => self.registers.a.set_bit(bit, val_to_set),
                            Op8::B => self.registers.bc.hi.set_bit(bit, val_to_set),
                            Op8::C => self.registers.bc.lo.set_bit(bit, val_to_set),
                            Op8::D => self.registers.de.hi.set_bit(bit, val_to_set),
                            Op8::E => self.registers.de.lo.set_bit(bit, val_to_set),
                            Op8::H => self.registers.hl.hi.set_bit(bit, val_to_set),
                            Op8::L => self.registers.hl.lo.set_bit(bit, val_to_set),
                            Op8::AddrHL => {
                                let addr = self.registers.hl.as_both();
                                let mut data = self.read_byte(addr);
                                data.set_bit(bit, val_to_set);
                                self.write_byte(addr, data);
                            }
                            _ => unimplemented!(),
                        }
                    }
                    OpcodePrefixed::Rl(target) => {
                        let mut new_carry;
                        match target {
                            Op8::A => {
                                new_carry = self.registers.a.shr(7u8).bitor(1);
                                self.registers.a = self.registers.a.shl(1u8).bitor(new_carry);
                            }
                            Op8::B => {
                                new_carry = self.registers.bc.hi.shr(7u8).bitor(1);
                                self.registers.bc.hi =
                                    self.registers.bc.hi.shl(1u8).bitor(new_carry);
                            }
                            Op8::C => {
                                new_carry = self.registers.bc.lo.shr(7u8).bitor(1);
                                self.registers.bc.lo =
                                    self.registers.bc.lo.shl(1u8).bitor(new_carry);
                            }
                            Op8::D => {
                                new_carry = self.registers.de.hi.shr(7u8).bitor(1);
                                self.registers.de.hi =
                                    self.registers.de.hi.shl(1u8).bitor(new_carry);
                            }
                            Op8::E => {
                                new_carry = self.registers.de.lo.shr(7u8).bitor(1);
                                self.registers.de.lo =
                                    self.registers.de.lo.shl(1u8).bitor(new_carry);
                            }
                            Op8::H => {
                                new_carry = self.registers.hl.hi.shr(7u8).bitor(1);
                                self.registers.hl.hi =
                                    self.registers.hl.hi.shl(1u8).bitor(new_carry);
                            }
                            Op8::L => {
                                new_carry = self.registers.hl.lo.shr(7u8).bitand(1);
                                self.registers.hl.lo =
                                    self.registers.hl.lo.shl(1u8).bitor(new_carry);
                            }
                            Op8::AddrHL => {
                                let addr = self.registers.hl.as_both();
                                let mut data = self.read_byte(addr);
                                new_carry = data.shr(7u8).bitor(1);
                                data = data.shl(1u8).bitor(new_carry);
                                self.write_byte(addr, data);
                            }
                            _ => unimplemented!(),
                        }

                        self.registers.flags.carry = new_carry == 1;
                    }
                    OpcodePrefixed::Rr(target) => {
                        let mut new_carry;
                        match target {
                            Op8::A => {
                                new_carry = self.registers.a.bitand(1);
                                self.registers.a = self.registers.a.shr(1u8).bitor(new_carry);
                            }
                            Op8::B => {
                                new_carry = self.registers.bc.hi.bitand(1);
                                self.registers.bc.hi =
                                    self.registers.bc.hi.shr(1u8).bitor(new_carry);
                            }
                            Op8::C => {
                                new_carry = self.registers.bc.lo.bitand(1);
                                self.registers.bc.lo =
                                    self.registers.bc.lo.shr(1u8).bitor(new_carry);
                            }
                            Op8::D => {
                                new_carry = self.registers.de.hi.bitand(1);
                                self.registers.de.hi =
                                    self.registers.de.hi.shr(1u8).bitor(new_carry);
                            }
                            Op8::E => {
                                new_carry = self.registers.de.lo.bitand(1);
                                self.registers.de.lo =
                                    self.registers.de.lo.shr(1u8).bitor(new_carry);
                            }
                            Op8::H => {
                                new_carry = self.registers.hl.hi.bitand(1);
                                self.registers.hl.hi =
                                    self.registers.hl.hi.shr(1u8).bitor(new_carry);
                            }
                            Op8::L => {
                                new_carry = self.registers.hl.lo.bitand(1);
                                self.registers.hl.lo =
                                    self.registers.hl.lo.shr(1u8).bitor(new_carry);
                            }
                            Op8::AddrHL => {
                                let addr = self.registers.hl.as_both();
                                let mut data = self.read_byte(addr);
                                new_carry = data.bitand(1);
                                data = data.shr(1u8).bitor(new_carry);
                                self.write_byte(addr, data);
                            }
                            _ => unimplemented!(),
                        }

                        self.registers.flags.carry = new_carry == 1;
                    }
                    OpcodePrefixed::Rlc(target) => match target {
                        Op8::A => {
                            self.registers.flags.carry = self.registers.a.shr(7u8) == 1;
                            self.registers.a = self.registers.a.rotate_left(1);
                        }
                        Op8::B => {
                            self.registers.flags.carry = self.registers.bc.hi.shr(7u8) == 1;
                            self.registers.bc.hi = self.registers.bc.hi.rotate_left(1);
                        }
                        Op8::C => {
                            self.registers.flags.carry = self.registers.bc.lo.shr(7u8) == 1;
                            self.registers.bc.lo = self.registers.bc.lo.rotate_left(1);
                        }
                        Op8::D => {
                            self.registers.flags.carry = self.registers.de.hi.shr(7u8) == 1;
                            self.registers.de.hi = self.registers.de.hi.rotate_left(1);
                        }
                        Op8::E => {
                            self.registers.flags.carry = self.registers.de.lo.shr(7u8) == 1;
                            self.registers.de.lo = self.registers.de.lo.rotate_left(1);
                        }
                        Op8::H => {
                            self.registers.flags.carry = self.registers.hl.hi.shr(7u8) == 1;
                            self.registers.hl.hi = self.registers.hl.hi.rotate_left(1);
                        }
                        Op8::L => {
                            self.registers.flags.carry = self.registers.hl.lo.shr(7u8) == 1;
                            self.registers.hl.lo = self.registers.hl.lo.rotate_left(1);
                        }
                        Op8::AddrHL => {
                            let addr = self.registers.hl.as_both();
                            let mut data = self.read_byte(addr);
                            self.registers.flags.carry = data.shr(7u8) == 1;
                            data = data.rotate_left(1);
                            self.write_byte(addr, data);
                        }
                        _ => unimplemented!(),
                    },
                    OpcodePrefixed::Rrc(target) => match target {
                        Op8::A => {
                            self.registers.flags.carry = self.registers.a.bitand(1) == 1;
                            self.registers.a = self.registers.a.rotate_right(1);
                        }
                        Op8::B => {
                            self.registers.flags.carry = self.registers.bc.hi.bitand(1) == 1;
                            self.registers.bc.hi = self.registers.bc.hi.rotate_right(1);
                        }
                        Op8::C => {
                            self.registers.flags.carry = self.registers.bc.lo.bitand(1) == 1;
                            self.registers.bc.lo = self.registers.bc.lo.rotate_right(1);
                        }
                        Op8::D => {
                            self.registers.flags.carry = self.registers.de.hi.bitand(1) == 1;
                            self.registers.de.hi = self.registers.de.hi.rotate_right(1);
                        }
                        Op8::E => {
                            self.registers.flags.carry = self.registers.de.lo.bitand(1) == 1;
                            self.registers.de.lo = self.registers.de.lo.rotate_right(1);
                        }
                        Op8::H => {
                            self.registers.flags.carry = self.registers.hl.hi.bitand(1) == 1;
                            self.registers.hl.hi = self.registers.hl.hi.rotate_right(1);
                        }
                        Op8::L => {
                            self.registers.flags.carry = self.registers.hl.lo.bitand(1) == 1;
                            self.registers.hl.lo = self.registers.hl.lo.rotate_right(1);
                        }
                        Op8::AddrHL => {
                            let addr = self.registers.hl.as_both();
                            let mut data = self.read_byte(addr);
                            self.registers.flags.carry = data.bitand(1) == 1;
                            data = data.rotate_right(1);
                            self.write_byte(addr, data);
                        }
                        _ => unimplemented!(),
                    },
                    OpcodePrefixed::Srl(target) => match target {
                        Op8::A => self.registers.a.shr_assign(1),
                        Op8::B => self.registers.bc.hi.shr_assign(1),
                        Op8::C => self.registers.bc.lo.shr_assign(1),
                        Op8::D => self.registers.de.hi.shr_assign(1),
                        Op8::E => self.registers.de.lo.shr_assign(1),
                        Op8::H => self.registers.hl.hi.shr_assign(1),
                        Op8::L => self.registers.hl.lo.shr_assign(1),
                        Op8::AddrHL => {
                            let addr = self.registers.hl.as_both();
                            let data = self.read_byte(addr).shr(1);
                            self.write_byte(addr, data);
                        }
                        _ => unimplemented!(),
                    },
                }
            }
            OpcodeUnprefixed::Reti => todo!(),
            OpcodeUnprefixed::Di => todo!(),
            OpcodeUnprefixed::Ei => todo!(),

            OpcodeUnprefixed::Illegal => panic!("got illegal command"),

            OpcodeUnprefixed::Rlca => {
                self.registers.flags.carry = self.registers.a.shr(7u8) == 1;
                self.registers.a = self.registers.a.rotate_left(1);
            }
            OpcodeUnprefixed::Rrca => {
                self.registers.flags.carry = self.registers.a.bitand(1) == 1;
                self.registers.a = self.registers.a.rotate_right(1);
            }
            OpcodeUnprefixed::Rla => {
                let mut new_carry = self.registers.a.shr(7u8).bitor(1);
                self.registers.a = self.registers.a.shl(1u8).bitor(new_carry);
            }
            OpcodeUnprefixed::Rra => {
                let mut new_carry = self.registers.a.bitand(1);
                self.registers.a = self.registers.a.shr(1u8).bitor(new_carry);
            }
        }
    }

    fn test_condition(&self, cond: opcode::Condition) -> bool {
        match cond {
            opcode::Condition::Zero => self.registers.flags.zero,
            opcode::Condition::NotZero => !self.registers.flags.zero,
            opcode::Condition::Carry => self.registers.flags.carry,
            opcode::Condition::NotCarry => !self.registers.flags.carry,
            opcode::Condition::Unconditional => true,
        }
    }

    pub fn jump(&mut self, addr: u16) {
        self.registers.pc = addr;
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Shr;

    use super::*;

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

    #[test]
    fn f_reg() {
        let mut regs = Registers::default();
        regs.flags.zero = false;
        regs.flags.neg = true;
        regs.flags.half_carry = true;
        regs.flags.carry = false;

        println!("{:b}", regs.flags.f_reg())
    }

    #[test]
    fn read_as_i16() {
        let signed = (-19_i8) as u16;
        println!("{}", 54_u16.wrapping_add(signed));
    }

    #[test]
    fn u8_overflow() {
        let n1 = 220_u8;
        let n2 = 50_u8;
        println!("{}", n1.wrapping_add(n2));
    }

    #[test]
    fn shifting() {
        let n = 0b1001_0111_u8;
        println!("{:08b}", n.rotate_right(1));
        println!("{:08b}", n.shr(1));
    }

    #[test]
    fn add_consuming() {
        let mut x: u8 = 0xFF;
        x.wrapping_add(1);
        println!("{}", x);

        assert_eq!(x, 0);
    }
}
