use crate::vm::jit;

pub struct VM {

}

impl VM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let mut exemem = jit::ExecMemory::new(10);

        exemem[0] = 0x48;  // mov RAX, 0x3
        exemem[1] = 0xc7;
        exemem[2] = 0xc0;
        exemem[3] = 0x03;
        exemem[4] = 0x00;
        exemem[5] = 0x00;
        exemem[6] = 0x00;

        println!("{}", exemem.run());
    }
}