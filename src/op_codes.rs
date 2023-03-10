use std::fmt;

#[allow(clippy::upper_case_acronyms)]
pub enum OpCode {
    BRK,
    INC,
    POP,
    NIP,
    SWP,
    ROT,
    DUP,
    OVR,
    EQU,
    NEQ,
    GTH,
    LTH,
    JMP,
    JCN,
    JSR,
    STH,
    LDZ,
    STZ,
    LDR,
    STR,
    LDA,
    STA,
    DEI,
    DEO,
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    ORA,
    EOR,
    SFT,
    NOP1,
    INC2,
    POP2,
    NIP2,
    SWP2,
    ROT2,
    DUP2,
    OVR2,
    EQU2,
    NEQ2,
    GTH2,
    LTH2,
    JMP2,
    JCN2,
    JSR2,
    STH2,
    LDZ2,
    STZ2,
    LDR2,
    STR2,
    LDA2,
    STA2,
    DEI2,
    DEO2,
    ADD2,
    SUB2,
    MUL2,
    DIV2,
    AND2,
    ORA2,
    EOR2,
    SFT2,
    NOP2,
    INCr,
    POPr,
    NIPr,
    SWPr,
    ROTr,
    DUPr,
    OVRr,
    EQUr,
    NEQr,
    GTHr,
    LTHr,
    JMPr,
    JCNr,
    JSRr,
    STHr,
    LDZr,
    STZr,
    LDRr,
    STRr,
    LDAr,
    STAr,
    DEIr,
    DEOr,
    ADDr,
    SUBr,
    MULr,
    DIVr,
    ANDr,
    ORAr,
    EORr,
    SFTr,
    NOP3,
    INC2r,
    POP2r,
    NIP2r,
    SWP2r,
    ROT2r,
    DUP2r,
    OVR2r,
    EQU2r,
    NEQ2r,
    GTH2r,
    LTH2r,
    JMP2r,
    JCN2r,
    JSR2r,
    STH2r,
    LDZ2r,
    STZ2r,
    LDR2r,
    STR2r,
    LDA2r,
    STA2r,
    DEI2r,
    DEO2r,
    ADD2r,
    SUB2r,
    MUL2r,
    DIV2r,
    AND2r,
    ORA2r,
    EOR2r,
    SFT2r,
    LIT,
    INCk,
    POPk,
    NIPk,
    SWPk,
    ROTk,
    DUPk,
    OVRk,
    EQUk,
    NEQk,
    GTHk,
    LTHk,
    JMPk,
    JCNk,
    JSRk,
    STHk,
    LDZk,
    STZk,
    LDRk,
    STRk,
    LDAk,
    STAk,
    DEIk,
    DEOk,
    ADDk,
    SUBk,
    MULk,
    DIVk,
    ANDk,
    ORAk,
    EORk,
    SFTk,
    LIT2,
    INC2k,
    POP2k,
    NIP2k,
    SWP2k,
    ROT2k,
    DUP2k,
    OVR2k,
    EQU2k,
    NEQ2k,
    GTH2k,
    LTH2k,
    JMP2k,
    JCN2k,
    JSR2k,
    STH2k,
    LDZ2k,
    STZ2k,
    LDR2k,
    STR2k,
    LDA2k,
    STA2k,
    DEI2k,
    DEO2k,
    ADD2k,
    SUB2k,
    MUL2k,
    DIV2k,
    AND2k,
    ORA2k,
    EOR2k,
    SFT2k,
    LITr,
    INCkr,
    POPkr,
    NIPkr,
    SWPkr,
    ROTkr,
    DUPkr,
    OVRkr,
    EQUkr,
    NEQkr,
    GTHkr,
    LTHkr,
    JMPkr,
    JCNkr,
    JSRkr,
    STHkr,
    LDZkr,
    STZkr,
    LDRkr,
    STRkr,
    LDAkr,
    STAkr,
    DEIkr,
    DEOkr,
    ADDkr,
    SUBkr,
    MULkr,
    DIVkr,
    ANDkr,
    ORAkr,
    EORkr,
    SFTkr,
    LIT2r,
    INC2kr,
    POP2kr,
    NIP2kr,
    SWP2kr,
    ROT2kr,
    DUP2kr,
    OVR2kr,
    EQU2kr,
    NEQ2kr,
    GTH2kr,
    LTH2kr,
    JMP2kr,
    JCN2kr,
    JSR2kr,
    STH2kr,
    LDZ2kr,
    STZ2kr,
    LDR2kr,
    STR2kr,
    LDA2kr,
    STA2kr,
    DEI2kr,
    DEO2kr,
    ADD2kr,
    SUB2kr,
    MUL2kr,
    DIV2kr,
    AND2kr,
    ORA2kr,
    EOR2kr,
    SFT2kr,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        match byte {
            0x00 => OpCode::BRK,
            0x01 => OpCode::INC,
            0x02 => OpCode::POP,
            0x03 => OpCode::NIP,
            0x04 => OpCode::SWP,
            0x05 => OpCode::ROT,
            0x06 => OpCode::DUP,
            0x07 => OpCode::OVR,
            0x08 => OpCode::EQU,
            0x09 => OpCode::NEQ,
            0x0a => OpCode::GTH,
            0x0b => OpCode::LTH,
            0x0c => OpCode::JMP,
            0x0d => OpCode::JCN,
            0x0e => OpCode::JSR,
            0x0f => OpCode::STH,
            0x10 => OpCode::LDZ,
            0x11 => OpCode::STZ,
            0x12 => OpCode::LDR,
            0x13 => OpCode::STR,
            0x14 => OpCode::LDA,
            0x15 => OpCode::STA,
            0x16 => OpCode::DEI,
            0x17 => OpCode::DEO,
            0x18 => OpCode::ADD,
            0x19 => OpCode::SUB,
            0x1a => OpCode::MUL,
            0x1b => OpCode::DIV,
            0x1c => OpCode::AND,
            0x1d => OpCode::ORA,
            0x1e => OpCode::EOR,
            0x1f => OpCode::SFT,
            0x20 => OpCode::NOP1,
            0x21 => OpCode::INC2,
            0x22 => OpCode::POP2,
            0x23 => OpCode::NIP2,
            0x24 => OpCode::SWP2,
            0x25 => OpCode::ROT2,
            0x26 => OpCode::DUP2,
            0x27 => OpCode::OVR2,
            0x28 => OpCode::EQU2,
            0x29 => OpCode::NEQ2,
            0x2a => OpCode::GTH2,
            0x2b => OpCode::LTH2,
            0x2c => OpCode::JMP2,
            0x2d => OpCode::JCN2,
            0x2e => OpCode::JSR2,
            0x2f => OpCode::STH2,
            0x30 => OpCode::LDZ2,
            0x31 => OpCode::STZ2,
            0x32 => OpCode::LDR2,
            0x33 => OpCode::STR2,
            0x34 => OpCode::LDA2,
            0x35 => OpCode::STA2,
            0x36 => OpCode::DEI2,
            0x37 => OpCode::DEO2,
            0x38 => OpCode::ADD2,
            0x39 => OpCode::SUB2,
            0x3a => OpCode::MUL2,
            0x3b => OpCode::DIV2,
            0x3c => OpCode::AND2,
            0x3d => OpCode::ORA2,
            0x3e => OpCode::EOR2,
            0x3f => OpCode::SFT2,
            0x40 => OpCode::NOP2,
            0x41 => OpCode::INCr,
            0x42 => OpCode::POPr,
            0x43 => OpCode::NIPr,
            0x44 => OpCode::SWPr,
            0x45 => OpCode::ROTr,
            0x46 => OpCode::DUPr,
            0x47 => OpCode::OVRr,
            0x48 => OpCode::EQUr,
            0x49 => OpCode::NEQr,
            0x4a => OpCode::GTHr,
            0x4b => OpCode::LTHr,
            0x4c => OpCode::JMPr,
            0x4d => OpCode::JCNr,
            0x4e => OpCode::JSRr,
            0x4f => OpCode::STHr,
            0x50 => OpCode::LDZr,
            0x51 => OpCode::STZr,
            0x52 => OpCode::LDRr,
            0x53 => OpCode::STRr,
            0x54 => OpCode::LDAr,
            0x55 => OpCode::STAr,
            0x56 => OpCode::DEIr,
            0x57 => OpCode::DEOr,
            0x58 => OpCode::ADDr,
            0x59 => OpCode::SUBr,
            0x5a => OpCode::MULr,
            0x5b => OpCode::DIVr,
            0x5c => OpCode::ANDr,
            0x5d => OpCode::ORAr,
            0x5e => OpCode::EORr,
            0x5f => OpCode::SFTr,
            0x60 => OpCode::NOP3,
            0x61 => OpCode::INC2r,
            0x62 => OpCode::POP2r,
            0x63 => OpCode::NIP2r,
            0x64 => OpCode::SWP2r,
            0x65 => OpCode::ROT2r,
            0x66 => OpCode::DUP2r,
            0x67 => OpCode::OVR2r,
            0x68 => OpCode::EQU2r,
            0x69 => OpCode::NEQ2r,
            0x6a => OpCode::GTH2r,
            0x6b => OpCode::LTH2r,
            0x6c => OpCode::JMP2r,
            0x6d => OpCode::JCN2r,
            0x6e => OpCode::JSR2r,
            0x6f => OpCode::STH2r,
            0x70 => OpCode::LDZ2r,
            0x71 => OpCode::STZ2r,
            0x72 => OpCode::LDR2r,
            0x73 => OpCode::STR2r,
            0x74 => OpCode::LDA2r,
            0x75 => OpCode::STA2r,
            0x76 => OpCode::DEI2r,
            0x77 => OpCode::DEO2r,
            0x78 => OpCode::ADD2r,
            0x79 => OpCode::SUB2r,
            0x7a => OpCode::MUL2r,
            0x7b => OpCode::DIV2r,
            0x7c => OpCode::AND2r,
            0x7d => OpCode::ORA2r,
            0x7e => OpCode::EOR2r,
            0x7f => OpCode::SFT2r,
            0x80 => OpCode::LIT,
            0x81 => OpCode::INCk,
            0x82 => OpCode::POPk,
            0x83 => OpCode::NIPk,
            0x84 => OpCode::SWPk,
            0x85 => OpCode::ROTk,
            0x86 => OpCode::DUPk,
            0x87 => OpCode::OVRk,
            0x88 => OpCode::EQUk,
            0x89 => OpCode::NEQk,
            0x8a => OpCode::GTHk,
            0x8b => OpCode::LTHk,
            0x8c => OpCode::JMPk,
            0x8d => OpCode::JCNk,
            0x8e => OpCode::JSRk,
            0x8f => OpCode::STHk,
            0x90 => OpCode::LDZk,
            0x91 => OpCode::STZk,
            0x92 => OpCode::LDRk,
            0x93 => OpCode::STRk,
            0x94 => OpCode::LDAk,
            0x95 => OpCode::STAk,
            0x96 => OpCode::DEIk,
            0x97 => OpCode::DEOk,
            0x98 => OpCode::ADDk,
            0x99 => OpCode::SUBk,
            0x9a => OpCode::MULk,
            0x9b => OpCode::DIVk,
            0x9c => OpCode::ANDk,
            0x9d => OpCode::ORAk,
            0x9e => OpCode::EORk,
            0x9f => OpCode::SFTk,
            0xa0 => OpCode::LIT2,
            0xa1 => OpCode::INC2k,
            0xa2 => OpCode::POP2k,
            0xa3 => OpCode::NIP2k,
            0xa4 => OpCode::SWP2k,
            0xa5 => OpCode::ROT2k,
            0xa6 => OpCode::DUP2k,
            0xa7 => OpCode::OVR2k,
            0xa8 => OpCode::EQU2k,
            0xa9 => OpCode::NEQ2k,
            0xaa => OpCode::GTH2k,
            0xab => OpCode::LTH2k,
            0xac => OpCode::JMP2k,
            0xad => OpCode::JCN2k,
            0xae => OpCode::JSR2k,
            0xaf => OpCode::STH2k,
            0xb0 => OpCode::LDZ2k,
            0xb1 => OpCode::STZ2k,
            0xb2 => OpCode::LDR2k,
            0xb3 => OpCode::STR2k,
            0xb4 => OpCode::LDA2k,
            0xb5 => OpCode::STA2k,
            0xb6 => OpCode::DEI2k,
            0xb7 => OpCode::DEO2k,
            0xb8 => OpCode::ADD2k,
            0xb9 => OpCode::SUB2k,
            0xba => OpCode::MUL2k,
            0xbb => OpCode::DIV2k,
            0xbc => OpCode::AND2k,
            0xbd => OpCode::ORA2k,
            0xbe => OpCode::EOR2k,
            0xbf => OpCode::SFT2k,
            0xc0 => OpCode::LITr,
            0xc1 => OpCode::INCkr,
            0xc2 => OpCode::POPkr,
            0xc3 => OpCode::NIPkr,
            0xc4 => OpCode::SWPkr,
            0xc5 => OpCode::ROTkr,
            0xc6 => OpCode::DUPkr,
            0xc7 => OpCode::OVRkr,
            0xc8 => OpCode::EQUkr,
            0xc9 => OpCode::NEQkr,
            0xca => OpCode::GTHkr,
            0xcb => OpCode::LTHkr,
            0xcc => OpCode::JMPkr,
            0xcd => OpCode::JCNkr,
            0xce => OpCode::JSRkr,
            0xcf => OpCode::STHkr,
            0xd0 => OpCode::LDZkr,
            0xd1 => OpCode::STZkr,
            0xd2 => OpCode::LDRkr,
            0xd3 => OpCode::STRkr,
            0xd4 => OpCode::LDAkr,
            0xd5 => OpCode::STAkr,
            0xd6 => OpCode::DEIkr,
            0xd7 => OpCode::DEOkr,
            0xd8 => OpCode::ADDkr,
            0xd9 => OpCode::SUBkr,
            0xda => OpCode::MULkr,
            0xdb => OpCode::DIVkr,
            0xdc => OpCode::ANDkr,
            0xdd => OpCode::ORAkr,
            0xde => OpCode::EORkr,
            0xdf => OpCode::SFTkr,
            0xe0 => OpCode::LIT2r,
            0xe1 => OpCode::INC2kr,
            0xe2 => OpCode::POP2kr,
            0xe3 => OpCode::NIP2kr,
            0xe4 => OpCode::SWP2kr,
            0xe5 => OpCode::ROT2kr,
            0xe6 => OpCode::DUP2kr,
            0xe7 => OpCode::OVR2kr,
            0xe8 => OpCode::EQU2kr,
            0xe9 => OpCode::NEQ2kr,
            0xea => OpCode::GTH2kr,
            0xeb => OpCode::LTH2kr,
            0xec => OpCode::JMP2kr,
            0xed => OpCode::JCN2kr,
            0xee => OpCode::JSR2kr,
            0xef => OpCode::STH2kr,
            0xf0 => OpCode::LDZ2kr,
            0xf1 => OpCode::STZ2kr,
            0xf2 => OpCode::LDR2kr,
            0xf3 => OpCode::STR2kr,
            0xf4 => OpCode::LDA2kr,
            0xf5 => OpCode::STA2kr,
            0xf6 => OpCode::DEI2kr,
            0xf7 => OpCode::DEO2kr,
            0xf8 => OpCode::ADD2kr,
            0xf9 => OpCode::SUB2kr,
            0xfa => OpCode::MUL2kr,
            0xfb => OpCode::DIV2kr,
            0xfc => OpCode::AND2kr,
            0xfd => OpCode::ORA2kr,
            0xfe => OpCode::EOR2kr,
            0xff => OpCode::SFT2kr,
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op_code: OpCode) -> u8 {
        match op_code {
            OpCode::BRK => 0x00,
            OpCode::INC => 0x01,
            OpCode::POP => 0x02,
            OpCode::NIP => 0x03,
            OpCode::SWP => 0x04,
            OpCode::ROT => 0x05,
            OpCode::DUP => 0x06,
            OpCode::OVR => 0x07,
            OpCode::EQU => 0x08,
            OpCode::NEQ => 0x09,
            OpCode::GTH => 0x0a,
            OpCode::LTH => 0x0b,
            OpCode::JMP => 0x0c,
            OpCode::JCN => 0x0d,
            OpCode::JSR => 0x0e,
            OpCode::STH => 0x0f,
            OpCode::LDZ => 0x10,
            OpCode::STZ => 0x11,
            OpCode::LDR => 0x12,
            OpCode::STR => 0x13,
            OpCode::LDA => 0x14,
            OpCode::STA => 0x15,
            OpCode::DEI => 0x16,
            OpCode::DEO => 0x17,
            OpCode::ADD => 0x18,
            OpCode::SUB => 0x19,
            OpCode::MUL => 0x1a,
            OpCode::DIV => 0x1b,
            OpCode::AND => 0x1c,
            OpCode::ORA => 0x1d,
            OpCode::EOR => 0x1e,
            OpCode::SFT => 0x1f,
            OpCode::NOP1 => 0x20,
            OpCode::INC2 => 0x21,
            OpCode::POP2 => 0x22,
            OpCode::NIP2 => 0x23,
            OpCode::SWP2 => 0x24,
            OpCode::ROT2 => 0x25,
            OpCode::DUP2 => 0x26,
            OpCode::OVR2 => 0x27,
            OpCode::EQU2 => 0x28,
            OpCode::NEQ2 => 0x29,
            OpCode::GTH2 => 0x2a,
            OpCode::LTH2 => 0x2b,
            OpCode::JMP2 => 0x2c,
            OpCode::JCN2 => 0x2d,
            OpCode::JSR2 => 0x2e,
            OpCode::STH2 => 0x2f,
            OpCode::LDZ2 => 0x30,
            OpCode::STZ2 => 0x31,
            OpCode::LDR2 => 0x32,
            OpCode::STR2 => 0x33,
            OpCode::LDA2 => 0x34,
            OpCode::STA2 => 0x35,
            OpCode::DEI2 => 0x36,
            OpCode::DEO2 => 0x37,
            OpCode::ADD2 => 0x38,
            OpCode::SUB2 => 0x39,
            OpCode::MUL2 => 0x3a,
            OpCode::DIV2 => 0x3b,
            OpCode::AND2 => 0x3c,
            OpCode::ORA2 => 0x3d,
            OpCode::EOR2 => 0x3e,
            OpCode::SFT2 => 0x3f,
            OpCode::NOP2 => 0x40,
            OpCode::INCr => 0x41,
            OpCode::POPr => 0x42,
            OpCode::NIPr => 0x43,
            OpCode::SWPr => 0x44,
            OpCode::ROTr => 0x45,
            OpCode::DUPr => 0x46,
            OpCode::OVRr => 0x47,
            OpCode::EQUr => 0x48,
            OpCode::NEQr => 0x49,
            OpCode::GTHr => 0x4a,
            OpCode::LTHr => 0x4b,
            OpCode::JMPr => 0x4c,
            OpCode::JCNr => 0x4d,
            OpCode::JSRr => 0x4e,
            OpCode::STHr => 0x4f,
            OpCode::LDZr => 0x50,
            OpCode::STZr => 0x51,
            OpCode::LDRr => 0x52,
            OpCode::STRr => 0x53,
            OpCode::LDAr => 0x54,
            OpCode::STAr => 0x55,
            OpCode::DEIr => 0x56,
            OpCode::DEOr => 0x57,
            OpCode::ADDr => 0x58,
            OpCode::SUBr => 0x59,
            OpCode::MULr => 0x5a,
            OpCode::DIVr => 0x5b,
            OpCode::ANDr => 0x5c,
            OpCode::ORAr => 0x5d,
            OpCode::EORr => 0x5e,
            OpCode::SFTr => 0x5f,
            OpCode::NOP3 => 0x60,
            OpCode::INC2r => 0x61,
            OpCode::POP2r => 0x62,
            OpCode::NIP2r => 0x63,
            OpCode::SWP2r => 0x64,
            OpCode::ROT2r => 0x65,
            OpCode::DUP2r => 0x66,
            OpCode::OVR2r => 0x67,
            OpCode::EQU2r => 0x68,
            OpCode::NEQ2r => 0x69,
            OpCode::GTH2r => 0x6a,
            OpCode::LTH2r => 0x6b,
            OpCode::JMP2r => 0x6c,
            OpCode::JCN2r => 0x6d,
            OpCode::JSR2r => 0x6e,
            OpCode::STH2r => 0x6f,
            OpCode::LDZ2r => 0x70,
            OpCode::STZ2r => 0x71,
            OpCode::LDR2r => 0x72,
            OpCode::STR2r => 0x73,
            OpCode::LDA2r => 0x74,
            OpCode::STA2r => 0x75,
            OpCode::DEI2r => 0x76,
            OpCode::DEO2r => 0x77,
            OpCode::ADD2r => 0x78,
            OpCode::SUB2r => 0x79,
            OpCode::MUL2r => 0x7a,
            OpCode::DIV2r => 0x7b,
            OpCode::AND2r => 0x7c,
            OpCode::ORA2r => 0x7d,
            OpCode::EOR2r => 0x7e,
            OpCode::SFT2r => 0x7f,
            OpCode::LIT => 0x80,
            OpCode::INCk => 0x81,
            OpCode::POPk => 0x82,
            OpCode::NIPk => 0x83,
            OpCode::SWPk => 0x84,
            OpCode::ROTk => 0x85,
            OpCode::DUPk => 0x86,
            OpCode::OVRk => 0x87,
            OpCode::EQUk => 0x88,
            OpCode::NEQk => 0x89,
            OpCode::GTHk => 0x8a,
            OpCode::LTHk => 0x8b,
            OpCode::JMPk => 0x8c,
            OpCode::JCNk => 0x8d,
            OpCode::JSRk => 0x8e,
            OpCode::STHk => 0x8f,
            OpCode::LDZk => 0x90,
            OpCode::STZk => 0x91,
            OpCode::LDRk => 0x92,
            OpCode::STRk => 0x93,
            OpCode::LDAk => 0x94,
            OpCode::STAk => 0x95,
            OpCode::DEIk => 0x96,
            OpCode::DEOk => 0x97,
            OpCode::ADDk => 0x98,
            OpCode::SUBk => 0x99,
            OpCode::MULk => 0x9a,
            OpCode::DIVk => 0x9b,
            OpCode::ANDk => 0x9c,
            OpCode::ORAk => 0x9d,
            OpCode::EORk => 0x9e,
            OpCode::SFTk => 0x9f,
            OpCode::LIT2 => 0xa0,
            OpCode::INC2k => 0xa1,
            OpCode::POP2k => 0xa2,
            OpCode::NIP2k => 0xa3,
            OpCode::SWP2k => 0xa4,
            OpCode::ROT2k => 0xa5,
            OpCode::DUP2k => 0xa6,
            OpCode::OVR2k => 0xa7,
            OpCode::EQU2k => 0xa8,
            OpCode::NEQ2k => 0xa9,
            OpCode::GTH2k => 0xaa,
            OpCode::LTH2k => 0xab,
            OpCode::JMP2k => 0xac,
            OpCode::JCN2k => 0xad,
            OpCode::JSR2k => 0xae,
            OpCode::STH2k => 0xaf,
            OpCode::LDZ2k => 0xb0,
            OpCode::STZ2k => 0xb1,
            OpCode::LDR2k => 0xb2,
            OpCode::STR2k => 0xb3,
            OpCode::LDA2k => 0xb4,
            OpCode::STA2k => 0xb5,
            OpCode::DEI2k => 0xb6,
            OpCode::DEO2k => 0xb7,
            OpCode::ADD2k => 0xb8,
            OpCode::SUB2k => 0xb9,
            OpCode::MUL2k => 0xba,
            OpCode::DIV2k => 0xbb,
            OpCode::AND2k => 0xbc,
            OpCode::ORA2k => 0xbd,
            OpCode::EOR2k => 0xbe,
            OpCode::SFT2k => 0xbf,
            OpCode::LITr => 0xc0,
            OpCode::INCkr => 0xc1,
            OpCode::POPkr => 0xc2,
            OpCode::NIPkr => 0xc3,
            OpCode::SWPkr => 0xc4,
            OpCode::ROTkr => 0xc5,
            OpCode::DUPkr => 0xc6,
            OpCode::OVRkr => 0xc7,
            OpCode::EQUkr => 0xc8,
            OpCode::NEQkr => 0xc9,
            OpCode::GTHkr => 0xca,
            OpCode::LTHkr => 0xcb,
            OpCode::JMPkr => 0xcc,
            OpCode::JCNkr => 0xcd,
            OpCode::JSRkr => 0xce,
            OpCode::STHkr => 0xcf,
            OpCode::LDZkr => 0xd0,
            OpCode::STZkr => 0xd1,
            OpCode::LDRkr => 0xd2,
            OpCode::STRkr => 0xd3,
            OpCode::LDAkr => 0xd4,
            OpCode::STAkr => 0xd5,
            OpCode::DEIkr => 0xd6,
            OpCode::DEOkr => 0xd7,
            OpCode::ADDkr => 0xd8,
            OpCode::SUBkr => 0xd9,
            OpCode::MULkr => 0xda,
            OpCode::DIVkr => 0xdb,
            OpCode::ANDkr => 0xdc,
            OpCode::ORAkr => 0xdd,
            OpCode::EORkr => 0xde,
            OpCode::SFTkr => 0xdf,
            OpCode::LIT2r => 0xe0,
            OpCode::INC2kr => 0xe1,
            OpCode::POP2kr => 0xe2,
            OpCode::NIP2kr => 0xe3,
            OpCode::SWP2kr => 0xe4,
            OpCode::ROT2kr => 0xe5,
            OpCode::DUP2kr => 0xe6,
            OpCode::OVR2kr => 0xe7,
            OpCode::EQU2kr => 0xe8,
            OpCode::NEQ2kr => 0xe9,
            OpCode::GTH2kr => 0xea,
            OpCode::LTH2kr => 0xeb,
            OpCode::JMP2kr => 0xec,
            OpCode::JCN2kr => 0xed,
            OpCode::JSR2kr => 0xee,
            OpCode::STH2kr => 0xef,
            OpCode::LDZ2kr => 0xf0,
            OpCode::STZ2kr => 0xf1,
            OpCode::LDR2kr => 0xf2,
            OpCode::STR2kr => 0xf3,
            OpCode::LDA2kr => 0xf4,
            OpCode::STA2kr => 0xf5,
            OpCode::DEI2kr => 0xf6,
            OpCode::DEO2kr => 0xf7,
            OpCode::ADD2kr => 0xf8,
            OpCode::SUB2kr => 0xf9,
            OpCode::MUL2kr => 0xfa,
            OpCode::DIV2kr => 0xfb,
            OpCode::AND2kr => 0xfc,
            OpCode::ORA2kr => 0xfd,
            OpCode::EOR2kr => 0xfe,
            OpCode::SFT2kr => 0xff,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::BRK => write!(f, "BRK"),
            OpCode::INC => write!(f, "INC"),
            OpCode::POP => write!(f, "POP"),
            OpCode::NIP => write!(f, "NIP"),
            OpCode::SWP => write!(f, "SWP"),
            OpCode::ROT => write!(f, "ROT"),
            OpCode::DUP => write!(f, "DUP"),
            OpCode::OVR => write!(f, "OVR"),
            OpCode::EQU => write!(f, "EQU"),
            OpCode::NEQ => write!(f, "NEQ"),
            OpCode::GTH => write!(f, "GTH"),
            OpCode::LTH => write!(f, "LTH"),
            OpCode::JMP => write!(f, "JMP"),
            OpCode::JCN => write!(f, "JCN"),
            OpCode::JSR => write!(f, "JSR"),
            OpCode::STH => write!(f, "STH"),
            OpCode::LDZ => write!(f, "LDZ"),
            OpCode::STZ => write!(f, "STZ"),
            OpCode::LDR => write!(f, "LDR"),
            OpCode::STR => write!(f, "STR"),
            OpCode::LDA => write!(f, "LDA"),
            OpCode::STA => write!(f, "STA"),
            OpCode::DEI => write!(f, "DEI"),
            OpCode::DEO => write!(f, "DEO"),
            OpCode::ADD => write!(f, "ADD"),
            OpCode::SUB => write!(f, "SUB"),
            OpCode::MUL => write!(f, "MUL"),
            OpCode::DIV => write!(f, "DIV"),
            OpCode::AND => write!(f, "AND"),
            OpCode::ORA => write!(f, "ORA"),
            OpCode::EOR => write!(f, "EOR"),
            OpCode::SFT => write!(f, "SFT"),
            OpCode::NOP1 => write!(f, "NOP1"),
            OpCode::INC2 => write!(f, "INC2"),
            OpCode::POP2 => write!(f, "POP2"),
            OpCode::NIP2 => write!(f, "NIP2"),
            OpCode::SWP2 => write!(f, "SWP2"),
            OpCode::ROT2 => write!(f, "ROT2"),
            OpCode::DUP2 => write!(f, "DUP2"),
            OpCode::OVR2 => write!(f, "OVR2"),
            OpCode::EQU2 => write!(f, "EQU2"),
            OpCode::NEQ2 => write!(f, "NEQ2"),
            OpCode::GTH2 => write!(f, "GTH2"),
            OpCode::LTH2 => write!(f, "LTH2"),
            OpCode::JMP2 => write!(f, "JMP2"),
            OpCode::JCN2 => write!(f, "JCN2"),
            OpCode::JSR2 => write!(f, "JSR2"),
            OpCode::STH2 => write!(f, "STH2"),
            OpCode::LDZ2 => write!(f, "LDZ2"),
            OpCode::STZ2 => write!(f, "STZ2"),
            OpCode::LDR2 => write!(f, "LDR2"),
            OpCode::STR2 => write!(f, "STR2"),
            OpCode::LDA2 => write!(f, "LDA2"),
            OpCode::STA2 => write!(f, "STA2"),
            OpCode::DEI2 => write!(f, "DEI2"),
            OpCode::DEO2 => write!(f, "DEO2"),
            OpCode::ADD2 => write!(f, "ADD2"),
            OpCode::SUB2 => write!(f, "SUB2"),
            OpCode::MUL2 => write!(f, "MUL2"),
            OpCode::DIV2 => write!(f, "DIV2"),
            OpCode::AND2 => write!(f, "AND2"),
            OpCode::ORA2 => write!(f, "ORA2"),
            OpCode::EOR2 => write!(f, "EOR2"),
            OpCode::SFT2 => write!(f, "SFT2"),
            OpCode::NOP2 => write!(f, "NOP2"),
            OpCode::INCr => write!(f, "INCr"),
            OpCode::POPr => write!(f, "POPr"),
            OpCode::NIPr => write!(f, "NIPr"),
            OpCode::SWPr => write!(f, "SWPr"),
            OpCode::ROTr => write!(f, "ROTr"),
            OpCode::DUPr => write!(f, "DUPr"),
            OpCode::OVRr => write!(f, "OVRr"),
            OpCode::EQUr => write!(f, "EQUr"),
            OpCode::NEQr => write!(f, "NEQr"),
            OpCode::GTHr => write!(f, "GTHr"),
            OpCode::LTHr => write!(f, "LTHr"),
            OpCode::JMPr => write!(f, "JMPr"),
            OpCode::JCNr => write!(f, "JCNr"),
            OpCode::JSRr => write!(f, "JSRr"),
            OpCode::STHr => write!(f, "STHr"),
            OpCode::LDZr => write!(f, "LDZr"),
            OpCode::STZr => write!(f, "STZr"),
            OpCode::LDRr => write!(f, "LDRr"),
            OpCode::STRr => write!(f, "STRr"),
            OpCode::LDAr => write!(f, "LDAr"),
            OpCode::STAr => write!(f, "STAr"),
            OpCode::DEIr => write!(f, "DEIr"),
            OpCode::DEOr => write!(f, "DEOr"),
            OpCode::ADDr => write!(f, "ADDr"),
            OpCode::SUBr => write!(f, "SUBr"),
            OpCode::MULr => write!(f, "MULr"),
            OpCode::DIVr => write!(f, "DIVr"),
            OpCode::ANDr => write!(f, "ANDr"),
            OpCode::ORAr => write!(f, "ORAr"),
            OpCode::EORr => write!(f, "EORr"),
            OpCode::SFTr => write!(f, "SFTr"),
            OpCode::NOP3 => write!(f, "NOP3"),
            OpCode::INC2r => write!(f, "INC2r"),
            OpCode::POP2r => write!(f, "POP2r"),
            OpCode::NIP2r => write!(f, "NIP2r"),
            OpCode::SWP2r => write!(f, "SWP2r"),
            OpCode::ROT2r => write!(f, "ROT2r"),
            OpCode::DUP2r => write!(f, "DUP2r"),
            OpCode::OVR2r => write!(f, "OVR2r"),
            OpCode::EQU2r => write!(f, "EQU2r"),
            OpCode::NEQ2r => write!(f, "NEQ2r"),
            OpCode::GTH2r => write!(f, "GTH2r"),
            OpCode::LTH2r => write!(f, "LTH2r"),
            OpCode::JMP2r => write!(f, "JMP2r"),
            OpCode::JCN2r => write!(f, "JCN2r"),
            OpCode::JSR2r => write!(f, "JSR2r"),
            OpCode::STH2r => write!(f, "STH2r"),
            OpCode::LDZ2r => write!(f, "LDZ2r"),
            OpCode::STZ2r => write!(f, "STZ2r"),
            OpCode::LDR2r => write!(f, "LDR2r"),
            OpCode::STR2r => write!(f, "STR2r"),
            OpCode::LDA2r => write!(f, "LDA2r"),
            OpCode::STA2r => write!(f, "STA2r"),
            OpCode::DEI2r => write!(f, "DEI2r"),
            OpCode::DEO2r => write!(f, "DEO2r"),
            OpCode::ADD2r => write!(f, "ADD2r"),
            OpCode::SUB2r => write!(f, "SUB2r"),
            OpCode::MUL2r => write!(f, "MUL2r"),
            OpCode::DIV2r => write!(f, "DIV2r"),
            OpCode::AND2r => write!(f, "AND2r"),
            OpCode::ORA2r => write!(f, "ORA2r"),
            OpCode::EOR2r => write!(f, "EOR2r"),
            OpCode::SFT2r => write!(f, "SFT2r"),
            OpCode::LIT => write!(f, "LIT"),
            OpCode::INCk => write!(f, "INCk"),
            OpCode::POPk => write!(f, "POPk"),
            OpCode::NIPk => write!(f, "NIPk"),
            OpCode::SWPk => write!(f, "SWPk"),
            OpCode::ROTk => write!(f, "ROTk"),
            OpCode::DUPk => write!(f, "DUPk"),
            OpCode::OVRk => write!(f, "OVRk"),
            OpCode::EQUk => write!(f, "EQUk"),
            OpCode::NEQk => write!(f, "NEQk"),
            OpCode::GTHk => write!(f, "GTHk"),
            OpCode::LTHk => write!(f, "LTHk"),
            OpCode::JMPk => write!(f, "JMPk"),
            OpCode::JCNk => write!(f, "JCNk"),
            OpCode::JSRk => write!(f, "JSRk"),
            OpCode::STHk => write!(f, "STHk"),
            OpCode::LDZk => write!(f, "LDZk"),
            OpCode::STZk => write!(f, "STZk"),
            OpCode::LDRk => write!(f, "LDRk"),
            OpCode::STRk => write!(f, "STRk"),
            OpCode::LDAk => write!(f, "LDAk"),
            OpCode::STAk => write!(f, "STAk"),
            OpCode::DEIk => write!(f, "DEIk"),
            OpCode::DEOk => write!(f, "DEOk"),
            OpCode::ADDk => write!(f, "ADDk"),
            OpCode::SUBk => write!(f, "SUBk"),
            OpCode::MULk => write!(f, "MULk"),
            OpCode::DIVk => write!(f, "DIVk"),
            OpCode::ANDk => write!(f, "ANDk"),
            OpCode::ORAk => write!(f, "ORAk"),
            OpCode::EORk => write!(f, "EORk"),
            OpCode::SFTk => write!(f, "SFTk"),
            OpCode::LIT2 => write!(f, "LIT2"),
            OpCode::INC2k => write!(f, "INC2k"),
            OpCode::POP2k => write!(f, "POP2k"),
            OpCode::NIP2k => write!(f, "NIP2k"),
            OpCode::SWP2k => write!(f, "SWP2k"),
            OpCode::ROT2k => write!(f, "ROT2k"),
            OpCode::DUP2k => write!(f, "DUP2k"),
            OpCode::OVR2k => write!(f, "OVR2k"),
            OpCode::EQU2k => write!(f, "EQU2k"),
            OpCode::NEQ2k => write!(f, "NEQ2k"),
            OpCode::GTH2k => write!(f, "GTH2k"),
            OpCode::LTH2k => write!(f, "LTH2k"),
            OpCode::JMP2k => write!(f, "JMP2k"),
            OpCode::JCN2k => write!(f, "JCN2k"),
            OpCode::JSR2k => write!(f, "JSR2k"),
            OpCode::STH2k => write!(f, "STH2k"),
            OpCode::LDZ2k => write!(f, "LDZ2k"),
            OpCode::STZ2k => write!(f, "STZ2k"),
            OpCode::LDR2k => write!(f, "LDR2k"),
            OpCode::STR2k => write!(f, "STR2k"),
            OpCode::LDA2k => write!(f, "LDA2k"),
            OpCode::STA2k => write!(f, "STA2k"),
            OpCode::DEI2k => write!(f, "DEI2k"),
            OpCode::DEO2k => write!(f, "DEO2k"),
            OpCode::ADD2k => write!(f, "ADD2k"),
            OpCode::SUB2k => write!(f, "SUB2k"),
            OpCode::MUL2k => write!(f, "MUL2k"),
            OpCode::DIV2k => write!(f, "DIV2k"),
            OpCode::AND2k => write!(f, "AND2k"),
            OpCode::ORA2k => write!(f, "ORA2k"),
            OpCode::EOR2k => write!(f, "EOR2k"),
            OpCode::SFT2k => write!(f, "SFT2k"),
            OpCode::LITr => write!(f, "LITr"),
            OpCode::INCkr => write!(f, "INCkr"),
            OpCode::POPkr => write!(f, "POPkr"),
            OpCode::NIPkr => write!(f, "NIPkr"),
            OpCode::SWPkr => write!(f, "SWPkr"),
            OpCode::ROTkr => write!(f, "ROTkr"),
            OpCode::DUPkr => write!(f, "DUPkr"),
            OpCode::OVRkr => write!(f, "OVRkr"),
            OpCode::EQUkr => write!(f, "EQUkr"),
            OpCode::NEQkr => write!(f, "NEQkr"),
            OpCode::GTHkr => write!(f, "GTHkr"),
            OpCode::LTHkr => write!(f, "LTHkr"),
            OpCode::JMPkr => write!(f, "JMPkr"),
            OpCode::JCNkr => write!(f, "JCNkr"),
            OpCode::JSRkr => write!(f, "JSRkr"),
            OpCode::STHkr => write!(f, "STHkr"),
            OpCode::LDZkr => write!(f, "LDZkr"),
            OpCode::STZkr => write!(f, "STZkr"),
            OpCode::LDRkr => write!(f, "LDRkr"),
            OpCode::STRkr => write!(f, "STRkr"),
            OpCode::LDAkr => write!(f, "LDAkr"),
            OpCode::STAkr => write!(f, "STAkr"),
            OpCode::DEIkr => write!(f, "DEIkr"),
            OpCode::DEOkr => write!(f, "DEOkr"),
            OpCode::ADDkr => write!(f, "ADDkr"),
            OpCode::SUBkr => write!(f, "SUBkr"),
            OpCode::MULkr => write!(f, "MULkr"),
            OpCode::DIVkr => write!(f, "DIVkr"),
            OpCode::ANDkr => write!(f, "ANDkr"),
            OpCode::ORAkr => write!(f, "ORAkr"),
            OpCode::EORkr => write!(f, "EORkr"),
            OpCode::SFTkr => write!(f, "SFTkr"),
            OpCode::LIT2r => write!(f, "LIT2r"),
            OpCode::INC2kr => write!(f, "INC2kr"),
            OpCode::POP2kr => write!(f, "POP2kr"),
            OpCode::NIP2kr => write!(f, "NIP2kr"),
            OpCode::SWP2kr => write!(f, "SWP2kr"),
            OpCode::ROT2kr => write!(f, "ROT2kr"),
            OpCode::DUP2kr => write!(f, "DUP2kr"),
            OpCode::OVR2kr => write!(f, "OVR2kr"),
            OpCode::EQU2kr => write!(f, "EQU2kr"),
            OpCode::NEQ2kr => write!(f, "NEQ2kr"),
            OpCode::GTH2kr => write!(f, "GTH2kr"),
            OpCode::LTH2kr => write!(f, "LTH2kr"),
            OpCode::JMP2kr => write!(f, "JMP2kr"),
            OpCode::JCN2kr => write!(f, "JCN2kr"),
            OpCode::JSR2kr => write!(f, "JSR2kr"),
            OpCode::STH2kr => write!(f, "STH2kr"),
            OpCode::LDZ2kr => write!(f, "LDZ2kr"),
            OpCode::STZ2kr => write!(f, "STZ2kr"),
            OpCode::LDR2kr => write!(f, "LDR2kr"),
            OpCode::STR2kr => write!(f, "STR2kr"),
            OpCode::LDA2kr => write!(f, "LDA2kr"),
            OpCode::STA2kr => write!(f, "STA2kr"),
            OpCode::DEI2kr => write!(f, "DEI2kr"),
            OpCode::DEO2kr => write!(f, "DEO2kr"),
            OpCode::ADD2kr => write!(f, "ADD2kr"),
            OpCode::SUB2kr => write!(f, "SUB2kr"),
            OpCode::MUL2kr => write!(f, "MUL2kr"),
            OpCode::DIV2kr => write!(f, "DIV2kr"),
            OpCode::AND2kr => write!(f, "AND2kr"),
            OpCode::ORA2kr => write!(f, "ORA2kr"),
            OpCode::EOR2kr => write!(f, "EOR2kr"),
            OpCode::SFT2kr => write!(f, "SFT2kr"),
        }
    }
}
