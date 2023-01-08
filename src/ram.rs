pub struct MemoryRam<'a>{
  memory_ram: &'a mut [u8]
}

impl<'a> MemoryRam<'a>{
  pub fn new(memory_ram: &'a mut [u8]) -> MemoryRam{
    MemoryRam{memory_ram}
  }

  pub fn read(&self, address: usize) -> u8{
    self.memory_ram[address]
  }
  pub fn read_addresses(&self, address: usize, bytes_number: usize) -> &[u8]{
    &self.memory_ram[address..(address + bytes_number)]
  }

  pub fn write(&mut self, address: usize, byte: u8) {
    self.memory_ram[address] = byte;
  }
  pub fn write_addresses(&mut self, address: usize, bytes: &[u8]){
    for (i, byte) in bytes.iter().enumerate(){
      self.memory_ram[address + i] = *byte;
    }
  }
}

#[cfg(test)]
mod tests{
  use super::MemoryRam;
  const RAM_SIZE: usize = 512;

  #[test]
  fn read_and_write(){
    let mut memory_array = [0; RAM_SIZE];
    let mut ram_memory = MemoryRam::new(&mut memory_array);
    
    ram_memory.write(0x0, 0x10);
    ram_memory.write(0xA, 0x90);
    ram_memory.write(0x1FF, 0xFF);

    assert_eq!(ram_memory.read(0x0), 0x10);
    assert_eq!(ram_memory.read(0xA), 0x90);
    assert_eq!(ram_memory.read(0x1FF), 0xFF);
  }

  #[test]
  fn read_and_write_addresses(){
    let mut memory_array = [0; RAM_SIZE];
    let mut ram_memory = MemoryRam::new(&mut memory_array);

    ram_memory.write_addresses(0x0, &[0,2,1]);
    ram_memory.write_addresses(0x4, &[0,123,255,0]);
    ram_memory.write_addresses(0xF, &[3]);

    assert_eq!(ram_memory.read_addresses(0x0,3), &[0,2,1]);
    assert_eq!(ram_memory.read_addresses(0x4, 4), &[0,123,255,0]);
    assert_eq!(ram_memory.read_addresses(0xF, 1), &[3]);
  }
}