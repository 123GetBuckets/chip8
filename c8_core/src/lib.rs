mod ram;
pub const L_SCREEN: usize = 32;
pub const W_SCREEN: usize = 64;
const NUM_REG: usize = 16;
const NUM_KEYS: usize = 16;

/* Emu
 *  pc: program counter = points to current instruction
 *  ram: Ram
 *      mem: main memory
 *      stack = stores program location
 *      sp: stack pointer
 *  vreg: v-register array
 *  ireg: i-register = indexes ram for read and write
 *  dt: delay timer = counts down for event on 0
 *  st: sound timer = counts down for noise on 0
 *  keys: array of true/false
 *  screen: b&w image = flattened array of true/false
 */
pub struct Emu {
    pc: u16,
    ram: ram::Ram,
    vreg: [u8; NUM_REG],
    ireg: u16,
    dt: u8,
    st: u8,
    keys: [bool; NUM_KEYS],
    screen: [bool; L_SCREEN * W_SCREEN],
}
impl Emu {
    // init
    pub fn new() -> Self {
        Emu {
            pc: ram::START_ADDR,
            ram: ram::Ram::new(),
            vreg: [0; NUM_REG],
            ireg: 0,
            dt: 0,
            st: 0,
            keys: [false; NUM_KEYS],
            screen: [false; L_SCREEN * W_SCREEN],
        }
    }

    // methods
}
