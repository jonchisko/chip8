

mod instructions;

const FONT_START: u16 = 0x050u16;


struct Cpu {
    memory: [u8; 4096],
    registers_general: [u8; 16],
    register_i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    display: [u32; 64 * 32],
    keypad: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { 
            memory: [0u8; 4096],
            registers_general: [0u8; 16], 
            register_i: 0u16, 
            delay_timer: 0u8,
            sound_timer: 0u8, 
            pc: 0x200, 
            sp: 0u8, 
            stack: [0u16; 16], 
            display: [0u32; 64 * 32],
            keypad: [0u8; 16],
        }
    }
}