#![allow(unused)]

use anyhow::Result;
use serde_json::{json, Value};
use std::fs::read_to_string;
mod cartridge;
mod cpu;

// mod opcode;
mod opcode;
mod util;

fn main() -> Result<()> {
    // let rom_path = "src/Dr. Mario (World).gb";
    // let mut rom = std::fs::read(rom_path).expect("couldn't open rom file");

    // let cart = cartridge::Cartridge::new(rom);

    let data = read_to_string("src/cb_prefixed.json")?;
    let values: Value = serde_json::from_str(&data)?;
    for i in 0x00..=0xFF {
        let hex = format!("0x{:02X}", i);
        let val = &values[&hex];

        let (cycles, cycles_no_branch) = {
            let cycles_arr = &val["cycles"];
            (
                &cycles_arr[0].clone(),
                cycles_arr
                    .get(1)
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| json!(0).to_string()),
            )
        };

        let operands = val["operands"].clone();

        let op1 = {
            let mut op1 = operands
                .get(0)
                .and_then(|op| op.get("name"))
                .cloned()
                .unwrap_or(json!(null));

            if op1.is_string() {
                let mut op1_str = op1.to_string();
                //remove quotes
                op1_str.remove(0);
                op1_str.pop();

                op1_str = op1_str.replace('d', "u");

                if operands[0].get("increment").is_some() {
                    op1_str.push('+');
                } else if operands[0].get("decrement").is_some() {
                    op1_str.push('-');
                }

                if operands[0].get("immediate").is_some() {
                    op1_str.insert(0, '(');
                    op1_str.push(')');
                };
                op1 = json!(op1_str);
            }

            op1
        };

        let op2 = {
            let mut op2 = operands
                .get(1)
                .and_then(|op| op.get("name"))
                .cloned()
                .unwrap_or(json!(null));

            if op2.is_string() {
                let mut op2_str = op2.to_string();
                op2_str.remove(0);
                op2_str.pop();

                op2_str = op2_str.replace('d', "u");

                if operands[1].get("increment").is_some() {
                    op2_str.push('+');
                } else if operands[1].get("decrement").is_some() {
                    op2_str.push('-');
                }

                if operands[1].get("immediate").is_some() {
                    op2_str.insert(0, '(');
                    op2_str.push(')');
                };

                op2 = json!(op2_str);
            };

            op2
        };

        let mnemonic_prefix = val["mnemonic"].as_str().unwrap();
        let mnemonic = if op1.is_null() {
            format!("\"{}\"", mnemonic_prefix)
        } else if op2.is_null() {
            format!("\"{} {}\"", mnemonic_prefix, op1.as_str().unwrap())
        } else {
            format!(
                "\"{} {},{}\"",
                mnemonic_prefix,
                op1.as_str().unwrap(),
                op2.as_str().unwrap()
            )
        };

        println!(
            "OpInfo {{ mnemonic: {}, cycles: {}, cycles_no_branch: {}, inst: Nop }}, //{}",
            mnemonic, cycles, cycles_no_branch, &hex
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
