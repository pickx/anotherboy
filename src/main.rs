#![allow(unused)]

use anyhow::Context;
use anyhow::Result;
use cartridge::Cartridge;
use cpu::Cpu;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;
mod cartridge;
mod cpu;

// mod opcode;
mod opcode;
mod util;

fn main() -> Result<()> {
    let path = Path::new("Tetris.gb");
    let rom = std::fs::read(path)?;
    let file_name = path
        .file_name()
        .and_then(OsStr::to_str)
        .context("bad filename")?;
    let cartridge = Cartridge::new(file_name, rom)?;
    let cpu = Cpu::new(cartridge);
    cpu.debug_header();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
