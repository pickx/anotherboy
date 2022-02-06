// use serde::{Deserialize, Serialize};
// use serde_json::Result;

// #[derive(Serialize, Deserialize)]

use std::ops::Add;

use Condition::*;
use Op16::*;
use Op8::*;
use OpcodePrefixed::*;
use OpcodeUnprefixed::*;

#[rustfmt::skip]
pub const LOOKUP: [UnprefixedOpInfo; 256] = [
    UnprefixedOpInfo { mnemonic: "NOP", cycles: 4, cycles_no_branch: 0, inst: Nop }, //0x00
    UnprefixedOpInfo { mnemonic: "LD BC,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(BC, U16) }, //0x01
    UnprefixedOpInfo { mnemonic: "LD (BC),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrBC, A) }, //0x02
    UnprefixedOpInfo { mnemonic: "INC BC", cycles: 8, cycles_no_branch: 0, inst: Inc16(BC) }, //0x03
    UnprefixedOpInfo { mnemonic: "INC B", cycles: 4, cycles_no_branch: 0, inst:Inc8(B) }, //0x04
    UnprefixedOpInfo { mnemonic: "DEC B", cycles: 4, cycles_no_branch: 0, inst: Dec8(B) }, //0x05
    UnprefixedOpInfo { mnemonic: "LD B,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(B, U8) }, //0x06
    UnprefixedOpInfo { mnemonic: "RLCA", cycles: 4, cycles_no_branch: 0, inst: Rlca }, //0x07
    UnprefixedOpInfo { mnemonic: "LD (u16),SP", cycles: 20, cycles_no_branch: 0, inst: LdU16StackAddr }, //0x08
    UnprefixedOpInfo { mnemonic: "ADD HL,BC", cycles: 8, cycles_no_branch: 0, inst: Add16(BC) }, //0x09
    UnprefixedOpInfo { mnemonic: "LD A,(BC)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrBC) }, //0x0A
    UnprefixedOpInfo { mnemonic: "DEC BC", cycles: 8, cycles_no_branch: 0, inst: Dec16(BC) }, //0x0B
    UnprefixedOpInfo { mnemonic: "INC C", cycles: 4, cycles_no_branch: 0, inst: Inc8(C) }, //0x0C
    UnprefixedOpInfo { mnemonic: "DEC C", cycles: 4, cycles_no_branch: 0, inst: Dec8(C) }, //0x0D
    UnprefixedOpInfo { mnemonic: "LD C,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(C, U8) }, //0x0E
    UnprefixedOpInfo { mnemonic: "RRCA", cycles: 4, cycles_no_branch: 0, inst: Rrca }, //0x0F
    UnprefixedOpInfo { mnemonic: "STOP u8", cycles: 4, cycles_no_branch: 0, inst: Stop }, //0x10
    UnprefixedOpInfo { mnemonic: "LD DE,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(DE, U16) }, //0x11
    UnprefixedOpInfo { mnemonic: "LD (DE),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrDE, A) }, //0x12
    UnprefixedOpInfo { mnemonic: "INC DE", cycles: 8, cycles_no_branch: 0, inst: Inc16(DE) }, //0x13
    UnprefixedOpInfo { mnemonic: "INC D", cycles: 4, cycles_no_branch: 0, inst: Inc8(D) }, //0x14
    UnprefixedOpInfo { mnemonic: "DEC D", cycles: 4, cycles_no_branch: 0, inst: Dec8(D) }, //0x15
    UnprefixedOpInfo { mnemonic: "LD D,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(D, U8) }, //0x16
    UnprefixedOpInfo { mnemonic: "RLA", cycles: 4, cycles_no_branch: 0, inst: Rla }, //0x17
    UnprefixedOpInfo { mnemonic: "JR r8", cycles: 12, cycles_no_branch: 0, inst: Jr(Unconditional) }, //0x18
    UnprefixedOpInfo { mnemonic: "ADD HL,DE", cycles: 8, cycles_no_branch: 0, inst: Add16(DE) }, //0x19
    UnprefixedOpInfo { mnemonic: "LD A,(DE)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrDE) }, //0x1A
    UnprefixedOpInfo { mnemonic: "DEC DE", cycles: 8, cycles_no_branch: 0, inst: Dec16(DE) }, //0x1B
    UnprefixedOpInfo { mnemonic: "INC E", cycles: 4, cycles_no_branch: 0, inst: Inc8(E) }, //0x1C
    UnprefixedOpInfo { mnemonic: "DEC E", cycles: 4, cycles_no_branch: 0, inst: Dec8(E) }, //0x1D
    UnprefixedOpInfo { mnemonic: "LD E,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(E, U8) }, //0x1E
    UnprefixedOpInfo { mnemonic: "RRA", cycles: 4, cycles_no_branch: 0, inst: Rra }, //0x1F
    UnprefixedOpInfo { mnemonic: "JR NZ,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(NotZero) }, //0x20
    UnprefixedOpInfo { mnemonic: "LD HL,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(HL, U16) }, //0x21
    UnprefixedOpInfo { mnemonic: "LD (HL+),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHLInc, A)}, //0x22
    UnprefixedOpInfo { mnemonic: "INC HL", cycles: 8, cycles_no_branch: 0, inst: Inc16(HL) }, //0x23
    UnprefixedOpInfo { mnemonic: "INC H", cycles: 4, cycles_no_branch: 0, inst: Inc8(H) }, //0x24
    UnprefixedOpInfo { mnemonic: "DEC H", cycles: 4, cycles_no_branch: 0, inst: Dec8(H) }, //0x25
    UnprefixedOpInfo { mnemonic: "LD H,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(H, U8) }, //0x26
    UnprefixedOpInfo { mnemonic: "DAA", cycles: 4, cycles_no_branch: 0, inst: Daa }, //0x27
    UnprefixedOpInfo { mnemonic: "JR Z,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(Zero) }, //0x28
    UnprefixedOpInfo { mnemonic: "ADD HL,HL", cycles: 8, cycles_no_branch: 0, inst: Add16(HL) }, //0x29
    UnprefixedOpInfo { mnemonic: "LD A,(HL+)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrHLInc) }, //0x2A
    UnprefixedOpInfo { mnemonic: "DEC HL", cycles: 8, cycles_no_branch: 0, inst: Dec16(HL) }, //0x2B
    UnprefixedOpInfo { mnemonic: "INC L", cycles: 4, cycles_no_branch: 0, inst: Inc8(L) }, //0x2C
    UnprefixedOpInfo { mnemonic: "DEC L", cycles: 4, cycles_no_branch: 0, inst: Dec8(L) }, //0x2D
    UnprefixedOpInfo { mnemonic: "LD L,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(L, U8) }, //0x2E
    UnprefixedOpInfo { mnemonic: "CPL", cycles: 4, cycles_no_branch: 0, inst: Cpl }, //0x2F
    UnprefixedOpInfo { mnemonic: "JR NC,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(NotCarry) }, //0x30
    UnprefixedOpInfo { mnemonic: "LD SP,u16", cycles: 12, cycles_no_branch: 0, inst: Ld16(SP, U16) }, //0x31
    UnprefixedOpInfo { mnemonic: "LD (HL-),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHLDec, A) }, //0x32
    UnprefixedOpInfo { mnemonic: "INC SP", cycles: 8, cycles_no_branch: 0, inst: Inc16(SP) }, //0x33
    UnprefixedOpInfo { mnemonic: "INC (HL)", cycles: 12, cycles_no_branch: 0, inst: Inc8(AddrHL) }, //0x34
    UnprefixedOpInfo { mnemonic: "DEC (HL)", cycles: 12, cycles_no_branch: 0, inst: Dec16(HL) }, //0x35
    UnprefixedOpInfo { mnemonic: "LD (HL),u8", cycles: 12, cycles_no_branch: 0, inst: Ld8(AddrHL, U8) }, //0x36
    UnprefixedOpInfo { mnemonic: "SCF", cycles: 4, cycles_no_branch: 0, inst: Scf }, //0x37
    UnprefixedOpInfo { mnemonic: "JR C,r8", cycles: 12, cycles_no_branch: 8, inst: Jr(Carry) }, //0x38
    UnprefixedOpInfo { mnemonic: "ADD HL,SP", cycles: 8, cycles_no_branch: 0, inst: Add16(SP) }, //0x39
    UnprefixedOpInfo { mnemonic: "LD A,(HL-)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrHLDec) }, //0x3A
    UnprefixedOpInfo { mnemonic: "DEC SP", cycles: 8, cycles_no_branch: 0, inst: Dec16(SP) }, //0x3B
    UnprefixedOpInfo { mnemonic: "INC A", cycles: 4, cycles_no_branch: 0, inst: Inc8(A) }, //0x3C
    UnprefixedOpInfo { mnemonic: "DEC A", cycles: 4, cycles_no_branch: 0, inst: Dec8(A) }, //0x3D
    UnprefixedOpInfo { mnemonic: "LD A,u8", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, U8) }, //0x3E
    UnprefixedOpInfo { mnemonic: "CCF", cycles: 4, cycles_no_branch: 0, inst: Ccf }, //0x3F
    UnprefixedOpInfo { mnemonic: "LD B,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, B) }, //0x40
    UnprefixedOpInfo { mnemonic: "LD B,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, C) }, //0x41
    UnprefixedOpInfo { mnemonic: "LD B,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, D) }, //0x42
    UnprefixedOpInfo { mnemonic: "LD B,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, E) }, //0x43
    UnprefixedOpInfo { mnemonic: "LD B,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, H) }, //0x44
    UnprefixedOpInfo { mnemonic: "LD B,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, L) }, //0x45
    UnprefixedOpInfo { mnemonic: "LD B,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(B, AddrHL) }, //0x46
    UnprefixedOpInfo { mnemonic: "LD B,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(B, A) }, //0x47
    UnprefixedOpInfo { mnemonic: "LD C,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, B) }, //0x48
    UnprefixedOpInfo { mnemonic: "LD C,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, C) }, //0x49
    UnprefixedOpInfo { mnemonic: "LD C,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, D) }, //0x4A
    UnprefixedOpInfo { mnemonic: "LD C,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, E) }, //0x4B
    UnprefixedOpInfo { mnemonic: "LD C,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, H) }, //0x4C
    UnprefixedOpInfo { mnemonic: "LD C,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, L) }, //0x4D
    UnprefixedOpInfo { mnemonic: "LD C,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(C, AddrHL) }, //0x4E
    UnprefixedOpInfo { mnemonic: "LD C,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(C, A) }, //0x4F
    UnprefixedOpInfo { mnemonic: "LD D,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, B) }, //0x50
    UnprefixedOpInfo { mnemonic: "LD D,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, C) }, //0x51
    UnprefixedOpInfo { mnemonic: "LD D,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, D) }, //0x52
    UnprefixedOpInfo { mnemonic: "LD D,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, E) }, //0x53
    UnprefixedOpInfo { mnemonic: "LD D,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, H) }, //0x54
    UnprefixedOpInfo { mnemonic: "LD D,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, L) }, //0x55
    UnprefixedOpInfo { mnemonic: "LD D,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(D, AddrHL) }, //0x56
    UnprefixedOpInfo { mnemonic: "LD D,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(D, A) }, //0x57
    UnprefixedOpInfo { mnemonic: "LD E,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, B) }, //0x58
    UnprefixedOpInfo { mnemonic: "LD E,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, C) }, //0x59
    UnprefixedOpInfo { mnemonic: "LD E,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, D) }, //0x5A
    UnprefixedOpInfo { mnemonic: "LD E,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, E) }, //0x5B
    UnprefixedOpInfo { mnemonic: "LD E,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, H) }, //0x5C
    UnprefixedOpInfo { mnemonic: "LD E,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, L) }, //0x5D
    UnprefixedOpInfo { mnemonic: "LD E,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(E, AddrHL) }, //0x5E
    UnprefixedOpInfo { mnemonic: "LD E,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(E, A) }, //0x5F
    UnprefixedOpInfo { mnemonic: "LD H,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, B) }, //0x60
    UnprefixedOpInfo { mnemonic: "LD H,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, C) }, //0x61
    UnprefixedOpInfo { mnemonic: "LD H,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, D) }, //0x62
    UnprefixedOpInfo { mnemonic: "LD H,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, E) }, //0x63
    UnprefixedOpInfo { mnemonic: "LD H,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, H) }, //0x64
    UnprefixedOpInfo { mnemonic: "LD H,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, L) }, //0x65
    UnprefixedOpInfo { mnemonic: "LD H,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(H, AddrHL) }, //0x66
    UnprefixedOpInfo { mnemonic: "LD H,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(H, A) }, //0x67
    UnprefixedOpInfo { mnemonic: "LD L,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, B) }, //0x68
    UnprefixedOpInfo { mnemonic: "LD L,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, C) }, //0x69
    UnprefixedOpInfo { mnemonic: "LD L,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, D) }, //0x6A
    UnprefixedOpInfo { mnemonic: "LD L,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, E) }, //0x6B
    UnprefixedOpInfo { mnemonic: "LD L,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, H) }, //0x6C
    UnprefixedOpInfo { mnemonic: "LD L,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, L) }, //0x6D
    UnprefixedOpInfo { mnemonic: "LD L,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(L, AddrHL) }, //0x6E
    UnprefixedOpInfo { mnemonic: "LD L,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(L, A) }, //0x6F
    UnprefixedOpInfo { mnemonic: "LD (HL),B", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, B) }, //0x70
    UnprefixedOpInfo { mnemonic: "LD (HL),C", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, C) }, //0x71
    UnprefixedOpInfo { mnemonic: "LD (HL),D", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, D) }, //0x72
    UnprefixedOpInfo { mnemonic: "LD (HL),E", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, E) }, //0x73
    UnprefixedOpInfo { mnemonic: "LD (HL),H", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, H) }, //0x74
    UnprefixedOpInfo { mnemonic: "LD (HL),L", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, L) }, //0x75
    UnprefixedOpInfo { mnemonic: "HALT", cycles: 4, cycles_no_branch: 0, inst: Halt }, //0x76
    UnprefixedOpInfo { mnemonic: "LD (HL),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(AddrHL, A) }, //0x77
    UnprefixedOpInfo { mnemonic: "LD A,B", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, B) }, //0x78
    UnprefixedOpInfo { mnemonic: "LD A,C", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, C) }, //0x79
    UnprefixedOpInfo { mnemonic: "LD A,D", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, D) }, //0x7A
    UnprefixedOpInfo { mnemonic: "LD A,E", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, E) }, //0x7B
    UnprefixedOpInfo { mnemonic: "LD A,H", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, H) }, //0x7C
    UnprefixedOpInfo { mnemonic: "LD A,L", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, L) }, //0x7D
    UnprefixedOpInfo { mnemonic: "LD A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, AddrHL) }, //0x7E
    UnprefixedOpInfo { mnemonic: "LD A,A", cycles: 4, cycles_no_branch: 0, inst: Ld8(A, A) }, //0x7F
    UnprefixedOpInfo { mnemonic: "ADD A,B", cycles: 4, cycles_no_branch: 0, inst: Add8(B) }, //0x80
    UnprefixedOpInfo { mnemonic: "ADD A,C", cycles: 4, cycles_no_branch: 0, inst: Add8(C) }, //0x81
    UnprefixedOpInfo { mnemonic: "ADD A,D", cycles: 4, cycles_no_branch: 0, inst: Add8(D) }, //0x82
    UnprefixedOpInfo { mnemonic: "ADD A,E", cycles: 4, cycles_no_branch: 0, inst: Add8(E) }, //0x83
    UnprefixedOpInfo { mnemonic: "ADD A,H", cycles: 4, cycles_no_branch: 0, inst: Add8(H) }, //0x84
    UnprefixedOpInfo { mnemonic: "ADD A,L", cycles: 4, cycles_no_branch: 0, inst: Add8(L) }, //0x85
    UnprefixedOpInfo { mnemonic: "ADD A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Add8(AddrHL) }, //0x86
    UnprefixedOpInfo { mnemonic: "ADD A,A", cycles: 4, cycles_no_branch: 0, inst: Add8(A) }, //0x87
    UnprefixedOpInfo { mnemonic: "ADC A,B", cycles: 4, cycles_no_branch: 0, inst: Adc(B) }, //0x88
    UnprefixedOpInfo { mnemonic: "ADC A,C", cycles: 4, cycles_no_branch: 0, inst: Adc(C) }, //0x89
    UnprefixedOpInfo { mnemonic: "ADC A,D", cycles: 4, cycles_no_branch: 0, inst: Adc(D) }, //0x8A
    UnprefixedOpInfo { mnemonic: "ADC A,E", cycles: 4, cycles_no_branch: 0, inst: Adc(E) }, //0x8B
    UnprefixedOpInfo { mnemonic: "ADC A,H", cycles: 4, cycles_no_branch: 0, inst: Adc(H) }, //0x8C
    UnprefixedOpInfo { mnemonic: "ADC A,L", cycles: 4, cycles_no_branch: 0, inst: Adc(L) }, //0x8D
    UnprefixedOpInfo { mnemonic: "ADC A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Adc(AddrHL) }, //0x8E
    UnprefixedOpInfo { mnemonic: "ADC A,A", cycles: 4, cycles_no_branch: 0, inst: Adc(A) }, //0x8F
    UnprefixedOpInfo { mnemonic: "SUB B", cycles: 4, cycles_no_branch: 0, inst: Sub(B) }, //0x90
    UnprefixedOpInfo { mnemonic: "SUB C", cycles: 4, cycles_no_branch: 0, inst: Sub(C) }, //0x91
    UnprefixedOpInfo { mnemonic: "SUB D", cycles: 4, cycles_no_branch: 0, inst: Sub(D) }, //0x92
    UnprefixedOpInfo { mnemonic: "SUB E", cycles: 4, cycles_no_branch: 0, inst: Sub(E) }, //0x93
    UnprefixedOpInfo { mnemonic: "SUB H", cycles: 4, cycles_no_branch: 0, inst: Sub(H) }, //0x94
    UnprefixedOpInfo { mnemonic: "SUB L", cycles: 4, cycles_no_branch: 0, inst: Sub(L) }, //0x95
    UnprefixedOpInfo { mnemonic: "SUB (HL)", cycles: 8, cycles_no_branch: 0, inst: Sub(AddrHL) }, //0x96
    UnprefixedOpInfo { mnemonic: "SUB A", cycles: 4, cycles_no_branch: 0, inst: Sub(A) }, //0x97
    UnprefixedOpInfo { mnemonic: "SBC A,B", cycles: 4, cycles_no_branch: 0, inst: Sbc(B) }, //0x98
    UnprefixedOpInfo { mnemonic: "SBC A,C", cycles: 4, cycles_no_branch: 0, inst: Sbc(C) }, //0x99
    UnprefixedOpInfo { mnemonic: "SBC A,D", cycles: 4, cycles_no_branch: 0, inst: Sbc(D) }, //0x9A
    UnprefixedOpInfo { mnemonic: "SBC A,E", cycles: 4, cycles_no_branch: 0, inst: Sbc(E) }, //0x9B
    UnprefixedOpInfo { mnemonic: "SBC A,H", cycles: 4, cycles_no_branch: 0, inst: Sbc(H) }, //0x9C
    UnprefixedOpInfo { mnemonic: "SBC A,L", cycles: 4, cycles_no_branch: 0, inst: Sbc(L) }, //0x9D
    UnprefixedOpInfo { mnemonic: "SBC A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Sbc(AddrHL) }, //0x9E
    UnprefixedOpInfo { mnemonic: "SBC A,A", cycles: 4, cycles_no_branch: 0, inst: Sbc(A) }, //0x9F
    UnprefixedOpInfo { mnemonic: "AND B", cycles: 4, cycles_no_branch: 0, inst: And(B) }, //0xA0
    UnprefixedOpInfo { mnemonic: "AND C", cycles: 4, cycles_no_branch: 0, inst: And(C) }, //0xA1
    UnprefixedOpInfo { mnemonic: "AND D", cycles: 4, cycles_no_branch: 0, inst: And(D) }, //0xA2
    UnprefixedOpInfo { mnemonic: "AND E", cycles: 4, cycles_no_branch: 0, inst: And(E) }, //0xA3
    UnprefixedOpInfo { mnemonic: "AND H", cycles: 4, cycles_no_branch: 0, inst: And(H) }, //0xA4
    UnprefixedOpInfo { mnemonic: "AND L", cycles: 4, cycles_no_branch: 0, inst: And(L) }, //0xA5
    UnprefixedOpInfo { mnemonic: "AND (HL)", cycles: 8, cycles_no_branch: 0, inst: And(AddrHL) }, //0xA6
    UnprefixedOpInfo { mnemonic: "AND A", cycles: 4, cycles_no_branch: 0, inst: And(A) }, //0xA7
    UnprefixedOpInfo { mnemonic: "XOR B", cycles: 4, cycles_no_branch: 0, inst: Xor(B) }, //0xA8
    UnprefixedOpInfo { mnemonic: "XOR C", cycles: 4, cycles_no_branch: 0, inst: Xor(C) }, //0xA9
    UnprefixedOpInfo { mnemonic: "XOR D", cycles: 4, cycles_no_branch: 0, inst: Xor(D) }, //0xAA
    UnprefixedOpInfo { mnemonic: "XOR E", cycles: 4, cycles_no_branch: 0, inst: Xor(E) }, //0xAB
    UnprefixedOpInfo { mnemonic: "XOR H", cycles: 4, cycles_no_branch: 0, inst: Xor(H) }, //0xAC
    UnprefixedOpInfo { mnemonic: "XOR L", cycles: 4, cycles_no_branch: 0, inst: Xor(L) }, //0xAD
    UnprefixedOpInfo { mnemonic: "XOR (HL)", cycles: 8, cycles_no_branch: 0, inst: Xor(AddrHL) }, //0xAE
    UnprefixedOpInfo { mnemonic: "XOR A", cycles: 4, cycles_no_branch: 0, inst: Xor(A) }, //0xAF
    UnprefixedOpInfo { mnemonic: "OR B", cycles: 4, cycles_no_branch: 0, inst: Or(B) }, //0xB0
    UnprefixedOpInfo { mnemonic: "OR C", cycles: 4, cycles_no_branch: 0, inst: Or(C) }, //0xB1
    UnprefixedOpInfo { mnemonic: "OR D", cycles: 4, cycles_no_branch: 0, inst: Or(D) }, //0xB2
    UnprefixedOpInfo { mnemonic: "OR E", cycles: 4, cycles_no_branch: 0, inst: Or(E) }, //0xB3
    UnprefixedOpInfo { mnemonic: "OR H", cycles: 4, cycles_no_branch: 0, inst: Or(H) }, //0xB4
    UnprefixedOpInfo { mnemonic: "OR L", cycles: 4, cycles_no_branch: 0, inst: Or(L) }, //0xB5
    UnprefixedOpInfo { mnemonic: "OR (HL)", cycles: 8, cycles_no_branch: 0, inst: Or(AddrHL) }, //0xB6
    UnprefixedOpInfo { mnemonic: "OR A", cycles: 4, cycles_no_branch: 0, inst: Or(A) }, //0xB7
    UnprefixedOpInfo { mnemonic: "CP B", cycles: 4, cycles_no_branch: 0, inst: Cp(B) }, //0xB8
    UnprefixedOpInfo { mnemonic: "CP C", cycles: 4, cycles_no_branch: 0, inst: Cp(C) }, //0xB9
    UnprefixedOpInfo { mnemonic: "CP D", cycles: 4, cycles_no_branch: 0, inst: Cp(D) }, //0xBA
    UnprefixedOpInfo { mnemonic: "CP E", cycles: 4, cycles_no_branch: 0, inst: Cp(E) }, //0xBB
    UnprefixedOpInfo { mnemonic: "CP H", cycles: 4, cycles_no_branch: 0, inst: Cp(H) }, //0xBC
    UnprefixedOpInfo { mnemonic: "CP L", cycles: 4, cycles_no_branch: 0, inst: Cp(L) }, //0xBD
    UnprefixedOpInfo { mnemonic: "CP (HL)", cycles: 8, cycles_no_branch: 0, inst: Cp(AddrHL) }, //0xBE
    UnprefixedOpInfo { mnemonic: "CP A", cycles: 4, cycles_no_branch: 0, inst: Cp(A) }, //0xBF
    UnprefixedOpInfo { mnemonic: "RET NZ", cycles: 20, cycles_no_branch: 8, inst: Ret(NotZero) }, //0xC0
    UnprefixedOpInfo { mnemonic: "POP BC", cycles: 12, cycles_no_branch: 0, inst: Pop(BC) }, //0xC1
    UnprefixedOpInfo { mnemonic: "JP NZ,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(NotZero, U16) }, //0xC2
    UnprefixedOpInfo { mnemonic: "JP u16", cycles: 16, cycles_no_branch: 0, inst: Jp(Unconditional, U16) }, //0xC3
    UnprefixedOpInfo { mnemonic: "CALL NZ,u16", cycles: 24, cycles_no_branch: 12, inst: Call(NotZero) }, //0xC4
    UnprefixedOpInfo { mnemonic: "PUSH BC", cycles: 16, cycles_no_branch: 0, inst: Push(BC) }, //0xC5
    UnprefixedOpInfo { mnemonic: "ADD A,u8", cycles: 8, cycles_no_branch: 0, inst: Add8(U8) }, //0xC6
    UnprefixedOpInfo { mnemonic: "RST 00H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x00) }, //0xC7
    UnprefixedOpInfo { mnemonic: "RET Z", cycles: 20, cycles_no_branch: 8, inst: Ret(Zero) }, //0xC8
    UnprefixedOpInfo { mnemonic: "RET", cycles: 16, cycles_no_branch: 0, inst: Ret(Unconditional), }, //0xC9
    UnprefixedOpInfo { mnemonic: "JP Z,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(Zero, U16) }, //0xCA
    UnprefixedOpInfo { mnemonic: "PREFIX", cycles: 4, cycles_no_branch: 0, inst: Prefix }, //0xCB
    UnprefixedOpInfo { mnemonic: "CALL Z,u16", cycles: 24, cycles_no_branch: 12, inst: Call(Zero) }, //0xCC
    UnprefixedOpInfo { mnemonic: "CALL u16", cycles: 24, cycles_no_branch: 0, inst: Call(Unconditional), }, //0xCD
    UnprefixedOpInfo { mnemonic: "ADC A,u8", cycles: 8, cycles_no_branch: 0, inst: Adc(U8) }, //0xCE
    UnprefixedOpInfo { mnemonic: "RST 08H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x08) }, //0xCF
    UnprefixedOpInfo { mnemonic: "RET NC", cycles: 20, cycles_no_branch: 8, inst: Ret(NotCarry) }, //0xD0
    UnprefixedOpInfo { mnemonic: "POP DE", cycles: 12, cycles_no_branch: 0, inst: Pop(DE) }, //0xD1
    UnprefixedOpInfo { mnemonic: "JP NC,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(NotCarry, U16) }, //0xD2
    UnprefixedOpInfo { mnemonic: "ILLEGAL_D3", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xD3
    UnprefixedOpInfo { mnemonic: "CALL NC,u16", cycles: 24, cycles_no_branch: 12, inst: Call(NotCarry) }, //0xD4
    UnprefixedOpInfo { mnemonic: "PUSH DE", cycles: 16, cycles_no_branch: 0, inst: Push(DE) }, //0xD5
    UnprefixedOpInfo { mnemonic: "SUB u8", cycles: 8, cycles_no_branch: 0, inst: Sub(U8) }, //0xD6
    UnprefixedOpInfo { mnemonic: "RST 10H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x10) }, //0xD7
    UnprefixedOpInfo { mnemonic: "RET C", cycles: 20, cycles_no_branch: 8, inst: Ret(Carry) }, //0xD8
    UnprefixedOpInfo { mnemonic: "RETI", cycles: 16, cycles_no_branch: 0, inst: Reti }, //0xD9
    UnprefixedOpInfo { mnemonic: "JP C,u16", cycles: 16, cycles_no_branch: 12, inst: Jp(Carry, U16) }, //0xDA
    UnprefixedOpInfo { mnemonic: "ILLEGAL_DB", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xDB
    UnprefixedOpInfo { mnemonic: "CALL C,u16", cycles: 24, cycles_no_branch: 12, inst: Call(Carry) }, //0xDC
    UnprefixedOpInfo { mnemonic: "ILLEGAL_DD", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xDD
    UnprefixedOpInfo { mnemonic: "SBC A,u8", cycles: 8, cycles_no_branch: 0, inst: Sbc(U8) }, //0xDE
    UnprefixedOpInfo { mnemonic: "RST 18H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x18) }, //0xDF
    UnprefixedOpInfo { mnemonic: "LDH (a8),A", cycles: 12, cycles_no_branch: 0, inst: Ld8(LowAddrU8, A) }, //0xE0
    UnprefixedOpInfo { mnemonic: "POP HL", cycles: 12, cycles_no_branch: 0, inst: Pop(HL) }, //0xE1
    UnprefixedOpInfo { mnemonic: "LD (C),A", cycles: 8, cycles_no_branch: 0, inst: Ld8(LowAddrC, A) }, //0xE2
    UnprefixedOpInfo { mnemonic: "ILLEGAL_E3", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xE3
    UnprefixedOpInfo { mnemonic: "ILLEGAL_E4", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xE4
    UnprefixedOpInfo { mnemonic: "PUSH HL", cycles: 16, cycles_no_branch: 0, inst: Push(HL) }, //0xE5
    UnprefixedOpInfo { mnemonic: "AND u8", cycles: 8, cycles_no_branch: 0, inst: And(U8) }, //0xE6
    UnprefixedOpInfo { mnemonic: "RST 20H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x20) }, //0xE7
    UnprefixedOpInfo { mnemonic: "ADD SP,r8", cycles: 16, cycles_no_branch: 0, inst: AddI8SP }, //0xE8
    UnprefixedOpInfo { mnemonic: "JP HL", cycles: 4, cycles_no_branch: 0, inst: Jp(Unconditional, HL) }, //0xE9
    UnprefixedOpInfo { mnemonic: "LD (u16),A", cycles: 16, cycles_no_branch: 0, inst: Ld8(AddrU16, A) }, //0xEA
    UnprefixedOpInfo { mnemonic: "ILLEGAL_EB", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xEB
    UnprefixedOpInfo { mnemonic: "ILLEGAL_EC", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xEC
    UnprefixedOpInfo { mnemonic: "ILLEGAL_ED", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xED
    UnprefixedOpInfo { mnemonic: "XOR u8", cycles: 8, cycles_no_branch: 0, inst: Xor(U8) }, //0xEE
    UnprefixedOpInfo { mnemonic: "RST 28H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x28) }, //0xEF
    UnprefixedOpInfo { mnemonic: "LDH A,(a8)", cycles: 12, cycles_no_branch: 0, inst: Ld8(A, LowAddrU8) }, //0xF0
    UnprefixedOpInfo { mnemonic: "POP AF", cycles: 12, cycles_no_branch: 0, inst: Pop(AF) }, //0xF1
    UnprefixedOpInfo { mnemonic: "LD A,(C)", cycles: 8, cycles_no_branch: 0, inst: Ld8(A, LowAddrC)}, //0xF2
    UnprefixedOpInfo { mnemonic: "DI", cycles: 4, cycles_no_branch: 0, inst: Di }, //0xF3
    UnprefixedOpInfo { mnemonic: "ILLEGAL_F4", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xF4
    UnprefixedOpInfo { mnemonic: "PUSH AF", cycles: 16, cycles_no_branch: 0, inst: Push(AF) }, //0xF5
    UnprefixedOpInfo { mnemonic: "OR u8", cycles: 8, cycles_no_branch: 0, inst: Or(U8) }, //0xF6
    UnprefixedOpInfo { mnemonic: "RST 30H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x30) }, //0xF7
    UnprefixedOpInfo { mnemonic: "LD HL,SP+r8", cycles: 12, cycles_no_branch: 0, inst: Ld16(HL, SPPlusI8) }, //0xF8
    UnprefixedOpInfo { mnemonic: "LD SP,HL", cycles: 8, cycles_no_branch: 0, inst: Ld16(SP, HL) }, //0xF9
    UnprefixedOpInfo { mnemonic: "LD A,(u16)", cycles: 16, cycles_no_branch: 0, inst: Ld8(A, AddrU16) }, //0xFA
    UnprefixedOpInfo { mnemonic: "EI", cycles: 4, cycles_no_branch: 0, inst: Ei }, //0xFB
    UnprefixedOpInfo { mnemonic: "ILLEGAL_FC", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xFC
    UnprefixedOpInfo { mnemonic: "ILLEGAL_FD", cycles: 4, cycles_no_branch: 0, inst: Illegal }, //0xFD
    UnprefixedOpInfo { mnemonic: "CP u8", cycles: 8, cycles_no_branch: 0, inst: Cp(U8) }, //0xFE
    UnprefixedOpInfo { mnemonic: "RST 38H", cycles: 16, cycles_no_branch: 0, inst: Rst(0x38) }, //0xFF
];

#[rustfmt::skip]
pub const SECONDARY: [PrefixedOpInfo; 256] = [
    PrefixedOpInfo { mnemonic: "RLC B", cycles: 8, inst: Rlc(B) }, //0x00
    PrefixedOpInfo { mnemonic: "RLC C", cycles: 8, inst: Rlc(C) }, //0x01
    PrefixedOpInfo { mnemonic: "RLC D", cycles: 8, inst: Rlc(D) }, //0x02
    PrefixedOpInfo { mnemonic: "RLC E", cycles: 8, inst: Rlc(E) }, //0x03
    PrefixedOpInfo { mnemonic: "RLC H", cycles: 8, inst: Rlc(H) }, //0x04
    PrefixedOpInfo { mnemonic: "RLC L", cycles: 8, inst: Rlc(L) }, //0x05
    PrefixedOpInfo { mnemonic: "RLC (HL)", cycles: 16, inst: Rlc(AddrHL) }, //0x06
    PrefixedOpInfo { mnemonic: "RLC A", cycles: 8, inst: Rlc(A) }, //0x07
    PrefixedOpInfo { mnemonic: "RRC B", cycles: 8, inst: Rrc(B) }, //0x08
    PrefixedOpInfo { mnemonic: "RRC C", cycles: 8, inst: Rrc(C) }, //0x09
    PrefixedOpInfo { mnemonic: "RRC D", cycles: 8, inst: Rrc(D) }, //0x0A
    PrefixedOpInfo { mnemonic: "RRC E", cycles: 8, inst: Rrc(E) }, //0x0B
    PrefixedOpInfo { mnemonic: "RRC H", cycles: 8, inst: Rrc(H) }, //0x0C
    PrefixedOpInfo { mnemonic: "RRC L", cycles: 8, inst: Rrc(L) }, //0x0D
    PrefixedOpInfo { mnemonic: "RRC (HL)", cycles: 16, inst: Rrc(AddrHL) }, //0x0E
    PrefixedOpInfo { mnemonic: "RRC A", cycles: 8, inst: Rrc(A) }, //0x0F
    PrefixedOpInfo { mnemonic: "RL B", cycles: 8, inst: Rl(B) }, //0x10
    PrefixedOpInfo { mnemonic: "RL C", cycles: 8, inst: Rl(C) }, //0x11
    PrefixedOpInfo { mnemonic: "RL D", cycles: 8, inst: Rl(D) }, //0x12
    PrefixedOpInfo { mnemonic: "RL E", cycles: 8, inst: Rl(E) }, //0x13
    PrefixedOpInfo { mnemonic: "RL H", cycles: 8, inst: Rl(H) }, //0x14
    PrefixedOpInfo { mnemonic: "RL L", cycles: 8, inst: Rl(L) }, //0x15
    PrefixedOpInfo { mnemonic: "RL (HL)", cycles: 16, inst: Rl(AddrHL) }, //0x16
    PrefixedOpInfo { mnemonic: "RL A", cycles: 8, inst: Rl(A) }, //0x17
    PrefixedOpInfo { mnemonic: "RR B", cycles: 8, inst: Rr(B) }, //0x18
    PrefixedOpInfo { mnemonic: "RR C", cycles: 8, inst: Rr(C) }, //0x19
    PrefixedOpInfo { mnemonic: "RR D", cycles: 8, inst: Rr(D) }, //0x1A
    PrefixedOpInfo { mnemonic: "RR E", cycles: 8, inst: Rr(E) }, //0x1B
    PrefixedOpInfo { mnemonic: "RR H", cycles: 8, inst: Rr(H) }, //0x1C
    PrefixedOpInfo { mnemonic: "RR L", cycles: 8, inst: Rr(L) }, //0x1D
    PrefixedOpInfo { mnemonic: "RR (HL)", cycles: 16, inst: Rr(AddrHL) }, //0x1E
    PrefixedOpInfo { mnemonic: "RR A", cycles: 8, inst: Rr(A) }, //0x1F
    PrefixedOpInfo { mnemonic: "SLA B", cycles: 8, inst: Sla(B) }, //0x20
    PrefixedOpInfo { mnemonic: "SLA C", cycles: 8, inst: Sla(C) }, //0x21
    PrefixedOpInfo { mnemonic: "SLA D", cycles: 8, inst: Sla(D) }, //0x22
    PrefixedOpInfo { mnemonic: "SLA E", cycles: 8, inst: Sla(E) }, //0x23
    PrefixedOpInfo { mnemonic: "SLA H", cycles: 8, inst: Sla(H) }, //0x24
    PrefixedOpInfo { mnemonic: "SLA L", cycles: 8, inst: Sla(L) }, //0x25
    PrefixedOpInfo { mnemonic: "SLA (HL)", cycles: 16, inst: Sla(AddrHL) }, //0x26
    PrefixedOpInfo { mnemonic: "SLA A", cycles: 8, inst: Sla(A) }, //0x27
    PrefixedOpInfo { mnemonic: "SRA B", cycles: 8, inst: Sra(B) }, //0x28
    PrefixedOpInfo { mnemonic: "SRA C", cycles: 8, inst: Sra(C) }, //0x29
    PrefixedOpInfo { mnemonic: "SRA D", cycles: 8, inst: Sra(D) }, //0x2A
    PrefixedOpInfo { mnemonic: "SRA E", cycles: 8, inst: Sra(E) }, //0x2B
    PrefixedOpInfo { mnemonic: "SRA H", cycles: 8, inst: Sra(H) }, //0x2C
    PrefixedOpInfo { mnemonic: "SRA L", cycles: 8, inst: Sra(L) }, //0x2D
    PrefixedOpInfo { mnemonic: "SRA (HL)", cycles: 16, inst: Sra(AddrHL) }, //0x2E
    PrefixedOpInfo { mnemonic: "SRA A", cycles: 8, inst: Sra(A) }, //0x2F
    PrefixedOpInfo { mnemonic: "SWAP B", cycles: 8, inst: Swap(B) }, //0x30
    PrefixedOpInfo { mnemonic: "SWAP C", cycles: 8, inst: Swap(C) }, //0x31
    PrefixedOpInfo { mnemonic: "SWAP D", cycles: 8, inst: Swap(D) }, //0x32
    PrefixedOpInfo { mnemonic: "SWAP E", cycles: 8, inst: Swap(E) }, //0x33
    PrefixedOpInfo { mnemonic: "SWAP H", cycles: 8, inst: Swap(H) }, //0x34
    PrefixedOpInfo { mnemonic: "SWAP L", cycles: 8, inst: Swap(L) }, //0x35
    PrefixedOpInfo { mnemonic: "SWAP (HL)", cycles: 16, inst: Swap(AddrHL) }, //0x36
    PrefixedOpInfo { mnemonic: "SWAP A", cycles: 8, inst: Swap(A) }, //0x37
    PrefixedOpInfo { mnemonic: "SRL B", cycles: 8, inst: Srl(B) }, //0x38
    PrefixedOpInfo { mnemonic: "SRL C", cycles: 8, inst: Srl(C) }, //0x39
    PrefixedOpInfo { mnemonic: "SRL D", cycles: 8, inst: Srl(D) }, //0x3A
    PrefixedOpInfo { mnemonic: "SRL E", cycles: 8, inst: Srl(E) }, //0x3B
    PrefixedOpInfo { mnemonic: "SRL H", cycles: 8, inst: Srl(H) }, //0x3C
    PrefixedOpInfo { mnemonic: "SRL L", cycles: 8, inst: Srl(L) }, //0x3D
    PrefixedOpInfo { mnemonic: "SRL (HL)", cycles: 16, inst: Srl(AddrHL) }, //0x3E
    PrefixedOpInfo { mnemonic: "SRL A", cycles: 8, inst: Srl(A) }, //0x3F
    PrefixedOpInfo { mnemonic: "BIT 0,B", cycles: 8, inst: Bit(0, B) }, //0x40
    PrefixedOpInfo { mnemonic: "BIT 0,C", cycles: 8, inst: Bit(0, C) }, //0x41
    PrefixedOpInfo { mnemonic: "BIT 0,D", cycles: 8, inst: Bit(0, D) }, //0x42
    PrefixedOpInfo { mnemonic: "BIT 0,E", cycles: 8, inst: Bit(0, E) }, //0x43
    PrefixedOpInfo { mnemonic: "BIT 0,H", cycles: 8, inst: Bit(0, H) }, //0x44
    PrefixedOpInfo { mnemonic: "BIT 0,L", cycles: 8, inst: Bit(0, L) }, //0x45
    PrefixedOpInfo { mnemonic: "BIT 0,(HL)", cycles: 12, inst: Bit(0, AddrHL) }, //0x46
    PrefixedOpInfo { mnemonic: "BIT 0,A", cycles: 8, inst: Bit(0, A) }, //0x47
    PrefixedOpInfo { mnemonic: "BIT 1,B", cycles: 8, inst: Bit(1, B) }, //0x48
    PrefixedOpInfo { mnemonic: "BIT 1,C", cycles: 8, inst: Bit(1, C) }, //0x49
    PrefixedOpInfo { mnemonic: "BIT 1,D", cycles: 8, inst: Bit(1, D) }, //0x4A
    PrefixedOpInfo { mnemonic: "BIT 1,E", cycles: 8, inst: Bit(1, E) }, //0x4B
    PrefixedOpInfo { mnemonic: "BIT 1,H", cycles: 8, inst: Bit(1, H) }, //0x4C
    PrefixedOpInfo { mnemonic: "BIT 1,L", cycles: 8, inst: Bit(1, L) }, //0x4D
    PrefixedOpInfo { mnemonic: "BIT 1,(HL)", cycles: 12, inst: Bit(1, AddrHL) }, //0x4E
    PrefixedOpInfo { mnemonic: "BIT 1,A", cycles: 8, inst: Bit(1, A) }, //0x4F
    PrefixedOpInfo { mnemonic: "BIT 2,B", cycles: 8, inst: Bit(2, B) }, //0x50
    PrefixedOpInfo { mnemonic: "BIT 2,C", cycles: 8, inst: Bit(2, C) }, //0x51
    PrefixedOpInfo { mnemonic: "BIT 2,D", cycles: 8, inst: Bit(2, D) }, //0x52
    PrefixedOpInfo { mnemonic: "BIT 2,E", cycles: 8, inst: Bit(2, E) }, //0x53
    PrefixedOpInfo { mnemonic: "BIT 2,H", cycles: 8, inst: Bit(2, H) }, //0x54
    PrefixedOpInfo { mnemonic: "BIT 2,L", cycles: 8, inst: Bit(2, L) }, //0x55
    PrefixedOpInfo { mnemonic: "BIT 2,(HL)", cycles: 12, inst: Bit(2, AddrHL) }, //0x56
    PrefixedOpInfo { mnemonic: "BIT 2,A", cycles: 8, inst: Bit(2, A) }, //0x57
    PrefixedOpInfo { mnemonic: "BIT 3,B", cycles: 8, inst: Bit(3, B) }, //0x58
    PrefixedOpInfo { mnemonic: "BIT 3,C", cycles: 8, inst: Bit(3, C) }, //0x59
    PrefixedOpInfo { mnemonic: "BIT 3,D", cycles: 8, inst: Bit(3, D) }, //0x5A
    PrefixedOpInfo { mnemonic: "BIT 3,E", cycles: 8, inst: Bit(3, E) }, //0x5B
    PrefixedOpInfo { mnemonic: "BIT 3,H", cycles: 8, inst: Bit(3, H) }, //0x5C
    PrefixedOpInfo { mnemonic: "BIT 3,L", cycles: 8, inst: Bit(3, L) }, //0x5D
    PrefixedOpInfo { mnemonic: "BIT 3,(HL)", cycles: 12, inst: Bit(3, AddrHL) }, //0x5E
    PrefixedOpInfo { mnemonic: "BIT 3,A", cycles: 8, inst: Bit(3, A) }, //0x5F
    PrefixedOpInfo { mnemonic: "BIT 4,B", cycles: 8, inst: Bit(4, B) }, //0x60
    PrefixedOpInfo { mnemonic: "BIT 4,C", cycles: 8, inst: Bit(4, C) }, //0x61
    PrefixedOpInfo { mnemonic: "BIT 4,D", cycles: 8, inst: Bit(4, D) }, //0x62
    PrefixedOpInfo { mnemonic: "BIT 4,E", cycles: 8, inst: Bit(4, E) }, //0x63
    PrefixedOpInfo { mnemonic: "BIT 4,H", cycles: 8, inst: Bit(4, H) }, //0x64
    PrefixedOpInfo { mnemonic: "BIT 4,L", cycles: 8, inst: Bit(4, L) }, //0x65
    PrefixedOpInfo { mnemonic: "BIT 4,(HL)", cycles: 12, inst: Bit(4, AddrHL) }, //0x66
    PrefixedOpInfo { mnemonic: "BIT 4,A", cycles: 8, inst: Bit(4, A) }, //0x67
    PrefixedOpInfo { mnemonic: "BIT 5,B", cycles: 8, inst: Bit(5, B) }, //0x68
    PrefixedOpInfo { mnemonic: "BIT 5,C", cycles: 8, inst: Bit(5, C) }, //0x69
    PrefixedOpInfo { mnemonic: "BIT 5,D", cycles: 8, inst: Bit(5, D) }, //0x6A
    PrefixedOpInfo { mnemonic: "BIT 5,E", cycles: 8, inst: Bit(5, E) }, //0x6B
    PrefixedOpInfo { mnemonic: "BIT 5,H", cycles: 8, inst: Bit(5, H) }, //0x6C
    PrefixedOpInfo { mnemonic: "BIT 5,L", cycles: 8, inst: Bit(5, L) }, //0x6D
    PrefixedOpInfo { mnemonic: "BIT 5,(HL)", cycles: 12, inst: Bit(5, AddrHL) }, //0x6E
    PrefixedOpInfo { mnemonic: "BIT 5,A", cycles: 8, inst: Bit(5, A) }, //0x6F
    PrefixedOpInfo { mnemonic: "BIT 6,B", cycles: 8, inst: Bit(6, B) }, //0x70
    PrefixedOpInfo { mnemonic: "BIT 6,C", cycles: 8, inst: Bit(6, C) }, //0x71
    PrefixedOpInfo { mnemonic: "BIT 6,D", cycles: 8, inst: Bit(6, D) }, //0x72
    PrefixedOpInfo { mnemonic: "BIT 6,E", cycles: 8, inst: Bit(6, E) }, //0x73
    PrefixedOpInfo { mnemonic: "BIT 6,H", cycles: 8, inst: Bit(6, H) }, //0x74
    PrefixedOpInfo { mnemonic: "BIT 6,L", cycles: 8, inst: Bit(6, L) }, //0x75
    PrefixedOpInfo { mnemonic: "BIT 6,(HL)", cycles: 12, inst: Bit(6, AddrHL) }, //0x76
    PrefixedOpInfo { mnemonic: "BIT 6,A", cycles: 8, inst: Bit(6, A) }, //0x77
    PrefixedOpInfo { mnemonic: "BIT 7,B", cycles: 8, inst: Bit(7, B) }, //0x78
    PrefixedOpInfo { mnemonic: "BIT 7,C", cycles: 8, inst: Bit(7, C) }, //0x79
    PrefixedOpInfo { mnemonic: "BIT 7,D", cycles: 8, inst: Bit(7, D) }, //0x7A
    PrefixedOpInfo { mnemonic: "BIT 7,E", cycles: 8, inst: Bit(7, E) }, //0x7B
    PrefixedOpInfo { mnemonic: "BIT 7,H", cycles: 8, inst: Bit(7, H) }, //0x7C
    PrefixedOpInfo { mnemonic: "BIT 7,L", cycles: 8, inst: Bit(7, L) }, //0x7D
    PrefixedOpInfo { mnemonic: "BIT 7,(HL)", cycles: 12, inst: Bit(7, AddrHL) }, //0x7E
    PrefixedOpInfo { mnemonic: "BIT 7,A", cycles: 8, inst: Bit(7, A) }, //0x7F
    PrefixedOpInfo { mnemonic: "RES 0,B", cycles: 8, inst: Res(0, B) }, //0x80
    PrefixedOpInfo { mnemonic: "RES 0,C", cycles: 8, inst: Res(0, C) }, //0x81
    PrefixedOpInfo { mnemonic: "RES 0,D", cycles: 8, inst: Res(0, D) }, //0x82
    PrefixedOpInfo { mnemonic: "RES 0,E", cycles: 8, inst: Res(0, E) }, //0x83
    PrefixedOpInfo { mnemonic: "RES 0,H", cycles: 8, inst: Res(0, H) }, //0x84
    PrefixedOpInfo { mnemonic: "RES 0,L", cycles: 8, inst: Res(0, L) }, //0x85
    PrefixedOpInfo { mnemonic: "RES 0,(HL)", cycles: 16, inst: Res(0, AddrHL) }, //0x86
    PrefixedOpInfo { mnemonic: "RES 0,A", cycles: 8, inst: Res(0, A) }, //0x87
    PrefixedOpInfo { mnemonic: "RES 1,B", cycles: 8, inst: Res(1, B) }, //0x88
    PrefixedOpInfo { mnemonic: "RES 1,C", cycles: 8, inst: Res(1, C) }, //0x89
    PrefixedOpInfo { mnemonic: "RES 1,D", cycles: 8, inst: Res(1, D) }, //0x8A
    PrefixedOpInfo { mnemonic: "RES 1,E", cycles: 8, inst: Res(1, E) }, //0x8B
    PrefixedOpInfo { mnemonic: "RES 1,H", cycles: 8, inst: Res(1, H) }, //0x8C
    PrefixedOpInfo { mnemonic: "RES 1,L", cycles: 8, inst: Res(1, L) }, //0x8D
    PrefixedOpInfo { mnemonic: "RES 1,(HL)", cycles: 16, inst: Res(1, AddrHL) }, //0x8E
    PrefixedOpInfo { mnemonic: "RES 1,A", cycles: 8, inst: Res(1, A) }, //0x8F
    PrefixedOpInfo { mnemonic: "RES 2,B", cycles: 8, inst: Res(2, B) }, //0x90
    PrefixedOpInfo { mnemonic: "RES 2,C", cycles: 8, inst: Res(2, C) }, //0x91
    PrefixedOpInfo { mnemonic: "RES 2,D", cycles: 8, inst: Res(2, D) }, //0x92
    PrefixedOpInfo { mnemonic: "RES 2,E", cycles: 8, inst: Res(2, E) }, //0x93
    PrefixedOpInfo { mnemonic: "RES 2,H", cycles: 8, inst: Res(2, H) }, //0x94
    PrefixedOpInfo { mnemonic: "RES 2,L", cycles: 8, inst: Res(2, L) }, //0x95
    PrefixedOpInfo { mnemonic: "RES 2,(HL)", cycles: 16, inst: Res(2, AddrHL) }, //0x96
    PrefixedOpInfo { mnemonic: "RES 2,A", cycles: 8, inst: Res(2, A) }, //0x97
    PrefixedOpInfo { mnemonic: "RES 3,B", cycles: 8, inst: Res(3, B) }, //0x98
    PrefixedOpInfo { mnemonic: "RES 3,C", cycles: 8, inst: Res(3, C) }, //0x99
    PrefixedOpInfo { mnemonic: "RES 3,D", cycles: 8, inst: Res(3, D) }, //0x9A
    PrefixedOpInfo { mnemonic: "RES 3,E", cycles: 8, inst: Res(3, E) }, //0x9B
    PrefixedOpInfo { mnemonic: "RES 3,H", cycles: 8, inst: Res(3, H) }, //0x9C
    PrefixedOpInfo { mnemonic: "RES 3,L", cycles: 8, inst: Res(3, L) }, //0x9D
    PrefixedOpInfo { mnemonic: "RES 3,(HL)", cycles: 16, inst: Res(3, AddrHL) }, //0x9E
    PrefixedOpInfo { mnemonic: "RES 3,A", cycles: 8, inst: Res(3, A) }, //0x9F
    PrefixedOpInfo { mnemonic: "RES 4,B", cycles: 8, inst: Res(4, B) }, //0xA0
    PrefixedOpInfo { mnemonic: "RES 4,C", cycles: 8, inst: Res(4, C) }, //0xA1
    PrefixedOpInfo { mnemonic: "RES 4,D", cycles: 8, inst: Res(4, D) }, //0xA2
    PrefixedOpInfo { mnemonic: "RES 4,E", cycles: 8, inst: Res(4, E) }, //0xA3
    PrefixedOpInfo { mnemonic: "RES 4,H", cycles: 8, inst: Res(4, H) }, //0xA4
    PrefixedOpInfo { mnemonic: "RES 4,L", cycles: 8, inst: Res(4, L) }, //0xA5
    PrefixedOpInfo { mnemonic: "RES 4,(HL)", cycles: 16, inst: Res(4, AddrHL) }, //0xA6
    PrefixedOpInfo { mnemonic: "RES 4,A", cycles: 8, inst: Res(4, A) }, //0xA7
    PrefixedOpInfo { mnemonic: "RES 5,B", cycles: 8, inst: Res(5, B) }, //0xA8
    PrefixedOpInfo { mnemonic: "RES 5,C", cycles: 8, inst: Res(5, C) }, //0xA9
    PrefixedOpInfo { mnemonic: "RES 5,D", cycles: 8, inst: Res(5, D) }, //0xAA
    PrefixedOpInfo { mnemonic: "RES 5,E", cycles: 8, inst: Res(5, E) }, //0xAB
    PrefixedOpInfo { mnemonic: "RES 5,H", cycles: 8, inst: Res(5, H) }, //0xAC
    PrefixedOpInfo { mnemonic: "RES 5,L", cycles: 8, inst: Res(5, L) }, //0xAD
    PrefixedOpInfo { mnemonic: "RES 5,(HL)", cycles: 16, inst: Res(5, AddrHL) }, //0xAE
    PrefixedOpInfo { mnemonic: "RES 5,A", cycles: 8, inst: Res(5, A) }, //0xAF
    PrefixedOpInfo { mnemonic: "RES 6,B", cycles: 8, inst: Res(6, B) }, //0xB0
    PrefixedOpInfo { mnemonic: "RES 6,C", cycles: 8, inst: Res(6, C) }, //0xB1
    PrefixedOpInfo { mnemonic: "RES 6,D", cycles: 8, inst: Res(6, D) }, //0xB2
    PrefixedOpInfo { mnemonic: "RES 6,E", cycles: 8, inst: Res(6, E) }, //0xB3
    PrefixedOpInfo { mnemonic: "RES 6,H", cycles: 8, inst: Res(6, H) }, //0xB4
    PrefixedOpInfo { mnemonic: "RES 6,L", cycles: 8, inst: Res(6, L) }, //0xB5
    PrefixedOpInfo { mnemonic: "RES 6,(HL)", cycles: 16, inst: Res(6, AddrHL) }, //0xB6
    PrefixedOpInfo { mnemonic: "RES 6,A", cycles: 8, inst: Res(6, A) }, //0xB7
    PrefixedOpInfo { mnemonic: "RES 7,B", cycles: 8, inst: Res(7, B) }, //0xB8
    PrefixedOpInfo { mnemonic: "RES 7,C", cycles: 8, inst: Res(7, C) }, //0xB9
    PrefixedOpInfo { mnemonic: "RES 7,D", cycles: 8, inst: Res(7, D) }, //0xBA
    PrefixedOpInfo { mnemonic: "RES 7,E", cycles: 8, inst: Res(7, E) }, //0xBB
    PrefixedOpInfo { mnemonic: "RES 7,H", cycles: 8, inst: Res(7, H) }, //0xBC
    PrefixedOpInfo { mnemonic: "RES 7,L", cycles: 8, inst: Res(7, L) }, //0xBD
    PrefixedOpInfo { mnemonic: "RES 7,(HL)", cycles: 16, inst: Res(7, AddrHL) }, //0xBE
    PrefixedOpInfo { mnemonic: "RES 7,A", cycles: 8, inst: Res(7, A) }, //0xBF
    PrefixedOpInfo { mnemonic: "SET 0,B", cycles: 8, inst: Set(0, B) }, //0xC0
    PrefixedOpInfo { mnemonic: "SET 0,C", cycles: 8, inst: Set(0, C) }, //0xC1
    PrefixedOpInfo { mnemonic: "SET 0,D", cycles: 8, inst: Set(0, D) }, //0xC2
    PrefixedOpInfo { mnemonic: "SET 0,E", cycles: 8, inst: Set(0, E) }, //0xC3
    PrefixedOpInfo { mnemonic: "SET 0,H", cycles: 8, inst: Set(0, H) }, //0xC4
    PrefixedOpInfo { mnemonic: "SET 0,L", cycles: 8, inst: Set(0, L) }, //0xC5
    PrefixedOpInfo { mnemonic: "SET 0,(HL)", cycles: 16, inst: Set(0, AddrHL) }, //0xC6
    PrefixedOpInfo { mnemonic: "SET 0,A", cycles: 8, inst: Set(0, A) }, //0xC7
    PrefixedOpInfo { mnemonic: "SET 1,B", cycles: 8, inst: Set(1, B) }, //0xC8
    PrefixedOpInfo { mnemonic: "SET 1,C", cycles: 8, inst: Set(1, C) }, //0xC9
    PrefixedOpInfo { mnemonic: "SET 1,D", cycles: 8, inst: Set(1, D) }, //0xCA
    PrefixedOpInfo { mnemonic: "SET 1,E", cycles: 8, inst: Set(1, E) }, //0xCB
    PrefixedOpInfo { mnemonic: "SET 1,H", cycles: 8, inst: Set(1, H) }, //0xCC
    PrefixedOpInfo { mnemonic: "SET 1,L", cycles: 8, inst: Set(1, L) }, //0xCD
    PrefixedOpInfo { mnemonic: "SET 1,(HL)", cycles: 16, inst: Set(1, AddrHL) }, //0xCE
    PrefixedOpInfo { mnemonic: "SET 1,A", cycles: 8, inst: Set(1, A) }, //0xCF
    PrefixedOpInfo { mnemonic: "SET 2,B", cycles: 8, inst: Set(2, B) }, //0xD0
    PrefixedOpInfo { mnemonic: "SET 2,C", cycles: 8, inst: Set(2, C) }, //0xD1
    PrefixedOpInfo { mnemonic: "SET 2,D", cycles: 8, inst: Set(2, D) }, //0xD2
    PrefixedOpInfo { mnemonic: "SET 2,E", cycles: 8, inst: Set(2, E) }, //0xD3
    PrefixedOpInfo { mnemonic: "SET 2,H", cycles: 8, inst: Set(2, H) }, //0xD4
    PrefixedOpInfo { mnemonic: "SET 2,L", cycles: 8, inst: Set(2, L) }, //0xD5
    PrefixedOpInfo { mnemonic: "SET 2,(HL)", cycles: 16, inst: Set(2, AddrHL) }, //0xD6
    PrefixedOpInfo { mnemonic: "SET 2,A", cycles: 8, inst: Set(2, A) }, //0xD7
    PrefixedOpInfo { mnemonic: "SET 3,B", cycles: 8, inst: Set(3, B) }, //0xD8
    PrefixedOpInfo { mnemonic: "SET 3,C", cycles: 8, inst: Set(3, C) }, //0xD9
    PrefixedOpInfo { mnemonic: "SET 3,D", cycles: 8, inst: Set(3, D) }, //0xDA
    PrefixedOpInfo { mnemonic: "SET 3,E", cycles: 8, inst: Set(3, E) }, //0xDB
    PrefixedOpInfo { mnemonic: "SET 3,H", cycles: 8, inst: Set(3, H) }, //0xDC
    PrefixedOpInfo { mnemonic: "SET 3,L", cycles: 8, inst: Set(3, L) }, //0xDD
    PrefixedOpInfo { mnemonic: "SET 3,(HL)", cycles: 16, inst: Set(3, AddrHL) }, //0xDE
    PrefixedOpInfo { mnemonic: "SET 3,A", cycles: 8, inst: Set(3, A) }, //0xDF
    PrefixedOpInfo { mnemonic: "SET 4,B", cycles: 8, inst: Set(4, B) }, //0xE0
    PrefixedOpInfo { mnemonic: "SET 4,C", cycles: 8, inst: Set(4, C) }, //0xE1
    PrefixedOpInfo { mnemonic: "SET 4,D", cycles: 8, inst: Set(4, D) }, //0xE2
    PrefixedOpInfo { mnemonic: "SET 4,E", cycles: 8, inst: Set(4, E) }, //0xE3
    PrefixedOpInfo { mnemonic: "SET 4,H", cycles: 8, inst: Set(4, H) }, //0xE4
    PrefixedOpInfo { mnemonic: "SET 4,L", cycles: 8, inst: Set(4, L) }, //0xE5
    PrefixedOpInfo { mnemonic: "SET 4,(HL)", cycles: 16, inst: Set(4, AddrHL) }, //0xE6
    PrefixedOpInfo { mnemonic: "SET 4,A", cycles: 8, inst: Set(4, A) }, //0xE7
    PrefixedOpInfo { mnemonic: "SET 5,B", cycles: 8, inst: Set(5, B) }, //0xE8
    PrefixedOpInfo { mnemonic: "SET 5,C", cycles: 8, inst: Set(5, C) }, //0xE9
    PrefixedOpInfo { mnemonic: "SET 5,D", cycles: 8, inst: Set(5, D) }, //0xEA
    PrefixedOpInfo { mnemonic: "SET 5,E", cycles: 8, inst: Set(5, E) }, //0xEB
    PrefixedOpInfo { mnemonic: "SET 5,H", cycles: 8, inst: Set(5, H) }, //0xEC
    PrefixedOpInfo { mnemonic: "SET 5,L", cycles: 8, inst: Set(5, L) }, //0xED
    PrefixedOpInfo { mnemonic: "SET 5,(HL)", cycles: 16, inst: Set(5, AddrHL) }, //0xEE
    PrefixedOpInfo { mnemonic: "SET 5,A", cycles: 8, inst: Set(5, A) }, //0xEF
    PrefixedOpInfo { mnemonic: "SET 6,B", cycles: 8, inst: Set(6, B) }, //0xF0
    PrefixedOpInfo { mnemonic: "SET 6,C", cycles: 8, inst: Set(6, C) }, //0xF1
    PrefixedOpInfo { mnemonic: "SET 6,D", cycles: 8, inst: Set(6, D) }, //0xF2
    PrefixedOpInfo { mnemonic: "SET 6,E", cycles: 8, inst: Set(6, E) }, //0xF3
    PrefixedOpInfo { mnemonic: "SET 6,H", cycles: 8, inst: Set(6, H) }, //0xF4
    PrefixedOpInfo { mnemonic: "SET 6,L", cycles: 8, inst: Set(6, L) }, //0xF5
    PrefixedOpInfo { mnemonic: "SET 6,(HL)", cycles: 16, inst: Set(6, AddrHL) }, //0xF6
    PrefixedOpInfo { mnemonic: "SET 6,A", cycles: 8, inst: Set(6, A) }, //0xF7
    PrefixedOpInfo { mnemonic: "SET 7,B", cycles: 8, inst: Set(7, B) }, //0xF8
    PrefixedOpInfo { mnemonic: "SET 7,C", cycles: 8, inst: Set(7, C) }, //0xF9
    PrefixedOpInfo { mnemonic: "SET 7,D", cycles: 8, inst: Set(7, D) }, //0xFA
    PrefixedOpInfo { mnemonic: "SET 7,E", cycles: 8, inst: Set(7, E) }, //0xFB
    PrefixedOpInfo { mnemonic: "SET 7,H", cycles: 8, inst: Set(7, H) }, //0xFC
    PrefixedOpInfo { mnemonic: "SET 7,L", cycles: 8, inst: Set(7, L) }, //0xFD
    PrefixedOpInfo { mnemonic: "SET 7,(HL)", cycles: 16, inst: Set(7, AddrHL) }, //0xFE
    PrefixedOpInfo { mnemonic: "SET 7,A", cycles: 8, inst: Set(7, A) }, //0xFF
];

pub struct UnprefixedOpInfo {
    pub mnemonic: &'static str,
    pub cycles: usize,
    pub cycles_no_branch: usize,
    pub inst: OpcodeUnprefixed,
}

pub struct PrefixedOpInfo {
    pub mnemonic: &'static str,
    pub cycles: usize,
    pub inst: OpcodePrefixed,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    U8,
    I8,
    AddrU16,
    AddrBC,
    AddrDE,
    AddrHL,
    AddrHLInc,
    AddrHLDec,
    LowAddrC,
    LowAddrU8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op16 {
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
pub enum OpcodeUnprefixed {
    Nop,

    Ld16(Op16, Op16),
    Ld8(Op8, Op8),
    LdU16StackAddr,

    Inc8(Op8),
    Inc16(Op16),

    Dec8(Op8),
    Dec16(Op16),

    Add16(Op16),
    Add8(Op8),
    AddI8SP,

    Stop,
    Halt,

    Jp(Condition, Op16),
    Jr(Condition),
    Ret(Condition),

    Rlca,
    Rrca,
    Rla,
    Rra,

    Daa,
    Cpl,
    Scf,
    Ccf,
    Adc(Op8),
    Sub(Op8),
    Sbc(Op8),
    And(Op8),
    Xor(Op8),
    Or(Op8),
    Cp(Op8),
    Pop(Op16),
    Call(Condition),
    Push(Op16),
    Rst(u16),
    Prefix,
    Reti,
    Di,
    Ei,

    Illegal,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpcodePrefixed {
    Rlc(Op8),
    Rrc(Op8),

    Rl(Op8),
    Rr(Op8),

    Sla(Op8),
    Sra(Op8),
    Srl(Op8),

    Swap(Op8),

    Bit(u8, Op8),
    Res(u8, Op8),
    Set(u8, Op8),
}
