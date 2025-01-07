pub const START_ADDR: u16 = 0x200;
pub const RAM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;

pub struct Ram {
    mem: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],
    sp: u8,
}
impl Ram {
    // init
    pub fn new() -> Ram {
        Ram {
            mem: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    // methods
    fn read(&mut self, addr: u16, val: u8) {
        // read
    }
    fn write(&mut self, addr: u16, val: u8) {
        // write
    }

    fn s_push(&mut self, val: u16) {
        // add to top of stack and return new sp
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    fn s_pop(&mut self) {
        // move back sp and return top of stack
        self.sp -= 1;
        self.stack[self.sp as usize];
    }
}
