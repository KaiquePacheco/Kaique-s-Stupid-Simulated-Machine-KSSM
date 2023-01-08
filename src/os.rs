use std::fs::read;

use crate::ram::MemoryRam;

pub struct OperationSystem<'a>{
  pub memory: &'a mut MemoryRam<'a>,
  processes: Vec<Process>,
  current_pid: usize
}
impl<'a> OperationSystem<'a>{
  pub fn new(initial_process_path: String, memory: &'a mut MemoryRam<'a>) -> OperationSystem<'a>{
    OperationSystem {
      memory, 
      processes: vec![Process::new(0,initial_process_path)], 
      current_pid: 0 }
  }
  pub fn init_process(&mut self, mut process: Process) -> &mut OperationSystem<'a>{
    self.current_pid = self.processes.len();
    self.processes.push(process);

    self
  }
}

pub struct Process{
  initial_address: usize,
  image_size: usize
}
impl Process{
  pub fn new(initial_address: usize, script_path: String) -> Process{
    let buffer = read(script_path)
      .unwrap();
    
    let image_size = usize::from_le_bytes(
      [buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]]
    );

    Process {initial_address, image_size}
  }
}

#[cfg(test)]
mod tests{
  use std::fs::write;

  use crate::ram::MemoryRam;

  use super::OperationSystem;
  use super::Process;

  #[test]
  fn new_process(){
    write("testing/test.ksil", usize::to_le_bytes(0xFF + 1)).unwrap();

    let process = Process::new(0, String::from("testing/test.ksil"));
    assert_eq!(process.image_size, 256);
    assert_eq!(process.initial_address, 0);
  }

  #[test]
  fn new_os(){
    write("testing/test.ksil", usize::to_le_bytes(0xFF + 1)).unwrap();

    let mut memory_array = [0_u8; 256];
    let mut memory = MemoryRam::new(&mut memory_array);
    
    let os = OperationSystem::new(String::from("testing/test.ksil"), &mut memory);
    
    assert_eq!(os.current_pid, 0);
    assert_eq!(os.memory.read(0), 0);
  }
}