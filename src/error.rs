use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub enum UxnError {
    UnderFlow,
    OverFlow,
    DivisionByZero,
    Unknown,
}

impl fmt::Display for UxnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UxnError::*;
        match self {
            UnderFlow => write!(f, "Underflow"),
            OverFlow => write!(f, "Overflow"),
            DivisionByZero => write!(f, "Division by zero"),
            Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Error for UxnError {}

impl From<UxnError> for u8 {
    fn from(error: UxnError) -> Self {
        match error {
            UxnError::UnderFlow => 0x01,
            UxnError::OverFlow => 0x02,
            UxnError::DivisionByZero => 0x03,
            UxnError::Unknown => 0xff,
        }
    }
}

impl From<u8> for UxnError {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => UxnError::UnderFlow,
            0x02 => UxnError::OverFlow,
            0x03 => UxnError::DivisionByZero,
            _ => UxnError::Unknown,
        }
    }
}
