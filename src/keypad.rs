#[derive(Default)]
pub struct Keypad {
    keys: [u8; 16],
    last_key: u8,
}

// TODO: Extract SDL from Keypad implementation :(
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

impl Keypad {
    pub fn reset(&mut self) {
        self.keys = [0; 16];
        self.last_key = 0;
    }

    pub fn handle_input(&mut self, event: Event) -> bool {
        let ret = match event {
            Event::KeyDown {
                keycode: Some(k), ..
            } => match k {
                Keycode::X => {
                    self.keys[0] = 1;
                    self.last_key = 0;
                    true
                }
                Keycode::Num1 => {
                    self.keys[1] = 1;
                    self.last_key = 1;
                    true
                }
                Keycode::Num2 => {
                    self.keys[2] = 1;
                    self.last_key = 2;
                    true
                }
                Keycode::Num3 => {
                    self.keys[3] = 1;
                    self.last_key = 3;
                    true
                }
                Keycode::Q => {
                    self.keys[4] = 1;
                    self.last_key = 4;
                    true
                }
                Keycode::W => {
                    self.keys[5] = 1;
                    self.last_key = 5;
                    true
                }
                Keycode::E => {
                    self.keys[6] = 1;
                    self.last_key = 6;
                    true
                }
                Keycode::A => {
                    self.keys[7] = 1;
                    self.last_key = 7;
                    true
                }
                Keycode::S => {
                    self.keys[8] = 1;
                    self.last_key = 8;
                    true
                }
                Keycode::D => {
                    self.keys[9] = 1;
                    self.last_key = 9;
                    true
                }
                Keycode::Z => {
                    self.keys[0xA] = 1;
                    self.last_key = 0xA;
                    true
                }
                Keycode::C => {
                    self.keys[0xB] = 1;
                    self.last_key = 0xB;
                    true
                }
                Keycode::Num4 => {
                    self.keys[0xC] = 1;
                    self.last_key = 0xC;
                    true
                }
                Keycode::R => {
                    self.keys[0xD] = 1;
                    self.last_key = 0xD;
                    true
                }
                Keycode::F => {
                    self.keys[0xE] = 1;
                    self.last_key = 0xE;
                    true
                }
                Keycode::V => {
                    self.keys[0xF] = 1;
                    self.last_key = 0xF;
                    true
                }
                _ => false,
            },
            Event::KeyUp {
                keycode: Some(k), ..
            } => match k {
                Keycode::X => {
                    self.keys[0] = 0;
                    false
                }
                Keycode::Num1 => {
                    self.keys[1] = 0;
                    false
                }
                Keycode::Num2 => {
                    self.keys[2] = 0;
                    false
                }
                Keycode::Num3 => {
                    self.keys[3] = 0;
                    false
                }
                Keycode::Q => {
                    self.keys[4] = 0;
                    false
                }
                Keycode::W => {
                    self.keys[5] = 0;
                    false
                }
                Keycode::E => {
                    self.keys[6] = 0;
                    false
                }
                Keycode::A => {
                    self.keys[7] = 0;
                    false
                }
                Keycode::S => {
                    self.keys[8] = 0;
                    false
                }
                Keycode::D => {
                    self.keys[9] = 0;
                    false
                }
                Keycode::Z => {
                    self.keys[0xA] = 0;
                    false
                }
                Keycode::C => {
                    self.keys[0xB] = 0;
                    false
                }
                Keycode::Num4 => {
                    self.keys[0xC] = 0;
                    false
                }
                Keycode::R => {
                    self.keys[0xD] = 0;
                    false
                }
                Keycode::F => {
                    self.keys[0xE] = 0;
                    false
                }
                Keycode::V => {
                    self.keys[0xF] = 0;
                    false
                }
                _ => false,
            },
            _ => false,
        };
        println!("Keystate:");
        println!(
            "{} {} {} {}",
            self.keys[1], self.keys[2], self.keys[3], self.keys[0xC]
        );
        println!(
            "{} {} {} {}",
            self.keys[4], self.keys[5], self.keys[6], self.keys[0xD]
        );
        println!(
            "{} {} {} {}",
            self.keys[7], self.keys[8], self.keys[9], self.keys[0xE]
        );
        println!(
            "{} {} {} {}",
            self.keys[0xA], self.keys[0], self.keys[0xB], self.keys[0xF]
        );
        ret
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        assert!(key < 16);
        self.keys[key as usize] == 1
    }

    pub fn get_last_key(&mut self) -> u8 {
        self.last_key
    }
}
