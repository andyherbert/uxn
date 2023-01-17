mod console;
mod date_time;
mod file_device;
mod system;
use crate::{memory::Memory, stack::Stack};
use date_time::{DeviceDateTime, DeviceSystemTime};
use file_device::{FileDevice, FileInterface, PhysicalFileSystem, VirtualFileSystem};
use std::path::Path;

fn peek_u16(ports: &[u8], addr: u8) -> u16 {
    let low = ports[addr as usize];
    let high = ports[addr.wrapping_add(1) as usize];
    u16::from_be_bytes([low, high])
}

fn poke_u16(ports: &mut [u8], addr: u8, short: u16) {
    let short_bytes = short.to_be_bytes();
    ports[addr as usize] = short_bytes[0];
    let addr_2 = addr.wrapping_add(1);
    ports[addr_2 as usize] = short_bytes[1];
}

pub struct Devices {
    system_time: DeviceSystemTime,
    ports: [u8; 256],
    file_0: FileDevice,
    file_1: FileDevice,
}

impl Default for Devices {
    fn default() -> Devices {
        let date_time_type = DeviceSystemTime::Local;
        let ports = [0; 256];
        let file_0 = FileDevice::default();
        let file_1 = FileDevice::default();
        Devices {
            system_time: date_time_type,
            ports,
            file_0,
            file_1,
        }
    }
}

impl Devices {
    pub fn device_input_u8(&self, port: u8) -> u8 {
        match port {
            0xc0..=0xcf => date_time::device_input_u8(port - 0xc0, &self.system_time),
            _ => self.ports[port as usize],
        }
    }

    pub fn device_input_u16(&mut self, port: u8) -> u16 {
        let low = self.device_input_u8(port);
        let high = self.device_input_u8(port.wrapping_add(1));
        u16::from_be_bytes([low, high])
    }

    pub fn console_vector(&self) -> Option<u16> {
        console::vector(&self.ports)
    }

    pub fn console_event_value(&mut self, byte: u8) {
        self.ports[0x12] = byte;
    }

    fn use_interface(&mut self, interface: FileInterface) {
        self.file_0 = FileDevice::with_interface(interface.clone());
        self.file_1 = FileDevice::with_interface(interface);
    }

    pub fn use_phycial_file_system(&mut self, path: impl AsRef<Path>, safety: bool) {
        self.use_interface(FileInterface::FileSystem(PhysicalFileSystem::new(
            path, safety,
        )));
    }

    pub fn use_virtual_file_system(&mut self) {
        self.use_interface(FileInterface::VirtualFileSystem(
            VirtualFileSystem::default(),
        ));
    }

    pub fn use_local_time(&mut self) {
        self.system_time = DeviceSystemTime::Local;
    }

    pub fn use_utc(&mut self) {
        self.system_time = DeviceSystemTime::Utc;
    }

    pub fn set_time(&mut self, date_time: DeviceDateTime) {
        self.system_time = DeviceSystemTime::new(date_time);
    }

    pub fn use_static_time(&mut self, date_time: DeviceDateTime) {
        self.system_time = DeviceSystemTime::Static(date_time);
    }

    fn trigger_event(
        &mut self,
        port: u8,
        wk_stack: &Stack,
        rt_stack: &Stack,
        memory: &mut Memory,
    ) -> Option<u8> {
        match port {
            // System
            0x00..=0x0f => {
                return system::trigger_event(port, &self.ports[0x00..=0x0f], wk_stack, rt_stack)
            }
            // Console
            0x10..=0x1f => {
                console::trigger_event(port - 0x10, &self.ports[0x10..=0x1f]);
            }
            // Screen
            0x20..=0x2f => {}
            // Audio
            0x30..=0x6f => {}
            // Midi
            0x70..=0x7f => {}
            // Controller
            0x80..=0x8f => {}
            // Mouse
            0x90..=0x9f => {}
            // File 0
            0xa0..=0xaf => {
                self.file_0
                    .trigger_event(port - 0xa0, &mut self.ports[0xa0..=0xaf], memory);
            }
            // File 1
            0xb0..=0xbf => {
                self.file_1
                    .trigger_event(port - 0xb0, &mut self.ports[0xb0..=0xbf], memory);
            }
            // Datetime
            0xc0..=0xcf => {}
            // Reserved
            0xd0..=0xef => {}
            // Emulator
            0xf0..=0xff => {}
        }
        None
    }

    pub fn device_output_u16(
        &mut self,
        port: u8,
        short: u16,
        wk_stack: &Stack,
        rt_stack: &Stack,
        memory: &mut Memory,
    ) -> Option<u8> {
        let short_bytes = short.to_be_bytes();
        self.ports[port as usize] = short_bytes[0];
        let addr_2 = port.wrapping_add(1);
        self.ports[addr_2 as usize] = short_bytes[1];
        if let Some(state) = self.trigger_event(port, wk_stack, rt_stack, memory) {
            Some(state)
        } else {
            self.trigger_event(addr_2, wk_stack, rt_stack, memory)
        }
    }

    pub fn device_output_u8(
        &mut self,
        port: u8,
        byte: u8,
        wk_stack: &Stack,
        rt_stack: &Stack,
        memory: &mut Memory,
    ) -> Option<u8> {
        self.ports[port as usize] = byte;
        self.trigger_event(port, wk_stack, rt_stack, memory)
    }
}
