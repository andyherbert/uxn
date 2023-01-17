use std::{error::Error, fmt, io::Read, path::Path};

pub struct Memory {
    program_counter: u16,
    bytes: [u8; 64 * 1024],
}

impl Default for Memory {
    fn default() -> Memory {
        let pc = 0x100;
        let memory = [0; 64 * 1024];
        Memory {
            program_counter: pc,
            bytes: memory,
        }
    }
}

fn format_hex(bytes: &[u8]) -> String {
    let count = bytes.iter().rev().filter(|byte| **byte == 0x00).count();
    let string = bytes[0..bytes.len() - count]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<String>>()
        .join(",");
    let end_of_string = match count {
        0 => ")".to_string(),
        1 => "... and 1 empty byte".to_string(),
        _ if count == bytes.len() => "empty)".to_string(),
        _ => format!("... and {count} empty bytes)"),
    };
    format!("({string}{end_of_string}")
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program counter: {}", self.program_counter)?;
        writeln!(f, "Zero page: {}", format_hex(&self.bytes[0..256]))?;
        writeln!(f, "Memory: {}", format_hex(&self.bytes[256..64 * 1024]))?;
        Ok(())
    }
}

impl Memory {
    pub fn get_string(&self, mut addr: u16) -> String {
        let mut string = String::new();
        loop {
            let byte = self.bytes[addr as usize];
            if byte == 0 {
                break;
            } else {
                string.push(byte as char);
                addr = addr.wrapping_add(1);
            }
        }
        string
    }

    pub fn peek_u8s(&self, mut addr: u16, length: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(length as usize);
        for _ in 0..length {
            bytes.push(self.bytes[addr as usize]);
            addr = addr.wrapping_add(1);
        }
        bytes
    }

    pub fn poke_u8s(&mut self, mut addr: u16, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.bytes[addr as usize] = *byte;
            addr = addr.wrapping_add(1);
        }
    }

    pub fn load_bytes(&mut self, src: &[u8]) {
        self.bytes[0x100..0x100 + src.len()].copy_from_slice(src)
    }

    pub fn load_rom(&mut self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let src = {
            let mut file = std::fs::File::open(path)?;
            let mut buf = vec![];
            file.read_to_end(&mut buf)?;
            buf
        };
        self.load_bytes(&src);
        Ok(())
    }

    pub fn peek_u8(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    pub fn peek_u16(&self, addr: u16) -> u16 {
        let low = self.bytes[addr as usize];
        let high = self.bytes[addr.wrapping_add(1) as usize];
        u16::from_be_bytes([low, high])
    }

    pub fn peek_u8_rel(&self, delta: i8) -> u8 {
        let addr = self.program_counter.wrapping_add(delta as u16);
        self.peek_u8(addr)
    }

    pub fn peek_u16_rel(&self, delta: i8) -> u16 {
        let addr = self.program_counter.wrapping_add(delta as u16);
        self.peek_u16(addr)
    }

    pub fn next_u8(&mut self) -> u8 {
        let value = self.peek_u8(self.program_counter);
        self.jump_rel(1);
        value
    }

    pub fn current_operation(&self) -> u8 {
        self.peek_u8(self.program_counter)
    }

    pub fn next_u16(&mut self) -> u16 {
        let value = self.peek_u16(self.program_counter);
        self.jump_rel(2);
        value
    }

    pub fn poke_u8(&mut self, addr: u16, byte: u8) {
        self.bytes[addr as usize] = byte;
    }

    pub fn poke_u16(&mut self, addr: u16, short: u16) {
        let bytes = short.to_be_bytes();
        self.bytes[addr as usize] = bytes[0];
        self.bytes[addr.wrapping_add(1) as usize] = bytes[1];
    }

    pub fn poke_u8_rel(&mut self, delta: i8, byte: u8) {
        let addr = self.program_counter.wrapping_add(delta as u16);
        self.poke_u8(addr, byte);
    }

    pub fn poke_u16_rel(&mut self, delta: i8, short: u16) {
        let addr = self.program_counter.wrapping_add(delta as u16);
        self.poke_u16(addr, short);
    }

    pub fn jump(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    pub fn jump_rel(&mut self, delta: i8) {
        self.program_counter = self.program_counter.wrapping_add(delta as u16);
    }

    pub fn pc_value(&self) -> u16 {
        self.program_counter
    }
}
