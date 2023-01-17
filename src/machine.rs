use crate::{devices::Devices, error::UxnError, memory::Memory, stack::Stack};
use std::{
    fmt,
    io::{stdout, Write},
};

#[derive(Default)]
pub struct Machine {
    pub memory: Memory,
    pub devices: Devices,
    pub wk_stack: Stack,
    pub rt_stack: Stack,
}

pub enum MachineEvent {
    Break,
    Halt(u8),
}

impl Machine {
    pub fn new() -> Machine {
        Default::default()
    }

    fn tic(&mut self) -> Result<Option<MachineEvent>, UxnError> {
        if self.memory.current_operation() == 0x00 {
            return Ok(Some(MachineEvent::Break));
        }
        let byte = self.memory.next_u8();
        let short_mode = (byte & (1 << 5)) != 0x00;
        let return_mode = (byte & (1 << 6)) != 0x00;
        let keep_mode = (byte & (1 << 7)) != 0x00;
        let operation = byte & 0b0001_1111;
        let (src_stack, dst_stack) = if return_mode {
            (&mut self.rt_stack, &mut self.wk_stack)
        } else {
            (&mut self.wk_stack, &mut self.rt_stack)
        };
        if keep_mode {
            src_stack.keep_on();
        } else {
            src_stack.keep_off();
        }
        match operation {
            /*
                Break
                [BRK --] Ends the evalutation of the current vector, the BRK opcode has no modes.
            */
            0x00 if byte == 0x00 => {
                return Ok(Some(MachineEvent::Break));
            }
            /* Stack Operations */
            /*
                Literal
                [LIT -- a] Pushes the next bytes in memory, on the stack, the LIT opcode always has the keep mode active.

                LIT 12          ( 12 )
                LIT2 abcd       ( ab cd )
            */
            0x00 if keep_mode => {
                if short_mode {
                    let short = self.memory.next_u16();
                    src_stack.push_u16(short)?;
                } else {
                    let byte = self.memory.next_u8();
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Increment
                [INC a -- b] Increments the value at the top of the stack, by 1.

                #01 INC         ( 02 )
                #0001 INC2      ( 00 02 )
                #0001 INC2k     ( 00 01 00 02 )
            */
            0x01 => {
                if short_mode {
                    let short = src_stack.pop_u16()?.wrapping_add(1);
                    src_stack.push_u16(short)?;
                } else {
                    let byte = src_stack.pop_u8()?.wrapping_add(1);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Pop
                [POP a --] Removes the value at the top of the stack.

                #1234 POP    ( 12 )
                #1234 POP2   ( )
                #1234 POP2k  ( 12 34 )
            */
            0x02 => {
                if short_mode {
                    src_stack.pop_u16()?;
                } else {
                    src_stack.pop_u8()?;
                }
            }
            /*
                Nip
                [NIP a b -- b] Removes the second value from the stack. This is practical to convert a small short into a byte.

                #1234 NIP          ( 34 )
                #1234 #5678 NIP2   ( 56 78 )
                #1234 #5678 NIP2k  ( 12 34 56 78 56 78 )
            */
            0x03 => {
                if short_mode {
                    let short = src_stack.pop_u16()?;
                    src_stack.pop_u16()?;
                    src_stack.push_u16(short)?;
                } else {
                    let byte = src_stack.pop_u8()?;
                    src_stack.pop_u8()?;
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Swap
                [SWP a b -- b a] Exchanges the first and second values at the top of the stack.

                #1234 SWP          ( 34 12 )
                #1234 SWPk         ( 12 34 34 12 )
                #1234 #5678 SWP2   ( 56 78 12 34 )
                #1234 #5678 SWP2k  ( 12 34 56 78 56 78 12 34 )
            */
            0x04 => {
                if short_mode {
                    let short_a = src_stack.pop_u16()?;
                    let short_b = src_stack.pop_u16()?;
                    src_stack.push_u16(short_a)?;
                    src_stack.push_u16(short_b)?;
                } else {
                    let byte_a = src_stack.pop_u8()?;
                    let byte_b = src_stack.pop_u8()?;
                    src_stack.push_u8(byte_a)?;
                    src_stack.push_u8(byte_b)?;
                }
            }
            /*
                Rotate
                [ROT] Rotates three values at the top of the stack, to the left, wrapping around.

                #1234 #56 ROT            ( 34 56 12 )
                #1234 #56 ROTk           ( 12 34 56 34 56 12 )
                #1234 #5678 #9abc ROT2   ( 56 78 9a bc 12 34 )
                #1234 #5678 #9abc ROT2k  ( 12 34 56 78 9a bc 56 78 9a bc 12 34 )
            */
            0x05 => {
                if short_mode {
                    let short_a = src_stack.pop_u16()?;
                    let short_b = src_stack.pop_u16()?;
                    let short_c = src_stack.pop_u16()?;
                    src_stack.push_u16(short_b)?;
                    src_stack.push_u16(short_a)?;
                    src_stack.push_u16(short_c)?;
                } else {
                    let byte_a = src_stack.pop_u8()?;
                    let byte_b = src_stack.pop_u8()?;
                    let byte_c = src_stack.pop_u8()?;
                    src_stack.push_u8(byte_b)?;
                    src_stack.push_u8(byte_a)?;
                    src_stack.push_u8(byte_c)?;
                }
            }
            /*
                Duplicate
                [DUP a -- a a] Duplicates the value at the top of the stack.

                #1234 DUP   ( 12 34 34 )
                #12 DUPk    ( 12 12 12 )
                #1234 DUP2  ( 12 34 12 34 )
            */
            0x06 => {
                if short_mode {
                    let short = src_stack.pop_u16()?;
                    src_stack.push_u16(short)?;
                    src_stack.push_u16(short)?;
                } else {
                    let byte = src_stack.pop_u8()?;
                    src_stack.push_u8(byte)?;
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Over
                [OVR a b -- a b a] Duplicates the second value at the top of the stack.

                #1234 OVR          ( 12 34 12 )
                #1234 OVRk         ( 12 34 12 34 12 )
                #1234 #5678 OVR2   ( 12 34 56 78 12 34 )
                #1234 #5678 OVR2k  ( 12 34 56 78 12 34 56 78 12 34 )
            */
            0x07 => {
                if short_mode {
                    let short_a = src_stack.pop_u16()?;
                    let short_b = src_stack.pop_u16()?;
                    src_stack.push_u16(short_b)?;
                    src_stack.push_u16(short_a)?;
                    src_stack.push_u16(short_b)?;
                } else {
                    let byte_a = src_stack.pop_u8()?;
                    let byte_b = src_stack.pop_u8()?;
                    src_stack.push_u8(byte_b)?;
                    src_stack.push_u8(byte_a)?;
                    src_stack.push_u8(byte_b)?;
                }
            }
            /* Logic Operations */
            /*
                Equal
                [EQU a b -- bool8] Pushes 01 to the stack if the two values at the top of the stack are equal, 00 otherwise.

                #1212 EQU          ( 01 )
                #1234 EQUk         ( 12 34 00 )
                #abcd #ef01 EQU2   ( 00 )
                #abcd #abcd EQU2k  ( ab cd ab cd 01 )
            */
            0x08 => {
                if short_mode {
                    let short_a = src_stack.pop_u16()?;
                    let short_b = src_stack.pop_u16()?;
                    let byte = (short_a == short_b).into();
                    src_stack.push_u8(byte)?;
                } else {
                    let byte_a = src_stack.pop_u8()?;
                    let byte_b = src_stack.pop_u8()?;
                    let byte = (byte_a == byte_b).into();
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Not Equal
                [NEQ a b -- bool8] Pushes 01 to the stack if the two values at the top of the stack are not equal, 00 otherwise.

                #1212 NEQ          ( 00 )
                #1234 NEQk         ( 12 34 01 )
                #abcd #ef01 NEQ2   ( 01 )
                #abcd #abcd NEQ2k  ( ab cd ab cd 00 )
            */
            0x09 => {
                if short_mode {
                    let short_a = src_stack.pop_u16()?;
                    let short_b = src_stack.pop_u16()?;
                    let byte = (short_b != short_a).into();
                    src_stack.push_u8(byte)?;
                } else {
                    let byte_a = src_stack.pop_u8()?;
                    let byte_b = src_stack.pop_u8()?;
                    let byte = (byte_b != byte_a).into();
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Greater Than
                [GTH a b -- bool8] Pushes 01 to the stack if the second value at the top of the stack is greater than the value at the top of the stack, 00 otherwise.

                #1234 GTH          ( 00 )
                #3412 GTHk         ( 34 12 01 )
                #3456 #1234 GTH2   ( 01 )
                #1234 #3456 GTH2k  ( 12 34 34 56 00 )
            */
            0x0a => {
                if short_mode {
                    let short_a = src_stack.pop_u16()?;
                    let short_b = src_stack.pop_u16()?;
                    let byte = (short_b > short_a).into();
                    src_stack.push_u8(byte)?;
                } else {
                    let byte_a = src_stack.pop_u8()?;
                    let byte_b = src_stack.pop_u8()?;
                    let byte = (byte_b > byte_a).into();
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Lesser Than
                [LTH a b -- bool8] Pushes 01 to the stack if the second value at the top of the stack is lesser than the value at the top of the stack, 00 otherwise.

                #0101 LTH          ( 00 )
                #0100 LTHk         ( 01 00 00 )
                #0001 #0000 LTH2   ( 00 )
                #0001 #0000 LTH2k  ( 00 01 00 00 00 )
            */
            0x0b => {
                if short_mode {
                    let short_a = src_stack.pop_u16()?;
                    let short_b = src_stack.pop_u16()?;
                    let byte = (short_b < short_a).into();
                    src_stack.push_u8(byte)?;
                } else {
                    let byte_a = src_stack.pop_u8()?;
                    let byte_b = src_stack.pop_u8()?;
                    let byte = (byte_b < byte_a).into();
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Jump
                [JMP addr --] Moves the program counter by a relative distance equal to the signed byte on the top of the stack, or to an absolute address in short mode.

                ,&skip-rel JMP BRK &skip-rel #01  ( 01 )
            */
            0x0c => {
                if short_mode {
                    let addr = src_stack.pop_u16()?;
                    self.memory.jump(addr);
                } else {
                    let delta = src_stack.pop_i8()?;
                    self.memory.jump_rel(delta);
                }
            }
            /*
                Jump Conditional
                [JCN cond8 addr --] If the byte preceeding the address is not 00, moves the program counter by a signed value equal to the byte on the top of the stack, or an absolute address in short mode.

                #abcd #01 ,&pass JCN SWP &pass POP  ( ab )
                #abcd #00 ,&fail JCN SWP &fail POP  ( cd )
            */
            0x0d => {
                if short_mode {
                    let addr = src_stack.pop_u16()?;
                    let value = src_stack.pop_u8()?;
                    if value != 0 {
                        self.memory.jump(addr);
                    }
                } else {
                    let delta = src_stack.pop_i8()?;
                    let value = src_stack.pop_u8()?;
                    if value != 0 {
                        self.memory.jump_rel(delta);
                    }
                }
            }
            /*
                Jump Stash Return
                [JSR addr --] Pushes the value of the program counter to the return-stack and moves the program counter by a signed value equal to the byte on the top of the stack, or an absolute address in short mode.

                ,&get JSR #01 BRK &get #02 JMP2r  ( 02 01 )
            */
            0x0e => {
                if short_mode {
                    let addr = src_stack.pop_u16()?;
                    let program_counter = self.memory.pc_value();
                    dst_stack.push_u16(program_counter)?;
                    self.memory.jump(addr);
                } else {
                    let delta = src_stack.pop_i8()?;
                    let program_counter = self.memory.pc_value();
                    dst_stack.push_u16(program_counter)?;
                    self.memory.jump_rel(delta);
                }
            }
            /*
                Stash
                [STH a --] Moves the value at the top of the stack, to the return stack.

                #01 STH LITr 02 ADDr STHr  ( 03 )
            */
            0x0f => {
                if short_mode {
                    let short = src_stack.pop_u16()?;
                    dst_stack.push_u16(short)?;
                } else {
                    let byte = src_stack.pop_u8()?;
                    dst_stack.push_u8(byte)?;
                }
            }
            /* Memory Operations */
            /*
                Load Zero-Page
                [LDZ addr8 -- value] Pushes the value at an address within the first 256 bytes of memory, to the top of the stack.

                |00 @cell $2 |0100 .cell LDZ ( 00 )
            */
            0x10 => {
                let addr = src_stack.pop_u8()? as u16;
                if short_mode {
                    let short = self.memory.peek_u16(addr);
                    src_stack.push_u16(short)?;
                } else {
                    let byte = self.memory.peek_u8(addr);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Store Zero-Page
                [SDZ val addr8 --] Writes a value to an address within the first 256 bytes of memory.

                |00 @cell $2 |0100 #abcd .cell STZ2  { ab cd }
            */
            0x11 => {
                let addr = src_stack.pop_u8()? as u16;
                if short_mode {
                    let short = src_stack.pop_u16()?;
                    self.memory.poke_u16(addr, short);
                } else {
                    let byte = src_stack.pop_u8()?;
                    self.memory.poke_u8(addr, byte);
                }
            }
            /*
                Load Relative
                [LDR addr8 -- value] Pushes the value at a relative address, to the top of the stack. The possible relative range is -128 to +127 bytes.

                ,cell LDR2 BRK @cell abcd  ( ab cd )
            */
            0x12 => {
                let delta = src_stack.pop_i8()?;
                if short_mode {
                    let short = self.memory.peek_u16_rel(delta);
                    src_stack.push_u16(short)?;
                } else {
                    let byte = self.memory.peek_u8_rel(delta);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Store Relative
                [STR val addr8 --] Writes a value to a relative address. The possible relative range is -128 to +127 bytes.

                #1234 ,cell STR2 BRK @cell $2  ( )
            */
            0x13 => {
                let delta = src_stack.pop_i8()?;
                if short_mode {
                    let short = src_stack.pop_u16()?;
                    self.memory.poke_u16_rel(delta, short);
                } else {
                    let byte = src_stack.pop_u8()?;
                    self.memory.poke_u8_rel(delta, byte);
                }
            }
            /*
                Load Absolute
                [LDA addr16 -- value] Pushes the value at a absolute address, to the top of the stack.

                ;cell LDA BRK @cell abcd ( ab )
            */
            0x14 => {
                let addr = src_stack.pop_u16()?;
                if short_mode {
                    let short = self.memory.peek_u16(addr);
                    src_stack.push_u16(short)?;
                } else {
                    let byte = self.memory.peek_u8(addr);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Store Absolute
                [STA val addr16 --] Writes a value to a absolute address.

                #abcd ;cell STA BRK @cell $1 ( ab )
            */
            0x15 => {
                let addr = src_stack.pop_u16()?;
                if short_mode {
                    let short = src_stack.pop_u16()?;
                    self.memory.poke_u16(addr, short);
                } else {
                    let byte = src_stack.pop_u8()?;
                    self.memory.poke_u8(addr, byte);
                }
            }
            /*
                Device Input
                [DEI device8 -- value] Pushes a value from the device page, to the top of the stack. The target device might capture the reading to trigger an I/O event.
            */
            0x16 => {
                let addr = src_stack.pop_u8()?;
                if short_mode {
                    let short = self.devices.device_input_u16(addr);
                    src_stack.push_u16(short)?;
                } else {
                    let byte = self.devices.device_input_u8(addr);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Device Output
                [DEO val device8 --] Writes a value to the device page. The target device might capture the writing to trigger an I/O event.
            */
            0x17 => {
                let addr = src_stack.pop_u8()?;
                if short_mode {
                    let short = src_stack.pop_u16()?;
                    if let Some(state) = self.devices.device_output_u16(
                        addr,
                        short,
                        &self.wk_stack,
                        &self.rt_stack,
                        &mut self.memory,
                    ) {
                        return Ok(Some(MachineEvent::Halt(state)));
                    }
                } else {
                    let byte = src_stack.pop_u8()?;
                    if let Some(state) = self.devices.device_output_u8(
                        addr,
                        byte,
                        &self.wk_stack,
                        &self.rt_stack,
                        &mut self.memory,
                    ) {
                        return Ok(Some(MachineEvent::Halt(state)));
                    }
                }
            }
            /* Arithmetic Operations */
            /*
                Add
                [ADD a b -- c] Pushes the sum of the two values at the top of the stack.

                #1a #2e ADD       ( 48 )
                #02 #5d ADDk      ( 01 5d 5f )
                #0001 #0002 ADD2  ( 00 03 )
            */
            0x18 => {
                if short_mode {
                    let a = src_stack.pop_u16()?;
                    let b = src_stack.pop_u16()?;
                    let short = b.wrapping_add(a);
                    src_stack.push_u16(short)?;
                } else {
                    let a = src_stack.pop_u8()?;
                    let b = src_stack.pop_u8()?;
                    let byte = b.wrapping_add(a);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Subtract
                [SUB a b -- c] Pushes the difference of the first value minus the second, to the top of the stack.
            */
            0x19 => {
                if short_mode {
                    let a = src_stack.pop_u16()?;
                    let b = src_stack.pop_u16()?;
                    let short = b.wrapping_sub(a);
                    src_stack.push_u16(short)?;
                } else {
                    let a = src_stack.pop_u8()?;
                    let b = src_stack.pop_u8()?;
                    let byte = b.wrapping_sub(a);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Multiply
                [MUL a b -- c] Pushes the product of the first and second values at the top of the stack.
            */
            0x1a => {
                if short_mode {
                    let a = src_stack.pop_u16()?;
                    let b = src_stack.pop_u16()?;
                    let short = b.wrapping_mul(a);
                    src_stack.push_u16(short)?;
                } else {
                    let a = src_stack.pop_u8()?;
                    let b = src_stack.pop_u8()?;
                    let byte = b.wrapping_mul(a);
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Divide
                [DIV a b -- c] Pushes the quotient of the first value over the second, to the top of the stack.
            */
            0x1b => {
                if short_mode {
                    let a = src_stack.pop_u16()?;
                    let b = src_stack.pop_u16()?;
                    match b.checked_div(a) {
                        Some(short) => src_stack.push_u16(short)?,
                        None => {
                            src_stack.report_divide_by_zero();
                            return Err(UxnError::DivisionByZero);
                        }
                    }
                } else {
                    let a = src_stack.pop_u8()?;
                    let b = src_stack.pop_u8()?;
                    match b.checked_div(a) {
                        Some(byte) => src_stack.push_u8(byte)?,
                        None => {
                            src_stack.report_divide_by_zero();
                            return Err(UxnError::DivisionByZero);
                        }
                    }
                }
            }
            /*
                And
                [AND a b -- c] Pushes the result of the bitwise operation AND, to the top of the stack.
            */
            0x1c => {
                if short_mode {
                    let a = src_stack.pop_u16()?;
                    let b = src_stack.pop_u16()?;
                    let short = b & a;
                    src_stack.push_u16(short)?;
                } else {
                    let a = src_stack.pop_u8()?;
                    let b = src_stack.pop_u8()?;
                    let byte = b & a;
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Or
                [ORA a b -- c] Pushes the result of the bitwise operation OR, to the top of the stack.
            */
            0x1d => {
                if short_mode {
                    let a = src_stack.pop_u16()?;
                    let b = src_stack.pop_u16()?;
                    let short = b | a;
                    src_stack.push_u16(short)?;
                } else {
                    let a = src_stack.pop_u8()?;
                    let b = src_stack.pop_u8()?;
                    let byte = b | a;
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Exclusive Or
                [EOR a b -- c] Pushes the result of the bitwise operation XOR, to the top of the stack.
            */
            0x1e => {
                if short_mode {
                    let a = src_stack.pop_u16()?;
                    let b = src_stack.pop_u16()?;
                    let short = b ^ a;
                    src_stack.push_u16(short)?;
                } else {
                    let a = src_stack.pop_u8()?;
                    let b = src_stack.pop_u8()?;
                    let byte = b ^ a;
                    src_stack.push_u8(byte)?;
                }
            }
            /*
                Shift
                [SFT a shift8 -- c] Shifts the bits of the second value of the stack to the left or right, depending on the control value at the top of the stack. The high nibble of the control value indicates how many bits to shift left, and the low nibble how many bits to shift right. The rightward shift is done first.

                #34 #10 SFT        ( 68 )
                #34 #01 SFT        ( 1a )
                #34 #33 SFTk       ( 34 33 30 )
                #1248 #34 SFTk2    ( 12 48 34 09 20 )
            */
            0x1f => {
                let byte = src_stack.pop_u8()?;
                let low_nibble = (byte & 0x0f) as u32; // 7
                let high_nibble = (byte >> 4) as u32; // 7
                if short_mode {
                    let mut short = src_stack.pop_u16()?;
                    short = short.checked_shr(low_nibble).unwrap_or(0);
                    short = short.checked_shl(high_nibble).unwrap_or(0);
                    src_stack.push_u16(short)?;
                } else {
                    let mut byte = src_stack.pop_u8()?;
                    byte = byte.checked_shr(low_nibble).unwrap_or(0);
                    byte = byte.checked_shl(high_nibble).unwrap_or(0);
                    src_stack.push_u8(byte)?;
                }
            }
            /* Unused Operations */
            0x00 if byte == 0x20 => {}
            0x00 if byte == 0x40 => {}
            0x00 if byte == 0x60 => {}
            /* Will never happen */
            _ => unreachable!(
                "byte: {byte:02x}, operation {operation:02x}, program counter: {:02x}",
                self.memory.pc_value()
            ),
        }
        Ok(None)
    }

    pub fn run(&mut self) -> Result<MachineEvent, UxnError> {
        loop {
            match self.tic()? {
                None => continue,
                Some(event) => {
                    stdout().flush().ok();
                    return Ok(event);
                }
            }
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Working stack: {}", self.wk_stack)?;
        writeln!(f, "Return stack: {}", self.rt_stack)?;
        write!(f, "{}", self.memory)?;
        Ok(())
    }
}
