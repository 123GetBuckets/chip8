const START_ADDR: u16 = 0x200;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REG: usize = 16;
const NUM_KEYS: usize = 16;
pub const L_SCREEN: usize = 32;
pub const W_SCREEN: usize = 64;

/* Emu
 *  pc = program counter: points to current instruction
 *  ram
 *  stack
 *  sp: stack pointer: points to head of stack
 *  vreg = v-register array
 *  ireg = i-register: indexes ram for read and write
 *  dt = delay timer: counts down for event on 0
 *  st = sound timer: counts down for noise on 0
 *  keys = array of true/false
 *  screen = b&w image: flattened array of true/false
 */
pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],
    sp: u16,
    vreg: [u8; NUM_REG],
    ireg: u16,
    dt: u8,
    st: u8,
    keys: [bool; NUM_KEYS],
    screen: [bool; L_SCREEN * W_SCREEN],
}
impl Emu {
    pub fn init() -> Self {
        // return
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            sp: 0,
            vreg: [0; NUM_REG],
            ireg: 0,
            dt: 0,
            st: 0,
            keys: [false; NUM_KEYS],
            screen: [false; L_SCREEN * W_SCREEN],
        }
    }
}
