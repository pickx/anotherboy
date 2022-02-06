    OpInfo { mnemonic: "ADD A,B", cycles: 4, cycles_no_branch: 0, inst: Add8(A, B) }, //0x80
    OpInfo { mnemonic: "ADD A,C", cycles: 4, cycles_no_branch: 0, inst: Add8(A, C) }, //0x81
    OpInfo { mnemonic: "ADD A,D", cycles: 4, cycles_no_branch: 0, inst: Add8(A, D) }, //0x82
    OpInfo { mnemonic: "ADD A,E", cycles: 4, cycles_no_branch: 0, inst: Add8(A, E) }, //0x83
    OpInfo { mnemonic: "ADD A,H", cycles: 4, cycles_no_branch: 0, inst: Add8(A, H) }, //0x84
    OpInfo { mnemonic: "ADD A,L", cycles: 4, cycles_no_branch: 0, inst: Add8(A, L) }, //0x85
    OpInfo { mnemonic: "ADD A,(HL)", cycles: 8, cycles_no_branch: 0, inst: Add8(A, AddrHL) }, //0x86
    OpInfo { mnemonic: "ADD A,A", cycles: 4, cycles_no_branch: 0, inst: Add8(A, A) }, //0x87
    OpInfo { mnemonic: "ADD A,u8", cycles: 8, cycles_no_branch: 0, inst: Add8(A, U8) }, //0xC6
    Add8(Operand8, Operand8),
