mod file_interface;
mod file_system;
mod virtual_file_system;
use super::{peek_u16, poke_u16};
use crate::memory::Memory;
pub use file_interface::FileInterface;
pub use file_system::PhysicalFileSystem;
use std::collections::VecDeque;
pub use virtual_file_system::VirtualFileSystem;

fn file_entry(name: impl AsRef<str>, len: usize) -> Vec<u8> {
    let string = if len > 0x9999 {
        format!("???? {}\n", name.as_ref())
    } else {
        format!("{len:04x} {}\n", name.as_ref())
    };
    string.into_bytes()
}

#[derive(Debug)]
pub enum ReadType {
    File(Vec<u8>),
    Stats(VecDeque<Vec<u8>>),
}

#[derive(Debug)]
enum State {
    Read(ReadType),
    Write(String),
}

fn path_from_bytes(bytes: &[u8], memory: &Memory) -> String {
    let ptr = peek_u16(bytes, 0x08);
    memory.get_string(ptr)
}

#[derive(Default)]
pub struct FileDevice {
    interface: FileInterface,
    state: Option<State>,
}

impl FileDevice {
    pub fn with_interface(interface: FileInterface) -> FileDevice {
        FileDevice {
            interface,
            ..Default::default()
        }
    }
}

impl FileDevice {
    pub fn trigger_event(&mut self, port: u8, ports: &mut [u8], memory: &mut Memory) {
        match port {
            // Stat
            0x04 => {
                let path = path_from_bytes(ports, memory);
                match self.interface.stat(&path) {
                    Some(bytes) => {
                        let length = peek_u16(ports, 0x0a);
                        let location = peek_u16(ports, 0x0c);
                        if (length as usize) <= bytes.len() {
                            memory.poke_u8s(location, &bytes);
                        } else {
                            poke_u16(ports, 0x02, 0x0000);
                        }
                    }
                    None => poke_u16(ports, 0x02, 0x0000),
                }
            }
            // Delete
            0x06 => {
                let path = path_from_bytes(ports, memory);
                self.interface.delete(&path);
            }
            // Change Name
            0x08 if self.state.is_some() => {
                self.state = None;
            }
            // Read
            0x0c => match self.state.take() {
                Some(State::Read(ReadType::File(bytes))) => {
                    let length = peek_u16(ports, 0x0a);
                    let mid = bytes.len().min(length as usize);
                    let (bytes, remainder) = bytes.split_at(mid);
                    poke_u16(ports, 0x02, bytes.len() as u16);
                    let location = peek_u16(ports, 0x0c);
                    memory.poke_u8s(location, bytes);
                    self.state = Some(State::Read(ReadType::File(remainder.to_vec())));
                }
                Some(State::Read(ReadType::Stats(mut bytes))) => {
                    let mut location = peek_u16(ports, 0x0c);
                    let mut length = peek_u16(ports, 0x0a);
                    let mut count = 0;
                    loop {
                        match bytes.front() {
                            Some(front) if length as usize >= front.len() => {
                                memory.poke_u8s(location, front);
                                let len = front.len() as u16;
                                location = location.wrapping_add(len);
                                length -= len;
                                count += len;
                                bytes.pop_front();
                            }
                            _ => {
                                poke_u16(ports, 0x02, count);
                                self.state = Some(State::Read(ReadType::Stats(bytes)));
                                break;
                            }
                        }
                    }
                }
                _ => {
                    let path = path_from_bytes(ports, memory);
                    match self.interface.read(&path) {
                        Ok(ReadType::File(bytes)) => {
                            self.state = Some(State::Read(ReadType::File(bytes)));
                            self.trigger_event(port, ports, memory);
                        }
                        Ok(ReadType::Stats(bytes)) => {
                            self.state = Some(State::Read(ReadType::Stats(bytes)));
                            self.trigger_event(port, ports, memory);
                        }
                        Err(_) => poke_u16(ports, 0x02, 0x0000),
                    }
                }
            },
            // Write
            0x0e => match self.state.take() {
                Some(State::Write(path)) => {
                    let length = peek_u16(ports, 0x0a);
                    let location = peek_u16(ports, 0x0e);
                    let buf = memory.peek_u8s(location, length);
                    match self.interface.write(&path, &buf) {
                        Ok(length) => poke_u16(ports, 0x02, length as u16),
                        Err(_) => poke_u16(ports, 0x02, 0x0000),
                    }
                    self.state = Some(State::Write(path));
                }
                _ => {
                    let path = path_from_bytes(ports, memory);
                    if path.is_empty() {
                        poke_u16(ports, 0x02, 0x0000);
                    } else {
                        let append = ports[0x07] == 1;
                        if !append {
                            self.interface.create(&path);
                        }
                        self.state = Some(State::Write(path));
                        self.trigger_event(port, ports, memory)
                    }
                }
            },
            _ => {}
        }
    }
}
