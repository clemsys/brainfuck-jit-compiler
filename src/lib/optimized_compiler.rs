use super::{optimized_command::OptimizedCommand, optimized_program::OptimizedProgram};
use memmap::MmapMut;

impl From<&OptimizedCommand> for Vec<u8> {
    /// Returns the machine code for the given command.
    fn from(command: &OptimizedCommand) -> Self {
        // %r13 is used as the data pointer
        match command {
            OptimizedCommand::Move(n) => {
                let mut v = vec![0x49, 0x81, 0xc5];
                v.extend(&n.to_le_bytes());
                v
            } // incr %r13
            OptimizedCommand::Add(n) => vec![0x41, 0x80, 0x45, 0x00, *n], // addb $0x1,0x0(%r13)
            OptimizedCommand::SetToZero => vec![0x41, 0xc6, 0x45, 0x00, 0x00], // movb $0x0,0x0(%r13)
            OptimizedCommand::Print => vec![
                0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, 0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00,
                0x4c, 0x89, 0xee, 0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x05,
            ], // mov $0x1,%rax  mov $0x1,%rdi  mov %r13,%rsi  mov $0x1,%rdx  syscall
            OptimizedCommand::Read => vec![
                0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x48, 0xC7, 0xC7, 0x00, 0x00, 0x00, 0x00,
                0x4C, 0x89, 0xEE, 0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00, 0x0F, 0x05,
            ],
            OptimizedCommand::JumpForward => vec![
                0x41, 0x80, 0x7d, 0x00, 0x00, 0x0F, 0x84, 0x00, 0x00, 0x00, 0x00,
            ], // cmpb $0, 0(%r13)  je ... (address to be filled in by compiler)
            OptimizedCommand::JumpBackwards => vec![
                0x41, 0x80, 0x7d, 0x00, 0x00, 0x0F, 0x85, 0x00, 0x00, 0x00, 0x00,
            ], // cmpb $0, 0(%r13)  jne ... (address to be filled in by compiler)
        }
    }
}

pub struct OptimizedCompiler {
    program: OptimizedProgram,
    machine_code: Vec<u8>,
    data: Vec<u8>,
}

impl OptimizedCompiler {
    /// # Arguments
    ///
    /// * `program` - The brainfuck program to be compiled and run
    /// * `data_size` - The size of the data array used by the brainfuck program.
    pub fn new(program: OptimizedProgram, data_size: usize) -> Self {
        Self {
            program,
            machine_code: Vec::new(),
            data: vec![0u8; data_size],
        }
    }

    fn compile(&mut self) {
        let data_ptr = self.data.as_ptr() as u64;
        self.machine_code.extend_from_slice(&[0x49, 0xbd]);
        self.machine_code.extend_from_slice(&data_ptr.to_le_bytes());

        let mut left_brackets_stack = Vec::new();

        for command in self.program.iter() {
            self.machine_code.append(&mut command.into());
            match command {
                OptimizedCommand::JumpForward => left_brackets_stack.push(self.machine_code.len()),
                OptimizedCommand::JumpBackwards => {
                    // address right after the matching left bracket
                    let left_addr = left_brackets_stack.pop().unwrap();
                    // address right after the matching right bracket
                    let right_addr = self.machine_code.len();
                    let rel_forward = i32::try_from(right_addr - left_addr).unwrap();
                    let rel_forward_bytes = rel_forward.to_le_bytes();
                    let rel_backwards_bytes = (-rel_forward).to_le_bytes();
                    self.machine_code
                        .splice((left_addr - 4)..left_addr, rel_forward_bytes);
                    self.machine_code
                        .splice((right_addr - 4)..right_addr, rel_backwards_bytes);
                }
                _ => (),
            }
        }
        assert!(left_brackets_stack.is_empty());

        self.machine_code.push(0xc3); // ret
    }

    pub fn run(&mut self) {
        self.compile();

        let mut m = MmapMut::map_anon(self.machine_code.len()).unwrap();
        m.clone_from_slice(&self.machine_code);
        let m = m.make_exec().unwrap();
        let func_ptr = m.as_ptr();
        unsafe {
            let func: extern "C" fn() = std::mem::transmute(func_ptr);
            func();
        }
    }
}
