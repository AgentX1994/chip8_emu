/// Opcodes
/// Mnemonics are mine

pub enum Opcode {
    CallAddress {
        address: u16,
    }, // 0NNN
    ClearScreen, // 00E0
    Return,      // 00EE
    Goto {
        address: u16,
    }, // 1NNN
    CallSubroutine {
        address: u16,
    }, // 2NNN
    IfRegEqual {
        register: u8,
        immediate: u8,
    }, // 3XNN
    IfRegNotEqual {
        register: u8,
        immediate: u8,
    }, // 4XNN
    IfRegsEqual {
        register1: u8,
        register2: u8,
    }, // 5XY0
    SetRegister {
        register: u8,
        immediate: u8,
    }, // 6XNN
    AddToRegister {
        register: u8,
        immediate: u8,
    }, // 7XNN
    MoveRegToReg {
        register1: u8,
        register2: u8,
    }, // 8XY0
    BitwiseOrRegs {
        register1: u8,
        register2: u8,
    }, // 8XY1
    BitwiseAndRegs {
        register1: u8,
        register2: u8,
    }, // 8XY2
    BitwiseXorRegs {
        register1: u8,
        register2: u8,
    }, // 8XY3
    AddRegs {
        register1: u8,
        register2: u8,
    }, // 8XY4
    SubtractRegs {
        register1: u8,
        register2: u8,
    }, // 8XY5
    RightShiftReg {
        register1: u8,
    }, // 8XY6
    SubtractRegsOppositeOrder {
        register1: u8,
        register2: u8,
    }, // 8XY7
    LeftShiftReg {
        register1: u8,
    }, // 8XY8
    IfRegsNotEqual {
        register1: u8,
        register2: u8,
    }, // 9XY0
    SetIToAddress {
        address: u16,
    }, // ANNN
    JumpIndirect {
        address: u16,
    }, // BNNN
    Rand {
        register: u8,
        immediate: u8,
    }, // CXNN
    Draw {
        register1: u8,
        register2: u8,
        height: u8,
    }, // DXYN
    IfKeyEqual {
        register: u8,
    }, // EX9E
    IfKeyNotEqual {
        register: u8,
    }, // EXA1
    GetDelay {
        register: u8,
    }, // FX07
    GetKey {
        register: u8,
    }, // FX0A
    SetDelay {
        register: u8,
    }, // FX15
    SetSound {
        register: u8,
    }, // FX18
    AddRegToI {
        register: u8,
    }, // FX1E
    GetSpriteAddr {
        register: u8,
    }, // FX29
    ToBinaryCodedDecimal {
        register: u8,
    }, // FX33
    DumpRegistersUntil {
        register: u8,
    }, // FX55
    LoadRegistersUntil {
        register: u8,
    }, // FX65
    Unknown {
        opcode: u16,
    }, // Anything else
}

impl From<u16> for Opcode {
    fn from(instruction: u16) -> Opcode {
        match instruction & 0xF000 {
            0x0000 => match instruction & 0x0FFF {
                0x0E0 => return Opcode::ClearScreen,
                0x0EE => return Opcode::Return,
                address => return Opcode::CallAddress { address },
            },
            0x1000 => {
                return Opcode::Goto {
                    address: instruction & 0xFFF,
                }
            }
            0x2000 => {
                return Opcode::CallSubroutine {
                    address: instruction & 0xFFF,
                }
            }
            0x3000 => {
                return Opcode::IfRegEqual {
                    register: ((instruction & 0xF00) >> 8) as u8,
                    immediate: (instruction & 0xFF) as u8,
                }
            }
            0x4000 => {
                return Opcode::IfRegNotEqual {
                    register: ((instruction & 0xF00) >> 8) as u8,
                    immediate: (instruction & 0xFF) as u8,
                }
            }
            0x5000 => {
                if instruction & 0xF == 0 {
                    return Opcode::IfRegsEqual {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
            }
            0x6000 => {
                return Opcode::SetRegister {
                    register: ((instruction & 0xF00) >> 8) as u8,
                    immediate: (instruction & 0xFF) as u8,
                }
            }
            0x7000 => {
                return Opcode::AddToRegister {
                    register: ((instruction & 0xF00) >> 8) as u8,
                    immediate: (instruction & 0xFF) as u8,
                }
            }
            0x8000 => match instruction & 0xF {
                0x0 => {
                    return Opcode::MoveRegToReg {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
                0x1 => {
                    return Opcode::BitwiseOrRegs {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
                0x2 => {
                    return Opcode::BitwiseAndRegs {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
                0x3 => {
                    return Opcode::BitwiseXorRegs {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
                0x4 => {
                    return Opcode::AddRegs {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
                0x5 => {
                    return Opcode::SubtractRegs {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
                0x6 => {
                    return Opcode::RightShiftReg {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                    };
                }
                0x7 => {
                    return Opcode::SubtractRegsOppositeOrder {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
                0xE => {
                    return Opcode::LeftShiftReg {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                    };
                }
                _ => (),
            },
            0x9000 => {
                if instruction & 0xF == 0 {
                    return Opcode::IfRegsNotEqual {
                        register1: ((instruction & 0xF00) >> 8) as u8,
                        register2: ((instruction & 0xF0) >> 4) as u8,
                    };
                }
            }
            0xA000 => {
                return Opcode::SetIToAddress {
                    address: instruction & 0xFFF,
                }
            }
            0xB000 => {
                return Opcode::JumpIndirect {
                    address: instruction & 0xFFF,
                }
            }
            0xC000 => {
                return Opcode::Rand {
                    register: ((instruction & 0xF00) >> 8) as u8,
                    immediate: (instruction & 0xFF) as u8,
                }
            }
            0xD000 => {
                return Opcode::Draw {
                    register1: ((instruction & 0xF00) >> 8) as u8,
                    register2: ((instruction & 0xF0) >> 4) as u8,
                    height: (instruction & 0xF) as u8, // + 1,
                };
            }
            0xE000 => match instruction & 0xFF {
                0x9E => {
                    return Opcode::IfKeyEqual {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0xA1 => {
                    return Opcode::IfKeyNotEqual {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                _ => (),
            },
            0xF000 => match instruction & 0xFF {
                0x07 => {
                    return Opcode::GetDelay {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x0A => {
                    return Opcode::GetKey {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x15 => {
                    return Opcode::SetDelay {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x18 => {
                    return Opcode::SetSound {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x1E => {
                    return Opcode::AddRegToI {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x29 => {
                    return Opcode::GetSpriteAddr {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x33 => {
                    return Opcode::ToBinaryCodedDecimal {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x55 => {
                    return Opcode::DumpRegistersUntil {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                0x65 => {
                    return Opcode::LoadRegistersUntil {
                        register: ((instruction & 0xF00) >> 8) as u8,
                    }
                }
                _ => (),
            },
            _ => unreachable!(),
        }

        Opcode::Unknown {
            opcode: instruction,
        }
    }
}
