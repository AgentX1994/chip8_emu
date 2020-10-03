pub const WIDTH: u16 = 64;
pub const HEIGHT: u16 = 32;
pub const SIZE: u16 = WIDTH * HEIGHT;

pub struct Screen {
    screen: [u8; SIZE as usize],
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            screen: [0; SIZE as usize],
        }
    }
}

impl Screen {
    pub fn reset(&mut self) {
        self.screen = [0; SIZE as usize];
    }

    fn index(x: u16, y: u16) -> usize {
        let index = x + (y * WIDTH);
        assert!(index < SIZE);
        index as usize
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> u8 {
        self.screen[Self::index(x, y)]
    }

    pub fn toggle_pixel(&mut self, x: u16, y: u16) -> bool {
        let ret = self.get_pixel(x, y) == 1;
        self.screen[Self::index(x, y)] ^= 1;
        ret
    }

    pub fn draw_sprite(&mut self, x: u16, y: u16, sprite: &[u8]) -> bool {
        let mut ret = false;
        let x = x % WIDTH;
        let y = y % HEIGHT;
        for (line, &pixel) in sprite.iter().enumerate() {
            for bit in 0..8u8 {
                if pixel & (0x80 >> bit) != 0 {
                    if x + (bit as u16) < WIDTH && y + (line as u16) < HEIGHT {
                        if self.toggle_pixel(x + bit as u16, y + line as u16) {
                            ret = true;
                        }
                    }
                }
            }
        }

        ret
    }

    pub fn get_pixel_data(&self) -> &[u8] {
        &self.screen[..]
    }
}
