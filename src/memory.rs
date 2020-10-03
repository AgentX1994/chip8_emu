#[derive(Copy, Clone, Debug)]
pub struct OutOfBoundsError;

pub type Result<T> = std::result::Result<T, OutOfBoundsError>;

pub const MEMORY_SIZE: u16 = 4096;

pub struct Memory {
    memory: [u8; MEMORY_SIZE as usize],
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: [0; MEMORY_SIZE as usize],
        }
    }
}

impl Memory {
    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE as usize];
    }

    pub fn write_u8(&mut self, address: u16, data: u8) -> Result<()> {
        if address > MEMORY_SIZE {
            return Err(OutOfBoundsError);
        }
        self.memory[address as usize] = data;
        Ok(())
    }

    pub fn write_data(&mut self, start_address: u16, data: &[u8]) -> Result<()> {
        if start_address > MEMORY_SIZE {
            return Err(OutOfBoundsError);
        }
        for (index, &byte) in data.iter().enumerate() {
            self.write_u8(start_address + index as u16, byte)?;
        }
        Ok(())
    }

    pub fn get_u8(&self, address: u16) -> Result<u8> {
        if address > MEMORY_SIZE {
            return Err(OutOfBoundsError);
        }
        Ok(self.memory[address as usize])
    }

    pub fn get_u16(&self, address: u16) -> Result<u16> {
        if address > MEMORY_SIZE || address + 1 > MEMORY_SIZE {
            return Err(OutOfBoundsError);
        }
        let high_byte = self.memory[address as usize] as u16;
        let low_byte = self.memory[(address + 1) as usize] as u16;
        Ok(high_byte << 8 | low_byte)
    }

    pub fn get_data(&self, address: u16, size: u16) -> Result<&[u8]> {
        if address > MEMORY_SIZE - size {
            return Err(OutOfBoundsError);
        }
        Ok(&self.memory[address as usize..(address + size) as usize])
    }
}
