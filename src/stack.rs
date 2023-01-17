use crate::error::UxnError;
use std::fmt;

static STACK_POINTER_INDEX: usize = 255;
static ERROR_INDEX: usize = 254;

pub struct Stack {
    page: [u8; 256],
    keep_ptr: Option<u8>,
}

impl Default for Stack {
    fn default() -> Stack {
        let page = [0; 256];
        let keep_ptr = None;
        Stack { page, keep_ptr }
    }
}

impl Stack {
    pub fn push_u8(&mut self, byte: u8) -> Result<(), UxnError> {
        let index = self.page[STACK_POINTER_INDEX] as usize;
        if index + 1 == STACK_POINTER_INDEX {
            let error = UxnError::OverFlow;
            self.page[ERROR_INDEX] = error.clone().into();
            Err(error)
        } else {
            self.page[STACK_POINTER_INDEX] += 1;
            self.page[index] = byte;
            Ok(())
        }
    }

    pub fn push_u16(&mut self, short: u16) -> Result<(), UxnError> {
        let index = self.page[STACK_POINTER_INDEX] as usize;
        if index + 2 >= STACK_POINTER_INDEX {
            let error = UxnError::OverFlow;
            self.page[ERROR_INDEX] = error.clone().into();
            Err(error)
        } else {
            self.page[STACK_POINTER_INDEX] += 2;
            let src = short.to_be_bytes();
            self.page[index..index + 2].copy_from_slice(&src);
            Ok(())
        }
    }

    #[inline]
    fn current_ptr(&mut self) -> &mut u8 {
        self.keep_ptr
            .as_mut()
            .unwrap_or_else(|| &mut self.page[STACK_POINTER_INDEX])
    }

    pub fn pop_u8(&mut self) -> Result<u8, UxnError> {
        let ptr = self.current_ptr();
        let index = *ptr as usize;
        if index == 0 {
            let error = UxnError::UnderFlow;
            self.page[ERROR_INDEX] = error.clone().into();
            Err(error)
        } else {
            *ptr -= 1;
            let byte = self.page[index - 1];
            Ok(byte)
        }
    }

    pub fn pop_i8(&mut self) -> Result<i8, UxnError> {
        Ok(self.pop_u8()? as i8)
    }

    pub fn pop_u16(&mut self) -> Result<u16, UxnError> {
        let ptr = self.current_ptr();
        let index = *ptr as usize;
        if index < 2 {
            let error = UxnError::UnderFlow;
            self.page[ERROR_INDEX] = error.clone().into();
            Err(error)
        } else {
            *ptr -= 2;
            let short = u16::from_be_bytes([self.page[index - 2], self.page[index - 1]]);
            Ok(short)
        }
    }

    pub fn keep_on(&mut self) {
        self.keep_ptr = Some(self.page[STACK_POINTER_INDEX]);
    }

    pub fn keep_off(&mut self) {
        self.keep_ptr = None;
    }

    pub fn report_divide_by_zero(&mut self) {
        self.page[ERROR_INDEX] = UxnError::DivisionByZero.into();
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self.page[255] as usize;
        let strings = self.page[0..ptr]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<Vec<String>>();
        if strings.is_empty() {
            write!(f, "(empty)")?;
        } else {
            let string = strings.join(",");
            write!(f, "({string})")?;
        }
        Ok(())
    }
}
