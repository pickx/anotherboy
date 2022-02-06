// use serde::{Deserialize, Serialize};
// use serde_json::Result;

// #[derive(Serialize, Deserialize)]

use std::ops::Add;

use Condition::*;
use Opcode::*;
use Operand16::*;
use Operand8::*;

#[rustfmt::skip]
pub const LOOKUP: [OpInfo; 256] = [
    OpInfo { mnemonic: "NOP", cycles: 4, cycles_no_branch: 0, inst: Nop }, //0x00
    OpInfo { mnemonic: "LD BC,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(BC, U16) }, //0x01
    OpInfo { mnemonic: "LD (BC),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrBC, A) }, //0x02
    OpInfo { mnemonic: "INC BC", cycles: 8, cycles_no_branch: 0, inst: Inc16(BC) }, //0x03
    OpInfo { mnemonic: "INC B", cycles: 4, cycles_no_branch: 0, inst:Inc8(B) }, //0x04
    OpInfo { mnemonic: "DEC B", cycles: 4, cycles_no_branch: 0, inst: Dec8(B) }, //0x05
    OpInfo { mnemonic: "LD B,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(B, U8) }, //0x06
    OpInfo { mnemonic: "RLCA", cycles: 4, cycles_no_branch: 0, inst: Rlc(A) }, //0x07
    OpInfo { mnemonic: "LD (u16),SP", cycles: 20, cycles_no_branch: 0, inst: LdU16StackAddr }, //0x08
    OpInfo { mnemonic: "ADD HL,BC", cycles: 8, cycles_no_branch: 0, inst: Add16(BC) }, //0x09
    OpInfo { mnemonic: "LD A,(BC)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrBC) }, //0x0A
    OpInfo { mnemonic: "DEC BC", cycles: 8, cycles_no_branch: 0, inst: Dec16(BC) }, //0x0B
    OpInfo { mnemonic: "INC C", cycles: 4, cycles_no_branch: 0, inst: Inc8(C) }, //0x0C
    OpInfo { mnemonic: "DEC C", cycles: 4, cycles_no_branch: 0, inst: Dec8(C) }, //0x0D
    OpInfo { mnemonic: "LD C,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(C, U8) }, //0x0E
    OpInfo { mnemonic: "RRCA", cycles: 4, cycles_no_branch: 0, inst: Rrc(A) }, //0x0F
    OpInfo { mnemonic: "STOP u8", cycles: 4, cycles_no_branch: 0, inst: Stop }, //0x10
    OpInfo { mnemonic: "LD DE,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(DE, U16) }, //0x11
    OpInfo { mnemonic: "LD (DE),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrDE, A) }, //0x12
    OpInfo { mnemonic: "INC DE", cycles: 8, cycles_no_branch: 0, inst: Inc16(DE) }, //0x13
    OpInfo { mnemonic: "INC D", cycles: 4, cycles_no_branch: 0, inst: Inc8(D) }, //0x14
    OpInfo { mnemonic: "DEC D", cycles: 4, cycles_no_branch: 0, inst: Dec8(D) }, //0x15
    OpInfo { mnemonic: "LD D,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(D, U8) }, //0x16
    OpInfo { mnemonic: "RLA", cycles: 4, cycles_no_branch: 0, inst: Rl(A) }, //0x17
    OpInfo { mnemonic: "JR r8", cycles: 12, cycles_no_branch: 0, inst: Jr(Unconditional) }, //0x18
    OpInfo { mnemonic: "ADD HL,DE", cycles: 8, cycles_no_branch: 0, inst: Add16(DE) }, //0x19
    OpInfo { mnemonic: "LD A,(DE)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrDE) }, //0x1A
    OpInfo { mnemonic: "DEC DE", cycles: 8, cycles_no_branch: 0, inst: Dec16(DE) }, //0x1B
    OpInfo { mnemonic: "INC E", cycles: 4, cycles_no_branch: 0, inst: Inc8(E) }, //0x1C
    OpInfo { mnemonic: "DEC E", cycles: 4, cycles_no_branch: 0, inst: Dec8(E) }, //0x1D
    OpInfo { mnemonic: "LD E,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(E, U8) }, //0x1E
    OpInfo { mnemonic: "RRA", cycles: 4, cycles_no_branch: 0, inst: Rr(A) }, //0x1F
    OpInfo { mnemonic: "JR NZ,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(NotZero) }, //0x20
    OpInfo { mnemonic: "LD HL,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(HL, U16) }, //0x21
    OpInfo { mnemonic: "LD (HL+),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHLInc, A)}, //0x22
    OpInfo { mnemonic: "INC HL", cycles: 8, cycles_no_branch: 0, inst: Inc16(HL) }, //0x23
    OpInfo { mnemonic: "INC H", cycles: 4, cycles_no_branch: 0, inst: Inc8(H) }, //0x24
    OpInfo { mnemonic: "DEC H", cycles: 4, cycles_no_branch: 0, inst: Dec8(H) }, //0x25
    OpInfo { mnemonic: "LD H,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(H, U8) }, //0x26
    OpInfo { mnemonic: "DAA", cycles: 4, cycles_no_branch: 0, inst: Daa }, //0x27
    OpInfo { mnemonic: "JR Z,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(Zero) }, //0x28
    OpInfo { mnemonic: "ADD HL,HL", cycles: 8, cycles_no_branch: 0, inst: Add16(HL) }, //0x29
    OpInfo { mnemonic: "LD A,(HL+)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrHLInc) }, //0x2A
    OpInfo { mnemonic: "DEC HL", cycles: 8, cycles_no_branch: 0, inst: Dec16(HL) }, //0x2B
    OpInfo { mnemonic: "INC L", cycles: 4, cycles_no_branch: 0, inst: Inc8(L) }, //0x2C
    OpInfo { mnemonic: "DEC L", cycles: 4, cycles_no_branch: 0, inst: Dec8(L) }, //0x2D
    OpInfo { mnemonic: "LD L,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(L, U8) }, //0x2E
    OpInfo { mnemonic: "CPL", cycles: 4, cycles_no_branch: 0, inst: Cpl }, //0x2F
    OpInfo { mnemonic: "JR NC,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(NotCarry) }, //0x30
    OpInfo { mnemonic: "LD SP,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(SP, U16) }, //0x31
    OpInfo { mnemonic: "LD (HL-),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHLDec, A) }, //0x32
    OpInfo { mnemonic: "INC SP", cycles: 8, cycles_no_branch: 0, inst: Inc16(SP) }, //0x33
    OpInfo { mnemonic: "INC (HL)", cycles: 12, cycles_no_branch: 0, inst: Inc8(AddrHL) }, //0x34
    OpInfo { mnemonic: "DEC (HL)", cycles: 12, cycles_no_branch: 0, inst: Dec16(HL) }, //0x35
    OpInfo { mnemonic: "LD (HL),u8", cycles: 12, cycles_no_branch: 0, inst: Ld8(AddrHL, U8) }, //0x36
    OpInfo { mnemonic: "SCF", cycles: 4, cycles_no_branch: 0, inst: Scf }, //0x37
    OpInfo { mnemonic: "JR C,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(Carry) }, //0x38
    OpInfo { mnemonic: "ADD HL,SP", cycles: 8, cycles_no_branch: 0, inst: Add16(SP) }, //0x39
    OpInfo { mnemonic: "LD A,(HL-)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrHLDec) }, //0x3A
    OpInfo { mnemonic: "DEC SP", cycles: 8, cycles_no_branch: 0, inst: Dec16(SP) }, //0x3B
    OpInfo { mnemonic: "INC A", cycles: 4, cycles_no_branch: 0, inst: Inc8(A) }, //0x3C
    OpInfo { mnemonic: "DEC A", cycles: 4, cycles_no_branch: 0, inst: Dec8(A) }, //0x3D
    OpInfo { mnemonic: "LD A,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, U8) }, //0x3E
    OpInfo { mnemonic: "CCF", cycles: 4, cycles_no_branch: 0, inst: Ccf }, //0x3F
    OpInfo { mnemonic: "LD B,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, B) }, //0x40
    OpInfo { mnemonic: "LD B,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, C) }, //0x41
    OpInfo { mnemonic: "LD B,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, D) }, //0x42
    OpInfo { mnemonic: "LD B,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, E) }, //0x43
    OpInfo { mnemonic: "LD B,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, H) }, //0x44
    OpInfo { mnemonic: "LD B,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, L) }, //0x45
    OpInfo { mnemonic: "LD B,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(B, AddrHL) }, //0x46
    OpInfo { mnemonic: "LD B,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, A) }, //0x47
    OpInfo { mnemonic: "LD C,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, B) }, //0x48
    OpInfo { mnemonic: "LD C,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, C) }, //0x49
    OpInfo { mnemonic: "LD C,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, D) }, //0x4A
    OpInfo { mnemonic: "LD C,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, E) }, //0x4B
    OpInfo { mnemonic: "LD C,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, H) }, //0x4C
    OpInfo { mnemonic: "LD C,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, L) }, //0x4D
    OpInfo { mnemonic: "LD C,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(C, AddrHL) }, //0x4E
    OpInfo { mnemonic: "LD C,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, A) }, //0x4F
    OpInfo { mnemonic: "LD D,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, B) }, //0x50
    OpInfo { mnemonic: "LD D,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, C) }, //0x51
    OpInfo { mnemonic: "LD D,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, D) }, //0x52
    OpInfo { mnemonic: "LD D,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, E) }, //0x53
    OpInfo { mnemonic: "LD D,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, H) }, //0x54
    OpInfo { mnemonic: "LD D,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, L) }, //0x55
    OpInfo { mnemonic: "LD D,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(D, AddrHL) }, //0x56
    OpInfo { mnemonic: "LD D,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, A) }, //0x57
    OpInfo { mnemonic: "LD E,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, B) }, //0x58
    OpInfo { mnemonic: "LD E,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, C) }, //0x59
    OpInfo { mnemonic: "LD E,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, D) }, //0x5A
    OpInfo { mnemonic: "LD E,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, E) }, //0x5B
    OpInfo { mnemonic: "LD E,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, H) }, //0x5C
    OpInfo { mnemonic: "LD E,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, L) }, //0x5D
    OpInfo { mnemonic: "LD E,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(E, AddrHL) }, //0x5E
    OpInfo { mnemonic: "LD E,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, A) }, //0x5F
    OpInfo { mnemonic: "LD H,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, B) }, //0x60
    OpInfo { mnemonic: "LD H,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, C) }, //0x61
    OpInfo { mnemonic: "LD H,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, D) }, //0x62
    OpInfo { mnemonic: "LD H,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, E) }, //0x63
    OpInfo { mnemonic: "LD H,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, H) }, //0x64
    OpInfo { mnemonic: "LD H,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, L) }, //0x65
    OpInfo { mnemonic: "LD H,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(H, AddrHL) }, //0x66
    OpInfo { mnemonic: "LD H,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, A) }, //0x67
    OpInfo { mnemonic: "LD L,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, B) }, //0x68
    OpInfo { mnemonic: "LD L,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, C) }, //0x69
    OpInfo { mnemonic: "LD L,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, D) }, //0x6A
    OpInfo { mnemonic: "LD L,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, E) }, //0x6B
    OpInfo { mnemonic: "LD L,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, H) }, //0x6C
    OpInfo { mnemonic: "LD L,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, L) }, //0x6D
    OpInfo { mnemonic: "LD L,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(L, AddrHL) }, //0x6E
    OpInfo { mnemonic: "LD L,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, A) }, //0x6F
    OpInfo { mnemonic: "LD (HL),B", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, B) }, //0x70
    OpInfo { mnemonic: "LD (HL),C", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, C) }, //0x71
    OpInfo { mnemonic: "LD (HL),D", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, D) }, //0x72
    OpInfo { mnemonic: "LD (HL),E", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, E) }, //0x73
    OpInfo { mnemonic: "LD (HL),H", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, H) }, //0x74
    OpInfo { mnemonic: "LD (HL),L", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, L) }, //0x75
    OpInfo { mnemonic: "HALT", cycles: 4, cycles_no_branch: 0, inst: Halt }, //0x76
    OpInfo { mnemonic: "LD (HL),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, A) }, //0x77
    OpInfo { mnemonic: "LD A,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, B) }, //0x78
    OpInfo { mnemonic: "LD A,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, C) }, //0x79
    OpInfo { mnemonic: "LD A,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, D) }, //0x7A
    OpInfo { mnemonic: "LD A,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, E) }, //0x7B
    OpInfo { mnemonic: "LD A,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, H) }, //0x7C
    OpInfo { mnemonic: "LD A,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, L) }, //0x7D
    OpInfo { mnemonic: "LD A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrHL) }, //0x7E
    OpInfo { mnemonic: "LD A,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, A) }, //0x7F
    OpInfo { mnemonic: "ADD A,B", cycles: 4, cycles_no_branch: 0, inst: Add8(B) }, //0x80
    OpInfo { mnemonic: "ADD A,C", cycles: 4, cycles_no_branch: 0, inst: Add8(C) }, //0x81
    OpInfo { mnemonic: "ADD A,D", cycles: 4, cycles_no_branch: 0, inst: Add8(D) }, //0x82
    OpInfo { mnemonic: "ADD A,E", cycles: 4, cycles_no_branch: 0, inst: Add8(E) }, //0x83
    OpInfo { mnemonic: "ADD A,H", cycles: 4, cycles_no_branch: 0, inst: Add8(H) }, //0x84
    OpInfo { mnemonic: "ADD A,L", cycles: 4, cycles_no_branch: 0, inst: Add8(L) }, //0x85
    OpInfo { mnemonic: "ADD A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Add8(AddrHL) }, //0x86
    OpInfo { mnemonic: "ADD A,A", cycles: 4, cycles_no_branch: 0, inst: Add8(A) }, //0x87
    OpInfo { mnemonic: "ADC A,B", cycles: 4, cycles_no_branch: 0, inst: Adc(B) }, //0x88
    OpInfo { mnemonic: "ADC A,C", cycles: 4, cycles_no_branch: 0, inst: Adc(C) }, //0x89
    OpInfo { mnemonic: "ADC A,D", cycles: 4, cycles_no_branch: 0, inst: Adc(D) }, //0x8A
    OpInfo { mnemonic: "ADC A,E", cycles: 4, cycles_no_branch: 0, inst: Adc(E) }, //0x8B
    OpInfo { mnemonic: "ADC A,H", cycles: 4, cycles_no_branch: 0, inst: Adc(H) }, //0x8C
    OpInfo { mnemonic: "ADC A,L", cycles: 4, cycles_no_branch: 0, inst: Adc(L) }, //0x8D
    OpInfo { mnemonic: "ADC A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Adc(AddrHL) }, //0x8E
    OpInfo { mnemonic: "ADC A,A", cycles: 4, cycles_no_branch: 0, inst: Adc(A) }, //0x8F
    OpInfo { mnemonic: "SUB B", cycles: 4, cycles_no_branch: 0, inst: Sub(B) }, //0x90
    OpInfo { mnemonic: "SUB C", cycles: 4, cycles_no_branch: 0, inst: Sub(C) }, //0x91
    OpInfo { mnemonic: "SUB D", cycles: 4, cycles_no_branch: 0, inst: Sub(D) }, //0x92
    OpInfo { mnemonic: "SUB E", cycles: 4, cycles_no_branch: 0, inst: Sub(E) }, //0x93
    OpInfo { mnemonic: "SUB H", cycles: 4, cycles_no_branch: 0, inst: Sub(H) }, //0x94
    OpInfo { mnemonic: "SUB L", cycles: 4, cycles_no_branch: 0, inst: Sub(L) }, //0x95
    OpInfo { mnemonic: "SUB (HL)", cycles: 8, cycles_no_branch: 0, inst: Sub(AddrHL) }, //0x96
    OpInfo { mnemonic: "SUB A", cycles: 4, cycles_no_branch: 0, inst: Sub(A) }, //0x97
    OpInfo { mnemonic: "SBC A,B", cycles: 4, cycles_no_branch: 0, inst: Sbc(B) }, //0x98
    OpInfo { mnemonic: "SBC A,C", cycles: 4, cycles_no_branch: 0, inst: Sbc(C) }, //0x99
    OpInfo { mnemonic: "SBC A,D", cycles: 4, cycles_no_branch: 0, inst: Sbc(D) }, //0x9A
    OpInfo { mnemonic: "SBC A,E", cycles: 4, cycles_no_branch: 0, inst: Sbc(E) }, //0x9B
    OpInfo { mnemonic: "SBC A,H", cycles: 4, cycles_no_branch: 0, inst: Sbc(H) }, //0x9C
    OpInfo { mnemonic: "SBC A,L", cycles: 4, cycles_no_branch: 0, inst: Sbc(L) }, //0x9D
    OpInfo { mnemonic: "SBC A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Sbc(AddrHL) }, //0x9E
    OpInfo { mnemonic: "SBC A,A", cycles: 4, cycles_no_branch: 0, inst: Sbc(A) }, //0x9F
    OpInfo { mnemonic: "AND B", cycles: 4, cycles_no_branch: 0, inst: And(B) }, //0xA0
    OpInfo { mnemonic: "AND C", cycles: 4, cycles_no_branch: 0, inst: And(C) }, //0xA1
    OpInfo { mnemonic: "AND D", cycles: 4, cycles_no_branch: 0, inst: And(D) }, //0xA2
    OpInfo { mnemonic: "AND E", cycles: 4, cycles_no_branch: 0, inst: And(E) }, //0xA3
    OpInfo { mnemonic: "AND H", cycles: 4, cycles_no_branch: 0, inst: And(H) }, //0xA4
    OpInfo { mnemonic: "AND L", cycles: 4, cycles_no_branch: 0, inst: And(L) }, //0xA5
    OpInfo { mnemonic: "AND (HL)", cycles: 8, cycles_no_branch: 0, inst: And(AddrHL) }, //0xA6
    OpInfo { mnemonic: "AND A", cycles: 4, cycles_no_branch: 0, inst: And(A) }, //0xA7
    OpInfo { mnemonic: "XOR B", cycles: 4, cycles_no_branch: 0, inst: Xor(B) }, //0xA8
    OpInfo { mnemonic: "XOR C", cycles: 4, cycles_no_branch: 0, inst: Xor(C) }, //0xA9
    OpInfo { mnemonic: "XOR D", cycles: 4, cycles_no_branch: 0, inst: Xor(D) }, //0xAA
    OpInfo { mnemonic: "XOR E", cycles: 4, cycles_no_branch: 0, inst: Xor(E) }, //0xAB
    OpInfo { mnemonic: "XOR H", cycles: 4, cycles_no_branch: 0, inst: Xor(H) }, //0xAC
    OpInfo { mnemonic: "XOR L", cycles: 4, cycles_no_branch: 0, inst: Xor(L) }, //0xAD
    OpInfo { mnemonic: "XOR (HL)", cycles: 8, cycles_no_branch: 0, inst: Xor(AddrHL) }, //0xAE
    OpInfo { mnemonic: "XOR A", cycles: 4, cycles_no_branch: 0, inst: Xor(A) }, //0xAF
    OpInfo { mnemonic: "OR B", cycles: 4, cycles_no_branch: 0, inst: Or(B) }, //0xB0
    OpInfo { mnemonic: "OR C", cycles: 4, cycles_no_branch: 0, inst: Or(C) }, //0xB1
    OpInfo { mnemonic: "OR D", cycles: 4, cycles_no_branch: 0, inst: Or(D) }, //0xB2
    OpInfo { mnemonic: "OR E", cycles: 4, cycles_no_branch: 0, inst: Or(E) }, //0xB3
    OpInfo { mnemonic: "OR H", cycles: 4, cycles_no_branch: 0, inst: Or(H) }, //0xB4
    OpInfo { mnemonic: "OR L", cycles: 4, cycles_no_branch: 0, inst: Or(L) }, //0xB5
    OpInfo { mnemonic: "OR (HL)", cycles: 8, cycles_no_branch: 0, inst: Or(AddrHL) }, //0xB6
    OpInfo { mnemonic: "OR A", cycles: 4, cycles_no_branch: 0, inst: Or(A) }, //0xB7
    OpInfo { mnemonic: "CP B", cycles: 4, cycles_no_branch: 0, inst: Cp(B) }, //0xB8
    OpInfo { mnemonic: "CP C", cycles: 4, cycles_no_branch: 0, inst: Cp(C) }, //0xB9
    OpInfo { mnemonic: "CP D", cycles: 4, cycles_no_branch: 0, inst: Cp(D) }, //0xBA
    OpInfo { mnemonic: "CP E", cycles: 4, cycles_no_branch: 0, inst: Cp(E) }, //0xBB
    OpInfo { mnemonic: "CP H", cycles: 4, cycles_no_branch: 0, inst: Cp(H) }, //0xBC
    OpInfo { mnemonic: "CP L", cycles: 4, cycles_no_branch: 0, inst: Cp(L) }, //0xBD
    OpInfo { mnemonic: "CP (HL)", cycles: 8, cycles_no_branch: 0, inst: Cp(AddrHL) }, //0xBE
    OpInfo { mnemonic: "CP A", cycles: 4, cycles_no_branch: 0, inst: Cp(A) }, //0xBF
    OpInfo { mnemonic: "RET NZ", cycles: 20, cycles_no_branch: 8, inst: Ret(NotZero) }, //0xC0
    OpInfo { mnemonic: "POP BC", cycles: 12, cycles_no_branch: 0, inst: Pop(BC) }, //0xC1
    OpInfo { mnemonic: "JP NZ,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(NotZero, U16) }, //0xC2
    OpInfo { mnemonic: "JP u16", cycles: 16, cycles_no_branch: 0, inst: Jp(Unconditional, U16) }, //0xC3
    OpInfo { mnemonic: "CALL NZ,u16", cycles: 24, cycles_no_branch: 12, inst: Call(NotZero) }, //0xC4
    OpInfo { mnemonic: "PUSH BC", cycles: 16, cycles_no_branch: 0, inst: Push(BC) }, //0xC5
    OpInfo { mnemonic: "ADD A,u8", cycles: 8, cycles_no_branch: 0, inst: Add8(U8) }, //0xC6
    OpInfo { mnemonic: "RST 00H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x00) }, //0xC7
    OpInfo { mnemonic: "RET Z", cycles: 20, cycles_no_branch: 8, inst: Ret(Zero) }, //0xC8
    OpInfo { mnemonic: "RET", cycles: 16, cycles_no_branch: 0, inst: Ret(Unconditional), }, //0xC9
    OpInfo { mnemonic: "JP Z,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(Zero, U16) }, //0xCA
    OpInfo { mnemonic: "PREFIX", cycles: 4, cycles_no_branch: 0, inst: Prefix }, //0xCB
    OpInfo { mnemonic: "CALL Z,u16", cycles: 24, cycles_no_branch: 12, inst: Call(Zero) }, //0xCC
    OpInfo { mnemonic: "CALL u16", cycles: 24, cycles_no_branch: 0, inst: Call(Unconditional), }, //0xCD
    OpInfo { mnemonic: "ADC A,u8", cycles: 8, cycles_no_branch: 0, inst: Adc(U8) }, //0xCE
    OpInfo { mnemonic: "RST 08H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x08) }, //0xCF
    OpInfo { mnemonic: "RET NC", cycles: 20, cycles_no_branch: 8, inst: Ret(NotCarry) }, //0xD0
    OpInfo { mnemonic: "POP DE", cycles: 12, cycles_no_branch: 0, inst: Pop(DE) }, //0xD1
    OpInfo { mnemonic: "JP NC,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(NotCarry, U16) }, //0xD2
    OpInfo { mnemonic: "ILLEGAL_D3", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xD3
    OpInfo { mnemonic: "CALL NC,u16", cycles: 24, cycles_no_branch: 12, inst: Call(NotCarry) }, //0xD4
    OpInfo { mnemonic: "PUSH DE", cycles: 16, cycles_no_branch: 0, inst: Push(DE) }, //0xD5
    OpInfo { mnemonic: "SUB u8", cycles: 8, cycles_no_branch: 0, inst: Sub(U8) }, //0xD6
    OpInfo { mnemonic: "RST 10H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x10) }, //0xD7
    OpInfo { mnemonic: "RET C", cycles: 20, cycles_no_branch: 8, inst: Ret(Carry) }, //0xD8
    OpInfo { mnemonic: "RETI", cycles: 16, cycles_no_branch: 0, inst: Reti }, //0xD9
    OpInfo { mnemonic: "JP C,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(Carry, U16) }, //0xDA
    OpInfo { mnemonic: "ILLEGAL_DB", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xDB
    OpInfo { mnemonic: "CALL C,u16", cycles: 24, cycles_no_branch: 12, inst: Call(Carry) }, //0xDC
    OpInfo { mnemonic: "ILLEGAL_DD", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xDD
    OpInfo { mnemonic: "SBC A,u8", cycles: 8, cycles_no_branch: 0, inst: Sbc(U8) }, //0xDE
    OpInfo { mnemonic: "RST 18H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x18) }, //0xDF
    OpInfo { mnemonic: "LDH (a8),A", cycles: 12, cycles_no_branch: 0, inst: Ld8(LowAddrU8, A) }, //0xE0
    OpInfo { mnemonic: "POP HL", cycles: 12, cycles_no_branch: 0, inst: Pop(HL) }, //0xE1
    OpInfo { mnemonic: "LD (C),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(LowAddrC, A) }, //0xE2
    OpInfo { mnemonic: "ILLEGAL_E3", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xE3
    OpInfo { mnemonic: "ILLEGAL_E4", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xE4
    OpInfo { mnemonic: "PUSH HL", cycles: 16, cycles_no_branch: 0, inst: Push(HL) }, //0xE5
    OpInfo { mnemonic: "AND u8", cycles: 8, cycles_no_branch: 0, inst: And(U8) }, //0xE6
    OpInfo { mnemonic: "RST 20H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x20) }, //0xE7
    OpInfo { mnemonic: "ADD SP,r8", cycles: 16, cycles_no_branch: 0, inst: AddI8SP }, //0xE8
    OpInfo { mnemonic: "JP HL", cycles: 4, cycles_no_branch: 0, inst: Jp(Unconditional, HL) }, //0xE9
    OpInfo { mnemonic: "LD (u16),A", cycles: 16, cycles_no_branch: 0, inst: Ld8(AddrU16, A) }, //0xEA
    OpInfo { mnemonic: "ILLEGAL_EB", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xEB
    OpInfo { mnemonic: "ILLEGAL_EC", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xEC
    OpInfo { mnemonic: "ILLEGAL_ED", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xED
    OpInfo { mnemonic: "XOR u8", cycles: 8, cycles_no_branch: 0, inst: Xor(U8) }, //0xEE
    OpInfo { mnemonic: "RST 28H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x28) }, //0xEF
    OpInfo { mnemonic: "LDH A,(a8)", cycles: 12, cycles_no_branch: 0, inst: Ld8(A, LowAddrU8) }, //0xF0
    OpInfo { mnemonic: "POP AF", cycles: 12, cycles_no_branch: 0, inst: Pop(AF) }, //0xF1
    OpInfo { mnemonic: "LD A,(C)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, LowAddrC)}, //0xF2
    OpInfo { mnemonic: "DI", cycles: 4, cycles_no_branch: 0, inst: Di }, //0xF3
    OpInfo { mnemonic: "ILLEGAL_F4", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xF4
    OpInfo { mnemonic: "PUSH AF", cycles: 16, cycles_no_branch: 0, inst: Push(AF) }, //0xF5
    OpInfo { mnemonic: "OR u8", cycles: 8, cycles_no_branch: 0, inst: Or(U8) }, //0xF6
    OpInfo { mnemonic: "RST 30H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x30) }, //0xF7
    OpInfo { mnemonic: "LD HL,SP+r8", cycles: 12, cycles_no_branch: 0, inst: Ld16(HL, SPPlusI8) }, //0xF8
    OpInfo { mnemonic: "LD SP,HL", cycles: 8, cycles_no_branch: 0, inst: Ld16(SP, HL) }, //0xF9
    OpInfo { mnemonic: "LD A,(u16)", cycles: 16, cycles_no_branch: 0, inst: Ld8(A, AddrU16) }, //0xFA
    OpInfo { mnemonic: "EI", cycles: 4, cycles_no_branch: 0, inst: Ei }, //0xFB
    OpInfo { mnemonic: "ILLEGAL_FC", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xFC
    OpInfo { mnemonic: "ILLEGAL_FD", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xFD
    OpInfo { mnemonic: "CP u8", cycles: 8, cycles_no_branch: 0, inst: Cp(U8) }, //0xFE
    OpInfo { mnemonic: "RST 38H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x38) }, //0xFF
];

#[rustfmt::skip]
const SECONDARY: [OpInfo; 256] = [
    OpInfo { mnemonic: "RLC B", cycles: 8, cycles_no_branch: 0, inst: Rlc(B) }, //0x00
    OpInfo { mnemonic: "RLC C", cycles: 8, cycles_no_branch: 0, inst: Rlc(C) }, //0x01
    OpInfo { mnemonic: "RLC D", cycles: 8, cycles_no_branch: 0, inst: Rlc(D) }, //0x02
    OpInfo { mnemonic: "RLC E", cycles: 8, cycles_no_branch: 0, inst: Rlc(E) }, //0x03
    OpInfo { mnemonic: "RLC H", cycles: 8, cycles_no_branch: 0, inst: Rlc(H) }, //0x04
    OpInfo { mnemonic: "RLC L", cycles: 8, cycles_no_branch: 0, inst: Rlc(L) }, //0x05
    OpInfo { mnemonic: "RLC (HL)", cycles: 16, cycles_no_branch: 0, inst: Rlc(AddrHL) }, //0x06
    OpInfo { mnemonic: "RLC A", cycles: 8, cycles_no_branch: 0, inst: Rlc(A) }, //0x07
    OpInfo { mnemonic: "RRC B", cycles: 8, cycles_no_branch: 0, inst: Rrc(B) }, //0x08
    OpInfo { mnemonic: "RRC C", cycles: 8, cycles_no_branch: 0, inst: Rrc(C) }, //0x09
    OpInfo { mnemonic: "RRC D", cycles: 8, cycles_no_branch: 0, inst: Rrc(D) }, //0x0A
    OpInfo { mnemonic: "RRC E", cycles: 8, cycles_no_branch: 0, inst: Rrc(E) }, //0x0B
    OpInfo { mnemonic: "RRC H", cycles: 8, cycles_no_branch: 0, inst: Rrc(H) }, //0x0C
    OpInfo { mnemonic: "RRC L", cycles: 8, cycles_no_branch: 0, inst: Rrc(L) }, //0x0D
    OpInfo { mnemonic: "RRC (HL)", cycles: 16, cycles_no_branch: 0, inst: Rrc(AddrHL) }, //0x0E
    OpInfo { mnemonic: "RRC A", cycles: 8, cycles_no_branch: 0, inst: Rrc(A) }, //0x0F
    OpInfo { mnemonic: "RL B", cycles: 8, cycles_no_branch: 0, inst: Rl(B) }, //0x10
    OpInfo { mnemonic: "RL C", cycles: 8, cycles_no_branch: 0, inst: Rl(C) }, //0x11
    OpInfo { mnemonic: "RL D", cycles: 8, cycles_no_branch: 0, inst: Rl(D) }, //0x12
    OpInfo { mnemonic: "RL E", cycles: 8, cycles_no_branch: 0, inst: Rl(E) }, //0x13
    OpInfo { mnemonic: "RL H", cycles: 8, cycles_no_branch: 0, inst: Rl(H) }, //0x14
    OpInfo { mnemonic: "RL L", cycles: 8, cycles_no_branch: 0, inst: Rl(L) }, //0x15
    OpInfo { mnemonic: "RL (HL)", cycles: 16, cycles_no_branch: 0, inst: Rl(AddrHL) }, //0x16
    OpInfo { mnemonic: "RL A", cycles: 8, cycles_no_branch: 0, inst: Rl(A) }, //0x17
    OpInfo { mnemonic: "RR B", cycles: 8, cycles_no_branch: 0, inst: Rr(B) }, //0x18
    OpInfo { mnemonic: "RR C", cycles: 8, cycles_no_branch: 0, inst: Rr(C) }, //0x19
    OpInfo { mnemonic: "RR D", cycles: 8, cycles_no_branch: 0, inst: Rr(D) }, //0x1A
    OpInfo { mnemonic: "RR E", cycles: 8, cycles_no_branch: 0, inst: Rr(E) }, //0x1B
    OpInfo { mnemonic: "RR H", cycles: 8, cycles_no_branch: 0, inst: Rr(H) }, //0x1C
    OpInfo { mnemonic: "RR L", cycles: 8, cycles_no_branch: 0, inst: Rr(L) }, //0x1D
    OpInfo { mnemonic: "RR (HL)", cycles: 16, cycles_no_branch: 0, inst: Rr(AddrHL) }, //0x1E
    OpInfo { mnemonic: "RR A", cycles: 8, cycles_no_branch: 0, inst: Rr(A) }, //0x1F
    OpInfo { mnemonic: "SLA B", cycles: 8, cycles_no_branch: 0, inst: Sla(B) }, //0x20
    OpInfo { mnemonic: "SLA C", cycles: 8, cycles_no_branch: 0, inst: Sla(C) }, //0x21
    OpInfo { mnemonic: "SLA D", cycles: 8, cycles_no_branch: 0, inst: Sla(D) }, //0x22
    OpInfo { mnemonic: "SLA E", cycles: 8, cycles_no_branch: 0, inst: Sla(E) }, //0x23
    OpInfo { mnemonic: "SLA H", cycles: 8, cycles_no_branch: 0, inst: Sla(H) }, //0x24
    OpInfo { mnemonic: "SLA L", cycles: 8, cycles_no_branch: 0, inst: Sla(L) }, //0x25
    OpInfo { mnemonic: "SLA (HL)", cycles: 16, cycles_no_branch: 0, inst: Sla(AddrHL) }, //0x26
    OpInfo { mnemonic: "SLA A", cycles: 8, cycles_no_branch: 0, inst: Sla(A) }, //0x27
    OpInfo { mnemonic: "SRA B", cycles: 8, cycles_no_branch: 0, inst: Sra(B) }, //0x28
    OpInfo { mnemonic: "SRA C", cycles: 8, cycles_no_branch: 0, inst: Sra(C) }, //0x29
    OpInfo { mnemonic: "SRA D", cycles: 8, cycles_no_branch: 0, inst: Sra(D) }, //0x2A
    OpInfo { mnemonic: "SRA E", cycles: 8, cycles_no_branch: 0, inst: Sra(E) }, //0x2B
    OpInfo { mnemonic: "SRA H", cycles: 8, cycles_no_branch: 0, inst: Sra(H) }, //0x2C
    OpInfo { mnemonic: "SRA L", cycles: 8, cycles_no_branch: 0, inst: Sra(L) }, //0x2D
    OpInfo { mnemonic: "SRA (HL)", cycles: 16, cycles_no_branch: 0, inst: Sra(AddrHL) }, //0x2E
    OpInfo { mnemonic: "SRA A", cycles: 8, cycles_no_branch: 0, inst: Sra(A) }, //0x2F
    OpInfo { mnemonic: "SWAP B", cycles: 8, cycles_no_branch: 0, inst: Swap(B) }, //0x30
    OpInfo { mnemonic: "SWAP C", cycles: 8, cycles_no_branch: 0, inst: Swap(C) }, //0x31
    OpInfo { mnemonic: "SWAP D", cycles: 8, cycles_no_branch: 0, inst: Swap(D) }, //0x32
    OpInfo { mnemonic: "SWAP E", cycles: 8, cycles_no_branch: 0, inst: Swap(E) }, //0x33
    OpInfo { mnemonic: "SWAP H", cycles: 8, cycles_no_branch: 0, inst: Swap(H) }, //0x34
    OpInfo { mnemonic: "SWAP L", cycles: 8, cycles_no_branch: 0, inst: Swap(L) }, //0x35
    OpInfo { mnemonic: "SWAP (HL)", cycles: 16, cycles_no_branch: 0, inst: Swap(AddrHL) }, //0x36
    OpInfo { mnemonic: "SWAP A", cycles: 8, cycles_no_branch: 0, inst: Swap(A) }, //0x37
    OpInfo { mnemonic: "SRL B", cycles: 8, cycles_no_branch: 0, inst: Srl(B) }, //0x38
    OpInfo { mnemonic: "SRL C", cycles: 8, cycles_no_branch: 0, inst: Srl(C) }, //0x39
    OpInfo { mnemonic: "SRL D", cycles: 8, cycles_no_branch: 0, inst: Srl(D) }, //0x3A
    OpInfo { mnemonic: "SRL E", cycles: 8, cycles_no_branch: 0, inst: Srl(E) }, //0x3B
    OpInfo { mnemonic: "SRL H", cycles: 8, cycles_no_branch: 0, inst: Srl(H) }, //0x3C
    OpInfo { mnemonic: "SRL L", cycles: 8, cycles_no_branch: 0, inst: Srl(L) }, //0x3D
    OpInfo { mnemonic: "SRL (HL)", cycles: 16, cycles_no_branch: 0, inst: Srl(AddrHL) }, //0x3E
    OpInfo { mnemonic: "SRL A", cycles: 8, cycles_no_branch: 0, inst: Srl(A) }, //0x3F
    OpInfo { mnemonic: "BIT 0,B", cycles: 8, cycles_no_branch: 0, inst: Bit(0, B) }, //0x40
    OpInfo { mnemonic: "BIT 0,C", cycles: 8, cycles_no_branch: 0, inst: Bit(0, C) }, //0x41
    OpInfo { mnemonic: "BIT 0,D", cycles: 8, cycles_no_branch: 0, inst: Bit(0, D) }, //0x42
    OpInfo { mnemonic: "BIT 0,E", cycles: 8, cycles_no_branch: 0, inst: Bit(0, E) }, //0x43
    OpInfo { mnemonic: "BIT 0,H", cycles: 8, cycles_no_branch: 0, inst: Bit(0, H) }, //0x44
    OpInfo { mnemonic: "BIT 0,L", cycles: 8, cycles_no_branch: 0, inst: Bit(0, L) }, //0x45
    OpInfo { mnemonic: "BIT 0,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(0, AddrHL) }, //0x46
    OpInfo { mnemonic: "BIT 0,A", cycles: 8, cycles_no_branch: 0, inst: Bit(0, A) }, //0x47
    OpInfo { mnemonic: "BIT 1,B", cycles: 8, cycles_no_branch: 0, inst: Bit(1, B) }, //0x48
    OpInfo { mnemonic: "BIT 1,C", cycles: 8, cycles_no_branch: 0, inst: Bit(1, C) }, //0x49
    OpInfo { mnemonic: "BIT 1,D", cycles: 8, cycles_no_branch: 0, inst: Bit(1, D) }, //0x4A
    OpInfo { mnemonic: "BIT 1,E", cycles: 8, cycles_no_branch: 0, inst: Bit(1, E) }, //0x4B
    OpInfo { mnemonic: "BIT 1,H", cycles: 8, cycles_no_branch: 0, inst: Bit(1, H) }, //0x4C
    OpInfo { mnemonic: "BIT 1,L", cycles: 8, cycles_no_branch: 0, inst: Bit(1, L) }, //0x4D
    OpInfo { mnemonic: "BIT 1,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(1, AddrHL) }, //0x4E
    OpInfo { mnemonic: "BIT 1,A", cycles: 8, cycles_no_branch: 0, inst: Bit(1, A) }, //0x4F
    OpInfo { mnemonic: "BIT 2,B", cycles: 8, cycles_no_branch: 0, inst: Bit(2, B) }, //0x50
    OpInfo { mnemonic: "BIT 2,C", cycles: 8, cycles_no_branch: 0, inst: Bit(2, C) }, //0x51
    OpInfo { mnemonic: "BIT 2,D", cycles: 8, cycles_no_branch: 0, inst: Bit(2, D) }, //0x52
    OpInfo { mnemonic: "BIT 2,E", cycles: 8, cycles_no_branch: 0, inst: Bit(2, E) }, //0x53
    OpInfo { mnemonic: "BIT 2,H", cycles: 8, cycles_no_branch: 0, inst: Bit(2, H) }, //0x54
    OpInfo { mnemonic: "BIT 2,L", cycles: 8, cycles_no_branch: 0, inst: Bit(2, L) }, //0x55
    OpInfo { mnemonic: "BIT 2,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(2, AddrHL) }, //0x56
    OpInfo { mnemonic: "BIT 2,A", cycles: 8, cycles_no_branch: 0, inst: Bit(2, A) }, //0x57
    OpInfo { mnemonic: "BIT 3,B", cycles: 8, cycles_no_branch: 0, inst: Bit(3, B) }, //0x58
    OpInfo { mnemonic: "BIT 3,C", cycles: 8, cycles_no_branch: 0, inst: Bit(3, C) }, //0x59
    OpInfo { mnemonic: "BIT 3,D", cycles: 8, cycles_no_branch: 0, inst: Bit(3, D) }, //0x5A
    OpInfo { mnemonic: "BIT 3,E", cycles: 8, cycles_no_branch: 0, inst: Bit(3, E) }, //0x5B
    OpInfo { mnemonic: "BIT 3,H", cycles: 8, cycles_no_branch: 0, inst: Bit(3, H) }, //0x5C
    OpInfo { mnemonic: "BIT 3,L", cycles: 8, cycles_no_branch: 0, inst: Bit(3, L) }, //0x5D
    OpInfo { mnemonic: "BIT 3,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(3, AddrHL) }, //0x5E
    OpInfo { mnemonic: "BIT 3,A", cycles: 8, cycles_no_branch: 0, inst: Bit(3, A) }, //0x5F
    OpInfo { mnemonic: "BIT 4,B", cycles: 8, cycles_no_branch: 0, inst: Bit(4, B) }, //0x60
    OpInfo { mnemonic: "BIT 4,C", cycles: 8, cycles_no_branch: 0, inst: Bit(4, C) }, //0x61
    OpInfo { mnemonic: "BIT 4,D", cycles: 8, cycles_no_branch: 0, inst: Bit(4, D) }, //0x62
    OpInfo { mnemonic: "BIT 4,E", cycles: 8, cycles_no_branch: 0, inst: Bit(4, E) }, //0x63
    OpInfo { mnemonic: "BIT 4,H", cycles: 8, cycles_no_branch: 0, inst: Bit(4, H) }, //0x64
    OpInfo { mnemonic: "BIT 4,L", cycles: 8, cycles_no_branch: 0, inst: Bit(4, L) }, //0x65
    OpInfo { mnemonic: "BIT 4,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(4, AddrHL) }, //0x66
    OpInfo { mnemonic: "BIT 4,A", cycles: 8, cycles_no_branch: 0, inst: Bit(4, A) }, //0x67
    OpInfo { mnemonic: "BIT 5,B", cycles: 8, cycles_no_branch: 0, inst: Bit(5, B) }, //0x68
    OpInfo { mnemonic: "BIT 5,C", cycles: 8, cycles_no_branch: 0, inst: Bit(5, C) }, //0x69
    OpInfo { mnemonic: "BIT 5,D", cycles: 8, cycles_no_branch: 0, inst: Bit(5, D) }, //0x6A
    OpInfo { mnemonic: "BIT 5,E", cycles: 8, cycles_no_branch: 0, inst: Bit(5, E) }, //0x6B
    OpInfo { mnemonic: "BIT 5,H", cycles: 8, cycles_no_branch: 0, inst: Bit(5, H) }, //0x6C
    OpInfo { mnemonic: "BIT 5,L", cycles: 8, cycles_no_branch: 0, inst: Bit(5, L) }, //0x6D
    OpInfo { mnemonic: "BIT 5,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(5, AddrHL) }, //0x6E
    OpInfo { mnemonic: "BIT 5,A", cycles: 8, cycles_no_branch: 0, inst: Bit(5, A) }, //0x6F
    OpInfo { mnemonic: "BIT 6,B", cycles: 8, cycles_no_branch: 0, inst: Bit(6, B) }, //0x70
    OpInfo { mnemonic: "BIT 6,C", cycles: 8, cycles_no_branch: 0, inst: Bit(6, C) }, //0x71
    OpInfo { mnemonic: "BIT 6,D", cycles: 8, cycles_no_branch: 0, inst: Bit(6, D) }, //0x72
    OpInfo { mnemonic: "BIT 6,E", cycles: 8, cycles_no_branch: 0, inst: Bit(6, E) }, //0x73
    OpInfo { mnemonic: "BIT 6,H", cycles: 8, cycles_no_branch: 0, inst: Bit(6, H) }, //0x74
    OpInfo { mnemonic: "BIT 6,L", cycles: 8, cycles_no_branch: 0, inst: Bit(6, L) }, //0x75
    OpInfo { mnemonic: "BIT 6,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(6, AddrHL) }, //0x76
    OpInfo { mnemonic: "BIT 6,A", cycles: 8, cycles_no_branch: 0, inst: Bit(6, A) }, //0x77
    OpInfo { mnemonic: "BIT 7,B", cycles: 8, cycles_no_branch: 0, inst: Bit(7, B) }, //0x78
    OpInfo { mnemonic: "BIT 7,C", cycles: 8, cycles_no_branch: 0, inst: Bit(7, C) }, //0x79
    OpInfo { mnemonic: "BIT 7,D", cycles: 8, cycles_no_branch: 0, inst: Bit(7, D) }, //0x7A
    OpInfo { mnemonic: "BIT 7,E", cycles: 8, cycles_no_branch: 0, inst: Bit(7, E) }, //0x7B
    OpInfo { mnemonic: "BIT 7,H", cycles: 8, cycles_no_branch: 0, inst: Bit(7, H) }, //0x7C
    OpInfo { mnemonic: "BIT 7,L", cycles: 8, cycles_no_branch: 0, inst: Bit(7, L) }, //0x7D
    OpInfo { mnemonic: "BIT 7,(HL)", cycles: 12, cycles_no_branch: 0, inst: Bit(7, AddrHL) }, //0x7E
    OpInfo { mnemonic: "BIT 7,A", cycles: 8, cycles_no_branch: 0, inst: Bit(7, A) }, //0x7F
    OpInfo { mnemonic: "RES 0,B", cycles: 8, cycles_no_branch: 0, inst: Res(0, B) }, //0x80
    OpInfo { mnemonic: "RES 0,C", cycles: 8, cycles_no_branch: 0, inst: Res(0, C) }, //0x81
    OpInfo { mnemonic: "RES 0,D", cycles: 8, cycles_no_branch: 0, inst: Res(0, D) }, //0x82
    OpInfo { mnemonic: "RES 0,E", cycles: 8, cycles_no_branch: 0, inst: Res(0, E) }, //0x83
    OpInfo { mnemonic: "RES 0,H", cycles: 8, cycles_no_branch: 0, inst: Res(0, H) }, //0x84
    OpInfo { mnemonic: "RES 0,L", cycles: 8, cycles_no_branch: 0, inst: Res(0, L) }, //0x85
    OpInfo { mnemonic: "RES 0,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(0, AddrHL) }, //0x86
    OpInfo { mnemonic: "RES 0,A", cycles: 8, cycles_no_branch: 0, inst: Res(0, A) }, //0x87
    OpInfo { mnemonic: "RES 1,B", cycles: 8, cycles_no_branch: 0, inst: Res(1, B) }, //0x88
    OpInfo { mnemonic: "RES 1,C", cycles: 8, cycles_no_branch: 0, inst: Res(1, C) }, //0x89
    OpInfo { mnemonic: "RES 1,D", cycles: 8, cycles_no_branch: 0, inst: Res(1, D) }, //0x8A
    OpInfo { mnemonic: "RES 1,E", cycles: 8, cycles_no_branch: 0, inst: Res(1, E) }, //0x8B
    OpInfo { mnemonic: "RES 1,H", cycles: 8, cycles_no_branch: 0, inst: Res(1, H) }, //0x8C
    OpInfo { mnemonic: "RES 1,L", cycles: 8, cycles_no_branch: 0, inst: Res(1, L) }, //0x8D
    OpInfo { mnemonic: "RES 1,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(1, AddrHL) }, //0x8E
    OpInfo { mnemonic: "RES 1,A", cycles: 8, cycles_no_branch: 0, inst: Res(1, A) }, //0x8F
    OpInfo { mnemonic: "RES 2,B", cycles: 8, cycles_no_branch: 0, inst: Res(2, B) }, //0x90
    OpInfo { mnemonic: "RES 2,C", cycles: 8, cycles_no_branch: 0, inst: Res(2, C) }, //0x91
    OpInfo { mnemonic: "RES 2,D", cycles: 8, cycles_no_branch: 0, inst: Res(2, D) }, //0x92
    OpInfo { mnemonic: "RES 2,E", cycles: 8, cycles_no_branch: 0, inst: Res(2, E) }, //0x93
    OpInfo { mnemonic: "RES 2,H", cycles: 8, cycles_no_branch: 0, inst: Res(2, H) }, //0x94
    OpInfo { mnemonic: "RES 2,L", cycles: 8, cycles_no_branch: 0, inst: Res(2, L) }, //0x95
    OpInfo { mnemonic: "RES 2,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(2, AddrHL) }, //0x96
    OpInfo { mnemonic: "RES 2,A", cycles: 8, cycles_no_branch: 0, inst: Res(2, A) }, //0x97
    OpInfo { mnemonic: "RES 3,B", cycles: 8, cycles_no_branch: 0, inst: Res(3, B) }, //0x98
    OpInfo { mnemonic: "RES 3,C", cycles: 8, cycles_no_branch: 0, inst: Res(3, C) }, //0x99
    OpInfo { mnemonic: "RES 3,D", cycles: 8, cycles_no_branch: 0, inst: Res(3, D) }, //0x9A
    OpInfo { mnemonic: "RES 3,E", cycles: 8, cycles_no_branch: 0, inst: Res(3, E) }, //0x9B
    OpInfo { mnemonic: "RES 3,H", cycles: 8, cycles_no_branch: 0, inst: Res(3, H) }, //0x9C
    OpInfo { mnemonic: "RES 3,L", cycles: 8, cycles_no_branch: 0, inst: Res(3, L) }, //0x9D
    OpInfo { mnemonic: "RES 3,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(3, AddrHL) }, //0x9E
    OpInfo { mnemonic: "RES 3,A", cycles: 8, cycles_no_branch: 0, inst: Res(3, A) }, //0x9F
    OpInfo { mnemonic: "RES 4,B", cycles: 8, cycles_no_branch: 0, inst: Res(4, B) }, //0xA0
    OpInfo { mnemonic: "RES 4,C", cycles: 8, cycles_no_branch: 0, inst: Res(4, C) }, //0xA1
    OpInfo { mnemonic: "RES 4,D", cycles: 8, cycles_no_branch: 0, inst: Res(4, D) }, //0xA2
    OpInfo { mnemonic: "RES 4,E", cycles: 8, cycles_no_branch: 0, inst: Res(4, E) }, //0xA3
    OpInfo { mnemonic: "RES 4,H", cycles: 8, cycles_no_branch: 0, inst: Res(4, H) }, //0xA4
    OpInfo { mnemonic: "RES 4,L", cycles: 8, cycles_no_branch: 0, inst: Res(4, L) }, //0xA5
    OpInfo { mnemonic: "RES 4,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(4, AddrHL) }, //0xA6
    OpInfo { mnemonic: "RES 4,A", cycles: 8, cycles_no_branch: 0, inst: Res(4, A) }, //0xA7
    OpInfo { mnemonic: "RES 5,B", cycles: 8, cycles_no_branch: 0, inst: Res(5, B) }, //0xA8
    OpInfo { mnemonic: "RES 5,C", cycles: 8, cycles_no_branch: 0, inst: Res(5, C) }, //0xA9
    OpInfo { mnemonic: "RES 5,D", cycles: 8, cycles_no_branch: 0, inst: Res(5, D) }, //0xAA
    OpInfo { mnemonic: "RES 5,E", cycles: 8, cycles_no_branch: 0, inst: Res(5, E) }, //0xAB
    OpInfo { mnemonic: "RES 5,H", cycles: 8, cycles_no_branch: 0, inst: Res(5, H) }, //0xAC
    OpInfo { mnemonic: "RES 5,L", cycles: 8, cycles_no_branch: 0, inst: Res(5, L) }, //0xAD
    OpInfo { mnemonic: "RES 5,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(5, AddrHL) }, //0xAE
    OpInfo { mnemonic: "RES 5,A", cycles: 8, cycles_no_branch: 0, inst: Res(5, A) }, //0xAF
    OpInfo { mnemonic: "RES 6,B", cycles: 8, cycles_no_branch: 0, inst: Res(6, B) }, //0xB0
    OpInfo { mnemonic: "RES 6,C", cycles: 8, cycles_no_branch: 0, inst: Res(6, C) }, //0xB1
    OpInfo { mnemonic: "RES 6,D", cycles: 8, cycles_no_branch: 0, inst: Res(6, D) }, //0xB2
    OpInfo { mnemonic: "RES 6,E", cycles: 8, cycles_no_branch: 0, inst: Res(6, E) }, //0xB3
    OpInfo { mnemonic: "RES 6,H", cycles: 8, cycles_no_branch: 0, inst: Res(6, H) }, //0xB4
    OpInfo { mnemonic: "RES 6,L", cycles: 8, cycles_no_branch: 0, inst: Res(6, L) }, //0xB5
    OpInfo { mnemonic: "RES 6,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(6, AddrHL) }, //0xB6
    OpInfo { mnemonic: "RES 6,A", cycles: 8, cycles_no_branch: 0, inst: Res(6, A) }, //0xB7
    OpInfo { mnemonic: "RES 7,B", cycles: 8, cycles_no_branch: 0, inst: Res(7, B) }, //0xB8
    OpInfo { mnemonic: "RES 7,C", cycles: 8, cycles_no_branch: 0, inst: Res(7, C) }, //0xB9
    OpInfo { mnemonic: "RES 7,D", cycles: 8, cycles_no_branch: 0, inst: Res(7, D) }, //0xBA
    OpInfo { mnemonic: "RES 7,E", cycles: 8, cycles_no_branch: 0, inst: Res(7, E) }, //0xBB
    OpInfo { mnemonic: "RES 7,H", cycles: 8, cycles_no_branch: 0, inst: Res(7, H) }, //0xBC
    OpInfo { mnemonic: "RES 7,L", cycles: 8, cycles_no_branch: 0, inst: Res(7, L) }, //0xBD
    OpInfo { mnemonic: "RES 7,(HL)", cycles: 16, cycles_no_branch: 0, inst: Res(7, AddrHL) }, //0xBE
    OpInfo { mnemonic: "RES 7,A", cycles: 8, cycles_no_branch: 0, inst: Res(7, A) }, //0xBF
    OpInfo { mnemonic: "SET 0,B", cycles: 8, cycles_no_branch: 0, inst: Set(0, B) }, //0xC0
    OpInfo { mnemonic: "SET 0,C", cycles: 8, cycles_no_branch: 0, inst: Set(0, C) }, //0xC1
    OpInfo { mnemonic: "SET 0,D", cycles: 8, cycles_no_branch: 0, inst: Set(0, D) }, //0xC2
    OpInfo { mnemonic: "SET 0,E", cycles: 8, cycles_no_branch: 0, inst: Set(0, E) }, //0xC3
    OpInfo { mnemonic: "SET 0,H", cycles: 8, cycles_no_branch: 0, inst: Set(0, H) }, //0xC4
    OpInfo { mnemonic: "SET 0,L", cycles: 8, cycles_no_branch: 0, inst: Set(0, L) }, //0xC5
    OpInfo { mnemonic: "SET 0,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(0, AddrHL) }, //0xC6
    OpInfo { mnemonic: "SET 0,A", cycles: 8, cycles_no_branch: 0, inst: Set(0, A) }, //0xC7
    OpInfo { mnemonic: "SET 1,B", cycles: 8, cycles_no_branch: 0, inst: Set(1, B) }, //0xC8
    OpInfo { mnemonic: "SET 1,C", cycles: 8, cycles_no_branch: 0, inst: Set(1, C) }, //0xC9
    OpInfo { mnemonic: "SET 1,D", cycles: 8, cycles_no_branch: 0, inst: Set(1, D) }, //0xCA
    OpInfo { mnemonic: "SET 1,E", cycles: 8, cycles_no_branch: 0, inst: Set(1, E) }, //0xCB
    OpInfo { mnemonic: "SET 1,H", cycles: 8, cycles_no_branch: 0, inst: Set(1, H) }, //0xCC
    OpInfo { mnemonic: "SET 1,L", cycles: 8, cycles_no_branch: 0, inst: Set(1, L) }, //0xCD
    OpInfo { mnemonic: "SET 1,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(1, AddrHL) }, //0xCE
    OpInfo { mnemonic: "SET 1,A", cycles: 8, cycles_no_branch: 0, inst: Set(1, A) }, //0xCF
    OpInfo { mnemonic: "SET 2,B", cycles: 8, cycles_no_branch: 0, inst: Set(2, B) }, //0xD0
    OpInfo { mnemonic: "SET 2,C", cycles: 8, cycles_no_branch: 0, inst: Set(2, C) }, //0xD1
    OpInfo { mnemonic: "SET 2,D", cycles: 8, cycles_no_branch: 0, inst: Set(2, D) }, //0xD2
    OpInfo { mnemonic: "SET 2,E", cycles: 8, cycles_no_branch: 0, inst: Set(2, E) }, //0xD3
    OpInfo { mnemonic: "SET 2,H", cycles: 8, cycles_no_branch: 0, inst: Set(2, H) }, //0xD4
    OpInfo { mnemonic: "SET 2,L", cycles: 8, cycles_no_branch: 0, inst: Set(2, L) }, //0xD5
    OpInfo { mnemonic: "SET 2,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(2, AddrHL) }, //0xD6
    OpInfo { mnemonic: "SET 2,A", cycles: 8, cycles_no_branch: 0, inst: Set(2, A) }, //0xD7
    OpInfo { mnemonic: "SET 3,B", cycles: 8, cycles_no_branch: 0, inst: Set(3, B) }, //0xD8
    OpInfo { mnemonic: "SET 3,C", cycles: 8, cycles_no_branch: 0, inst: Set(3, C) }, //0xD9
    OpInfo { mnemonic: "SET 3,D", cycles: 8, cycles_no_branch: 0, inst: Set(3, D) }, //0xDA
    OpInfo { mnemonic: "SET 3,E", cycles: 8, cycles_no_branch: 0, inst: Set(3, E) }, //0xDB
    OpInfo { mnemonic: "SET 3,H", cycles: 8, cycles_no_branch: 0, inst: Set(3, H) }, //0xDC
    OpInfo { mnemonic: "SET 3,L", cycles: 8, cycles_no_branch: 0, inst: Set(3, L) }, //0xDD
    OpInfo { mnemonic: "SET 3,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(3, AddrHL) }, //0xDE
    OpInfo { mnemonic: "SET 3,A", cycles: 8, cycles_no_branch: 0, inst: Set(3, A) }, //0xDF
    OpInfo { mnemonic: "SET 4,B", cycles: 8, cycles_no_branch: 0, inst: Set(4, B) }, //0xE0
    OpInfo { mnemonic: "SET 4,C", cycles: 8, cycles_no_branch: 0, inst: Set(4, C) }, //0xE1
    OpInfo { mnemonic: "SET 4,D", cycles: 8, cycles_no_branch: 0, inst: Set(4, D) }, //0xE2
    OpInfo { mnemonic: "SET 4,E", cycles: 8, cycles_no_branch: 0, inst: Set(4, E) }, //0xE3
    OpInfo { mnemonic: "SET 4,H", cycles: 8, cycles_no_branch: 0, inst: Set(4, H) }, //0xE4
    OpInfo { mnemonic: "SET 4,L", cycles: 8, cycles_no_branch: 0, inst: Set(4, L) }, //0xE5
    OpInfo { mnemonic: "SET 4,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(4, AddrHL) }, //0xE6
    OpInfo { mnemonic: "SET 4,A", cycles: 8, cycles_no_branch: 0, inst: Set(4, A) }, //0xE7
    OpInfo { mnemonic: "SET 5,B", cycles: 8, cycles_no_branch: 0, inst: Set(5, B) }, //0xE8
    OpInfo { mnemonic: "SET 5,C", cycles: 8, cycles_no_branch: 0, inst: Set(5, C) }, //0xE9
    OpInfo { mnemonic: "SET 5,D", cycles: 8, cycles_no_branch: 0, inst: Set(5, D) }, //0xEA
    OpInfo { mnemonic: "SET 5,E", cycles: 8, cycles_no_branch: 0, inst: Set(5, E) }, //0xEB
    OpInfo { mnemonic: "SET 5,H", cycles: 8, cycles_no_branch: 0, inst: Set(5, H) }, //0xEC
    OpInfo { mnemonic: "SET 5,L", cycles: 8, cycles_no_branch: 0, inst: Set(5, L) }, //0xED
    OpInfo { mnemonic: "SET 5,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(5, AddrHL) }, //0xEE
    OpInfo { mnemonic: "SET 5,A", cycles: 8, cycles_no_branch: 0, inst: Set(5, A) }, //0xEF
    OpInfo { mnemonic: "SET 6,B", cycles: 8, cycles_no_branch: 0, inst: Set(6, B) }, //0xF0
    OpInfo { mnemonic: "SET 6,C", cycles: 8, cycles_no_branch: 0, inst: Set(6, C) }, //0xF1
    OpInfo { mnemonic: "SET 6,D", cycles: 8, cycles_no_branch: 0, inst: Set(6, D) }, //0xF2
    OpInfo { mnemonic: "SET 6,E", cycles: 8, cycles_no_branch: 0, inst: Set(6, E) }, //0xF3
    OpInfo { mnemonic: "SET 6,H", cycles: 8, cycles_no_branch: 0, inst: Set(6, H) }, //0xF4
    OpInfo { mnemonic: "SET 6,L", cycles: 8, cycles_no_branch: 0, inst: Set(6, L) }, //0xF5
    OpInfo { mnemonic: "SET 6,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(6, AddrHL) }, //0xF6
    OpInfo { mnemonic: "SET 6,A", cycles: 8, cycles_no_branch: 0, inst: Set(6, A) }, //0xF7
    OpInfo { mnemonic: "SET 7,B", cycles: 8, cycles_no_branch: 0, inst: Set(7, B) }, //0xF8
    OpInfo { mnemonic: "SET 7,C", cycles: 8, cycles_no_branch: 0, inst: Set(7, C) }, //0xF9
    OpInfo { mnemonic: "SET 7,D", cycles: 8, cycles_no_branch: 0, inst: Set(7, D) }, //0xFA
    OpInfo { mnemonic: "SET 7,E", cycles: 8, cycles_no_branch: 0, inst: Set(7, E) }, //0xFB
    OpInfo { mnemonic: "SET 7,H", cycles: 8, cycles_no_branch: 0, inst: Set(7, H) }, //0xFC
    OpInfo { mnemonic: "SET 7,L", cycles: 8, cycles_no_branch: 0, inst: Set(7, L) }, //0xFD
    OpInfo { mnemonic: "SET 7,(HL)", cycles: 16, cycles_no_branch: 0, inst: Set(7, AddrHL) }, //0xFE
    OpInfo { mnemonic: "SET 7,A", cycles: 8, cycles_no_branch: 0, inst: Set(7, A) }, //0xFF
];

pub struct OpInfo {
    pub mnemonic: &'static str,
    pub cycles: usize,
    pub cycles_no_branch: usize,
    pub inst: Opcode,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand8 {
    U8,
    I8,
    AddrU16,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AddrBC,
    AddrDE,
    AddrHL,
    AddrHLInc,
    AddrHLDec,
    LowAddrC,
    LowAddrU8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand16 {
    U16,
    SP,
    BC,
    DE,
    HL,
    AF,
    SPPlusI8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    Zero,
    NotZero,
    Carry,
    NotCarry,
    Unconditional,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    Nop,

    Ld16(Operand16, Operand16),
    Ld8(Operand8, Operand8),
    LdU16StackAddr,

    Inc8(Operand8),
    Inc16(Operand16),

    Dec8(Operand8),
    Dec16(Operand16),

    Add16(Operand16),
    Add8(Operand8),
    AddI8SP,

    Stop,

    Rlc(Operand8),
    Rrc(Operand8),

    Rl(Operand8),
    Rr(Operand8),

    Jp(Condition, Operand16),
    Jr(Condition),
    Ret(Condition),

    Daa,
    Cpl,
    Scf,
    Ccf,
    Halt,
    Adc(Operand8),
    Sub(Operand8),
    Sbc(Operand8),
    And(Operand8),
    Xor(Operand8),
    Or(Operand8),
    Cp(Operand8),
    Pop(Operand16),
    Call(Condition),
    Push(Operand16),
    Rst(u16),
    Prefix,
    Reti,
    Di,
    Ei,
    Sla(Operand8),
    Sra(Operand8),
    Swap(Operand8),
    Srl(Operand8),
    Bit(u8, Operand8),
    Res(u8, Operand8),
    Set(u8, Operand8),
    Illegal,
}
