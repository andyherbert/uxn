mod devices;
mod error;
mod machine;
mod memory;
mod op_codes;
mod stack;
use getch::Getch;
pub use machine::{Machine, MachineEvent};
use std::error::Error;

fn event_loop(path: &str) -> Result<u8, Box<dyn Error>> {
    let mut uxn = Machine::new();
    uxn.memory.load_rom(path)?;
    uxn.devices.use_virtual_file_system();
    // uxn.devices
    // .use_file_interface(FileInterface::new_with_path("../", true));
    let getch = Getch::new();
    loop {
        match uxn.run()? {
            MachineEvent::Break => {
                if let Some(addr) = uxn.devices.console_vector() {
                    let byte = getch.getch()?;
                    uxn.devices.console_event_value(byte);
                    uxn.memory.jump(addr);
                }
            }
            MachineEvent::Halt(byte) => return Ok(byte),
        }
    }
}

fn main() {
    match event_loop("roms/devices/file.rom") {
        Ok(value) => {
            dbg!(value);
        }
        Err(error) => eprintln!("{error}"),
    }
}
