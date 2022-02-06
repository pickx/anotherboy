use num_enum::TryFromPrimitive;
use std::{convert::TryInto, fmt::Debug, str::from_utf8};

use anyhow::{bail, Context, Result};

const BANK_SIZE: usize = 16 * 1024;
type Bank = [u8; BANK_SIZE];

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[derive(Debug)]
pub struct Cartridge {
    file_name: String,
    rom: Vec<u8>,
    num_banks: usize,
    cur_bank: usize,
    pub header: CartridgeHeader,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
#[allow(non_camel_case_types)]
enum CartridgeType {
    ROM_ONLY = 0x00,
    MBC1 = 0x01,
    MBC1_RAM = 0x02,
    MBC1_RAM_BATTERY = 0x03,
    MBC2 = 0x05,
    MBC2_BATTERY = 0x06,
    ROM_RAM = 0x08,
    ROM_RAM_BATTERY = 0x09,
    MMM01 = 0x0B,
    MMM01_RAM = 0x0C,
    MMM01_RAM_BATTERY = 0x0D,
    MBC3_TIMER_BATTERY = 0x0F,
    MBC3_TIMER_RAM_BATTERY = 0x10,
    MBC3 = 0x11,
    MBC3_RAM = 0x12,
    MBC3_RAM_BATTERY = 0x13,
    MBC5 = 0x19,
    MBC5_RAM = 0x1A,
    MBC5_RAM_BATTERY = 0x1B,
    MBC5_RUMBLE = 0x1C,
    MBC5_RUMBLE_RAM = 0x1D,
    MBC5_RUMBLE_RAM_BATTERY = 0x1E,
    MBC6 = 0x20,
    MBC7_SENSOR_RUMBLE_RAM_BATTERY = 0x22,
    POCKET_CAMERA = 0xFC,
    BANDAI_TAMA5 = 0xFD,
    HuC3 = 0xFE,
    HuC1_RAM_BATTERY = 0xFF,
}

pub struct CartridgeHeader {
    title: [u8; 16],
    licensee_code: u8,
    cgb_support: bool,
    sgb_support: bool,
    cartridge_type: CartridgeType,
    rom_banks: u8,
    ram_banks: u8, //each RAM bank is 8KB
}

impl Debug for CartridgeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = self
            .title
            .iter()
            .map_while(|&b| {
                if (32..=96).contains(&b) {
                    Some(b as char)
                } else {
                    None
                }
            })
            .collect::<String>();
        f.debug_struct("CartridgeHeader")
            .field("title", &title)
            .field("licensee_code", &self.licensee_code)
            .field("cgb_support", &self.cgb_support)
            .field("sgb_support", &self.sgb_support)
            .field("cartridge_type", &self.cartridge_type)
            .field("rom_banks", &self.rom_banks)
            .field("ram_banks", &self.ram_banks)
            .finish()
    }
}

impl CartridgeHeader {
    fn from_rom(rom: &[u8]) -> Result<CartridgeHeader> {
        if rom.len() < 0x150 {
            bail!("ROM size smaller than expected")
        }

        if rom[0x104..=0x133] != NINTENDO_LOGO {
            bail!("Nintendo Logo mismatch")
        }

        let title: [u8; 16] = rom[0x0134..=0x0143].try_into()?;

        let uses_new_licensee = rom[0x14B] == 0x33;
        dbg!(&uses_new_licensee);

        let licensee_code = if uses_new_licensee {
            let new_licensee_bits = &rom[0x144..=0x145];
            from_utf8(new_licensee_bits)?.parse()?
        } else {
            rom[0x148]
        };

        dbg!(&rom[0x144..=0x145]);

        let cgb_support = uses_new_licensee && (title[15] == 0x80 || title[15] == 0xC0); //TODO: C0 should disallow old Gameboy from running this

        let sgb_support = rom[0x146] == 0x03;

        let cartridge_type = CartridgeType::try_from_primitive(rom[0x147])?;

        let rom_banks = match rom[0x148] {
            exp @ 0x0..=0x8 => {
                let exp = u32::from(exp);
                2_u8.pow(exp + 1)
            }
            0x52 => 72, //1.1 MByte
            0x53 => 80, //1.2 MByte
            0x54 => 96, //1.5 MByte
            _ => bail!("invalid value for number of rom banks"),
        };

        let ram_banks = match rom[0x149] {
            0x00 => 0,
            0x02 => 1,  //8KB
            0x03 => 4,  //32KB
            0x04 => 16, //128KB
            0x05 => 8,  //64KB
            _ => bail!("invalid value for number of ram banks"),
        };

        let header = CartridgeHeader {
            title,
            cgb_support,
            licensee_code,
            sgb_support,
            cartridge_type,
            rom_banks,
            ram_banks,
        };

        Ok(header)
    }
}

impl Cartridge {
    pub fn new(file_name: &str, rom: Vec<u8>) -> Result<Cartridge> {
        let data_len = rom.len();

        let rom = if data_len % BANK_SIZE == 0 {
            rom
        } else {
            bail!("ROM size not alligned to BANK_SIZE")
        };

        let file_name = file_name.to_string();

        let num_banks = data_len / BANK_SIZE;

        let cur_bank = if num_banks > 1 { 1 } else { 0 };

        let header = CartridgeHeader::from_rom(&rom)?;

        let cart = Self {
            file_name,
            rom,
            num_banks,
            cur_bank,
            header,
        };
        Ok(cart)
    }

    pub fn bank0(&self) -> &[u8] {
        self.nth_bank(0).expect("could not get bank")
    }

    pub fn bank0_mut(&mut self) -> &mut [u8] {
        self.nth_bank_mut(0).expect("could not get bank")
    }

    pub fn cur_bank(&self) -> &[u8] {
        self.nth_bank(self.cur_bank).expect("could not get bank")
    }

    pub fn cur_bank_mut(&mut self) -> &mut [u8] {
        self.nth_bank_mut(self.cur_bank)
            .expect("could not get bank")
    }

    fn nth_bank(&self, bank_num: usize) -> Option<&[u8]> {
        match bank_num {
            n if n <= self.num_banks => {
                let bank = &self.rom[(bank_num * BANK_SIZE)..=((bank_num + 1) * BANK_SIZE)];
                Some(bank)
            }
            _ => None,
        }
    }

    fn nth_bank_mut(&mut self, bank_num: usize) -> Option<&mut [u8]> {
        match bank_num {
            n if n <= self.num_banks => {
                let bank = &mut self.rom[(bank_num * BANK_SIZE)..=((bank_num + 1) * BANK_SIZE)];
                Some(bank)
            }
            _ => None,
        }
    }
}

#[test]
fn parsing_new_licensee_bits() -> Result<()> {
    let new_licensee_bits: [u8; 2] = [0x31, 0x32];

    let parsed: u8 = from_utf8(&new_licensee_bits)?.parse()?;
    println!("Parsed as {parsed}");
    Ok(())
}
