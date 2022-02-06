use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct OpcodeInfo {
    mnemonic: String,
    bytes: u8,
    cycles: u8,
    cycles_taken: Option<u8>,
    operands: Vec<Operand>,
}

#[derive(Serialize, Deserialize)]
enum Operand {
    R8(Reg8),
    R16(Reg16),
    D8(u8),
    D16(u16),
    A8(u8),
    Offset(u8),
    Bits(u8),
    Condition(Condition),
    RstVec(u8),
}

#[derive(Serialize, Deserialize)]
enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Serialize, Deserialize)]
enum Reg16 {
    BC,
    DE,
    HL,
}

#[derive(Serialize, Deserialize)]
enum Condition {
    Z,
    NZ,
    C,
    NC,
}

enum Opcode {
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    RET,
    POP,
    JP,
    CALL,
    PUSH,
    RST,
    PREFIX,
    RETI,
    LDH,
    DI,
    EI,
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
    ILLEGAL,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;

    let data = std::fs::read_to_string("./src/Opcodes.json").expect("could not read file");

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(&data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
