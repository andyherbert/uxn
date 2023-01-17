use crate::stack::Stack;

pub fn trigger_event(port: u8, ports: &[u8], wk_stack: &Stack, rt_stack: &Stack) -> Option<u8> {
    match port {
        // Red
        0x08 => {}
        // Green
        0x0a => {}
        // Blue
        0x0c => {}
        // Debug
        0x0e => {
            let byte = ports[port as usize];
            if byte != 0 {
                println!("Working stack: {wk_stack}");
                println!("Return stack: {rt_stack}");
            }
        }
        // State
        0x0f => {
            let byte = ports[port as usize];
            if byte != 0 {
                return Some(byte);
            }
        }
        _ => {}
    }
    None
}
