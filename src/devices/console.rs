use super::peek_u16;

pub fn vector(ports: &[u8]) -> Option<u16> {
    match peek_u16(ports, 0x10) {
        0 => None,
        addr => Some(addr),
    }
}

pub fn trigger_event(port: u8, ports: &[u8]) {
    match port {
        // Write
        0x08 => {
            let ch = ports[port as usize] as char;
            print!("{ch}");
        }
        // Error
        0x09 => {
            let ch = ports[port as usize] as char;
            eprint!("{ch}");
        }
        _ => {}
    }
}
