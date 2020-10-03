use crate::font::FONT_SET;
use crate::keypad::Keypad;
use crate::memory::Memory;
use crate::opcode::Opcode;
use crate::screen::Screen;

use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

// TODO: Extract SDL from CPU implementation :(
use sdl2::event::Event;

pub struct Cpu {
    registers: [u8; 16],
    memory: Memory,
    i_reg: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    program_counter: u16,
    screen: Screen,
    keypad: Keypad,
    delay_timer: u8,
    delay_timer_instant: Instant,
    sound_timer: u8,
    sound_timer_instant: Instant,
    draw_flag: bool,
    waiting_for_key: bool,
    register_for_key: u8,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            registers: [0; 16],
            memory: Memory::default(),
            i_reg: 0,
            stack: [0; 16],
            stack_pointer: 0,
            program_counter: 0,
            screen: Screen::default(),
            keypad: Keypad::default(),
            delay_timer: 0,
            delay_timer_instant: Instant::now(),
            sound_timer: 0,
            sound_timer_instant: Instant::now(),
            draw_flag: false,
            waiting_for_key: true,
            register_for_key: 0,
        };
        cpu.reset();
        return cpu;
    }

    pub fn reset(&mut self) {
        self.screen.reset();
        self.memory.reset();
        self.keypad.reset();
        self.registers = [0; 16];
        self.stack = [0; 16];
        self.stack_pointer = 0;
        self.i_reg = 0;
        self.program_counter = 0x200;
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.load_fontset();
    }

    pub fn emulate_cycle(&mut self) {
        if !self.waiting_for_key {
            let opcode = Opcode::from(self.memory.get_u16(self.program_counter).unwrap());
            self.program_counter += 2;

            match opcode {
                Opcode::CallAddress { address: _ } => {
                    // TODO
                }
                Opcode::ClearScreen => {
                    self.screen.reset();
                }
                Opcode::Return => {
                    if self.stack_pointer == 0 {
                        panic!("Stack underflow at address {:X}", self.program_counter);
                    }
                    self.stack_pointer -= 1;
                    self.program_counter = self.stack[self.stack_pointer as usize];
                    self.program_counter += 2;
                }
                Opcode::Goto { address } => {
                    self.program_counter = address;
                }
                Opcode::CallSubroutine { address } => {
                    if self.stack_pointer == 16 {
                        panic!("Stack overflow at address {:X}!", self.program_counter);
                    }
                    self.stack[self.stack_pointer as usize] = self.program_counter;
                    self.stack_pointer += 1;
                    self.program_counter = address;
                }
                Opcode::IfRegEqual {
                    register,
                    immediate,
                } => {
                    assert!(register < 16);
                    if self.registers[register as usize] == immediate {
                        // Skip the next instruction
                        self.program_counter += 2;
                    }
                }
                Opcode::IfRegNotEqual {
                    register,
                    immediate,
                } => {
                    assert!(register < 16);
                    if self.registers[register as usize] != immediate {
                        // Skip the next instruction
                        self.program_counter += 2;
                    }
                }
                Opcode::IfRegsEqual {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    if self.registers[register1 as usize] == self.registers[register2 as usize] {
                        // Skip the next instruction
                        self.program_counter += 2;
                    }
                }
                Opcode::SetRegister {
                    register,
                    immediate,
                } => {
                    assert!(register < 16);
                    self.registers[register as usize] = immediate;
                }
                Opcode::AddToRegister {
                    register,
                    immediate,
                } => {
                    assert!(register < 16);
                    self.registers[register as usize] =
                        self.registers[register as usize].wrapping_add(immediate);
                }
                Opcode::MoveRegToReg {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    self.registers[register1 as usize] = self.registers[register2 as usize];
                }
                Opcode::BitwiseOrRegs {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    let result =
                        self.registers[register1 as usize] | self.registers[register2 as usize];
                    self.registers[register1 as usize] = result;
                }
                Opcode::BitwiseAndRegs {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    let result =
                        self.registers[register1 as usize] & self.registers[register2 as usize];
                    self.registers[register1 as usize] = result;
                }
                Opcode::BitwiseXorRegs {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    let result =
                        self.registers[register1 as usize] ^ self.registers[register2 as usize];
                    self.registers[register1 as usize] = result;
                }
                Opcode::AddRegs {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    let (result, overflow) = self.registers[register1 as usize]
                        .overflowing_add(self.registers[register2 as usize]);
                    self.registers[0xF] = overflow as u8;
                    self.registers[register1 as usize] = result;
                }
                Opcode::SubtractRegs {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    let (result, overflow) = self.registers[register1 as usize]
                        .overflowing_sub(self.registers[register2 as usize]);
                    self.registers[0xF] = (!overflow) as u8;
                    self.registers[register1 as usize] = result;
                }
                Opcode::RightShiftReg { register1 } => {
                    assert!(register1 < 16);
                    let value = self.registers[register1 as usize];
                    self.registers[0xF] = value & 1;
                    self.registers[register1 as usize] = value >> 1;
                }
                Opcode::SubtractRegsOppositeOrder {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    let (result, overflow) = self.registers[register2 as usize]
                        .overflowing_sub(self.registers[register1 as usize]);
                    self.registers[0xF] = (!overflow) as u8;
                    self.registers[register1 as usize] = result;
                }
                Opcode::LeftShiftReg { register1 } => {
                    assert!(register1 < 16);
                    let value = self.registers[register1 as usize];
                    self.registers[0xF] = value >> 7;
                    self.registers[register1 as usize] = value << 1;
                }
                Opcode::IfRegsNotEqual {
                    register1,
                    register2,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    if self.registers[register1 as usize] != self.registers[register2 as usize] {
                        // Skip the next instruction
                        self.program_counter += 2;
                    }
                }
                Opcode::SetIToAddress { address } => {
                    self.i_reg = address;
                }
                Opcode::JumpIndirect { address } => {
                    self.program_counter = self.registers[0] as u16 + address;
                }
                Opcode::Rand {
                    register,
                    immediate,
                } => {
                    assert!(register < 16);
                    self.registers[register as usize] = rand::random::<u8>() & immediate;
                }
                Opcode::Draw {
                    register1,
                    register2,
                    height,
                } => {
                    assert!(register1 < 16 && register2 < 16);
                    let sprite = self.memory.get_data(self.i_reg, height as u16).unwrap();
                    let x = self.registers[register1 as usize];
                    let y = self.registers[register2 as usize];
                    self.registers[0xF] = if self.screen.draw_sprite(x as u16, y as u16, sprite) {
                        1
                    } else {
                        0
                    };
                    self.draw_flag = true;
                }
                Opcode::IfKeyEqual { register } => {
                    assert!(register < 16);
                    let is_pressed = self
                        .keypad
                        .is_key_pressed(self.registers[register as usize]);
                    if is_pressed {
                        // Skip next instruction
                        self.program_counter += 2
                    }
                }
                Opcode::IfKeyNotEqual { register } => {
                    assert!(register < 16);
                    let is_pressed = self
                        .keypad
                        .is_key_pressed(self.registers[register as usize]);
                    if !is_pressed {
                        // Skip next instruction
                        self.program_counter += 2
                    }
                }
                Opcode::GetDelay { register } => {
                    assert!(register < 16);
                    self.registers[register as usize] = self.delay_timer;
                }
                Opcode::GetKey { register } => {
                    assert!(register < 16);
                    self.waiting_for_key = true;
                    self.register_for_key = register;
                }
                Opcode::SetDelay { register } => {
                    assert!(register < 16);
                    self.delay_timer = self.registers[register as usize];
                    self.delay_timer_instant = Instant::now();
                }
                Opcode::SetSound { register } => {
                    assert!(register < 16);
                    self.sound_timer = self.registers[register as usize];
                    self.sound_timer_instant = Instant::now();
                }
                Opcode::AddRegToI { register } => {
                    assert!(register < 16);
                    self.i_reg = self.i_reg + self.registers[register as usize] as u16;
                    if self.i_reg > 0xFFF {
                        // wrap
                        self.i_reg -= 0xFFF;
                    }
                }
                Opcode::GetSpriteAddr { register } => {
                    assert!(register < 16);
                    let sprite = self.registers[register as usize];
                    assert!(sprite < 16);
                    // font map starts at 0x50, and each sprite is 5 bytes
                    self.i_reg = 0x50 + (sprite as u16) * 5;
                }
                Opcode::ToBinaryCodedDecimal { register } => {
                    assert!(register < 16);
                    let value_to_convert = self.registers[register as usize];
                    // I'm not sure if this is the best way to do this, but it works
                    self.memory
                        .write_u8(self.i_reg, value_to_convert / 100)
                        .unwrap();
                    self.memory
                        .write_u8(self.i_reg + 1, (value_to_convert % 100) / 10)
                        .unwrap();
                    self.memory
                        .write_u8(self.i_reg + 2, value_to_convert % 10)
                        .unwrap();
                }
                Opcode::DumpRegistersUntil { register } => {
                    assert!(register < 16);
                    for reg in 0..=register as u16 {
                        self.memory
                            .write_u8(self.i_reg + reg, self.registers[reg as usize])
                            .unwrap();
                    }
                }
                Opcode::LoadRegistersUntil { register } => {
                    assert!(register < 16);
                    for reg in 0..=register as u16 {
                        self.registers[reg as usize] =
                            self.memory.get_u8(self.i_reg + reg).unwrap();
                    }
                }
                Opcode::Unknown { opcode } => eprintln!("Unknown opcode {:X}", opcode),
            }

            while self.program_counter > 0xFFF {
                self.program_counter -= 0xFFF;
            }
        }

        const COUNTDOWN_RATE: u64 = 60; // HZ
        const NANOS_PER_SECOND: u64 = 1_000_000_000;
        const TIME_BETWEEN_COUNTS: u64 = NANOS_PER_SECOND / COUNTDOWN_RATE;
        let now = Instant::now();
        if self.delay_timer > 0 {
            if now - self.delay_timer_instant > Duration::from_nanos(TIME_BETWEEN_COUNTS) {
                self.delay_timer -= 1;
                self.delay_timer_instant = now;
            }
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            if now - self.sound_timer_instant > Duration::from_secs(TIME_BETWEEN_COUNTS) {
                self.sound_timer -= 1;
                self.sound_timer_instant = now;
            }
        }
    }

    pub fn load_program<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let program = fs::read(path)?;
        self.memory.write_data(0x200, &program[..]).unwrap();
        Ok(())
    }

    pub fn draw_needed(&self) -> bool {
        self.draw_flag
    }

    pub fn get_pixel_data(&self) -> &[u8] {
        self.screen.get_pixel_data()
    }

    pub fn handle_input(&mut self, event: Event) {
        let handled = self.keypad.handle_input(event);
        if self.waiting_for_key && handled {
            self.waiting_for_key = false;
            self.registers[self.register_for_key as usize] = self.keypad.get_last_key();
        }
    }

    fn load_fontset(&mut self) {
        self.memory.write_data(0x50, &FONT_SET[..]).unwrap();
    }
}
