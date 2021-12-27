mod instructions;

use instructions::Instructions;
use std::fs;

const MEM_START: u16 = 0x200;
const FONT_START: u16 = 0x050;
const FONT_SIZE: usize = 16*5;
const FONTS: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Cpu {
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
    instructions: Instructions,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu { 
            memory: [0u8; 4096],
            registers_general: [0u8; 16], 
            register_i: 0u16, 
            delay_timer: 0u8,
            sound_timer: 0u8, 
            pc: MEM_START, 
            sp: 0u8, 
            stack: [0u16; 16], 
            display: [0u32; 64 * 32],
            keypad: [0u8; 16],
            instructions: Instructions::new(),
        };

        cpu.load_fonts();
        
        cpu
    }

    fn load_fonts(&mut self) {
        for i in FONT_START as usize..(FONT_START as usize + FONT_SIZE) {
            self.memory[i] = FONTS[i - FONT_START as usize];
        }
    }
}

pub fn fetch(cpu: &mut Cpu) -> u16 {
    let cmd1:u16 = (cpu.memory[cpu.pc as usize] as u16) << 8;
    let cmd2:u16 = cpu.memory[(cpu.pc + 1) as usize] as u16;
    let cmd = cmd1 | cmd2;
    cpu.pc += 2;
    cmd
}

pub fn execute(cmd: u16, cpu: &mut Cpu) {
    instructions::execute(cmd, cpu);
}

pub fn decrease_timers(cpu: &mut Cpu) {
    if cpu.delay_timer > 0 {
        cpu.delay_timer -= 1;
    }

    if cpu.sound_timer > 0 {
        cpu.sound_timer -= 1;
    }
}

pub fn load_to_mem(file: &str, cpu: &mut Cpu) {
    let bytes = fs::read(file).expect("Something went wrong when loading the file to memory!");
    for (i, byte) in bytes.iter().enumerate() {
        cpu.memory[MEM_START as usize + i] = *byte;
    }
}
