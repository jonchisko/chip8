use super::Cpu;
use rand::{self, Rng};

struct Instructions {
    collection_0: [fn(cmd: u16, cpu: &mut Cpu) -> (); 0xF + 1],
    collection_1: [fn(cmd: u16, cpu: &mut Cpu) -> (); 0xE + 1],
    collection_2: [fn(cmd: u16, cpu: &mut Cpu) -> (); 0xE + 1],
    collection_3: [fn(cmd: u16, cpu: &mut Cpu) -> (); 0xE + 1],
    collection_4: [fn(cmd: u16, cpu: &mut Cpu) -> (); 0x65 + 1],
}

impl Instructions {
    pub fn new() -> Instructions {
        let mut new_instructions = Instructions {
            collection_0: [noop; 0xF + 1],
            collection_1: [noop; 0xE + 1],
            collection_2: [noop; 0xE + 1],
            collection_3: [noop; 0xE + 1],
            collection_4: [noop; 0x65 + 1],
        };

        new_instructions.set_instructions();

        new_instructions
    }

    pub fn execute(&self, cmd: u16, cpu: &mut Cpu) {
        
    }

    fn set_instructions(&mut self) {

    }
}

fn get_x(cmd: u16) -> u16 {
    (cmd & 0x0F00) >> 8
}

fn get_y(cmd: u16) -> u16 {
    (cmd & 0x00F0) >> 4
}

fn noop(_cmd: u16, _cpu: &mut Cpu) {

}

fn clear_00E0(_cmd: u16, cpu: &mut Cpu) {
    for i in 0..cpu.display.len() {
        cpu.display[i] = 0;
    }
}

fn return_00EE(_cmd: u16, cpu: &mut Cpu) {
    cpu.sp -= 1;
    cpu.pc = cpu.stack[cpu.sp as usize];
}

fn jump_1nnn(cmd: u16, cpu: &mut Cpu) {
    let jump_location = cmd & 0x0FFF;
    cpu.pc = jump_location;
}

fn call_2nnn(cmd: u16, cpu: &mut Cpu) {
    let call_subr_location = cmd & 0x0FFF;
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.sp += 1;
    cpu.pc = call_subr_location;
}

fn skip_3xkk(cmd: u16, cpu: &mut Cpu) {
    let reg_x = get_x(cmd);
    let constant = (cmd & 0x00FF) as u8;

    if cpu.registers_general[reg_x as usize] == constant {
        cpu.pc += 2;
    }
}

fn skip_4xkk(cmd: u16, cpu: &mut Cpu) {
    let reg_x = get_x(cmd);
    let constant = (cmd & 0x00FF) as u8;

    if cpu.registers_general[reg_x as usize] != constant {
        cpu.pc += 2;
    }
}

fn skip_5xy0(cmd: u16, cpu: &mut Cpu) {
    let reg_x = get_x(cmd);
    let reg_y = get_y(cmd);

    if cpu.registers_general[reg_x as usize] == cpu.registers_general[reg_y as usize] {
        cpu.pc += 2;
    }
}

fn skip_9xy0(cmd: u16, cpu: &mut Cpu) {
    let reg_x = get_x(cmd);
    let reg_y = get_y(cmd);

    if cpu.registers_general[reg_x as usize] != cpu.registers_general[reg_y as usize] {
        cpu.pc += 2;
    }
}

fn set_6xkk(cmd: u16, cpu: &mut Cpu) {
    let reg_x = get_x(cmd);
    let constant = cmd & 0x00FF;
    cpu.registers_general[reg_x as usize] = constant as u8; 
}

fn add_7xkk(cmd: u16, cpu: &mut Cpu) {
    let reg_x = get_x(cmd);
    let constant = cmd & 0x00FF;
    cpu.registers_general[reg_x as usize] += constant as u8;
}

// <------------------------------------------------->

fn op_8(cmd: u16, cpu: &mut Cpu, fun: fn(u16, u16, &mut Cpu)) {
    let reg_x = get_x(cmd);
    let reg_y = get_y(cmd);
    fun(reg_x, reg_y, cpu);
}

fn set_8xy0(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, y, cpu|{
        cpu.registers_general[x as usize] = cpu.registers_general[y as usize];
    });
}

fn or_8xy1(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, y, cpu|{
        cpu.registers_general[x as usize] |= cpu.registers_general[y as usize];
    });
}

fn and_8xy2(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, y, cpu|{
        cpu.registers_general[x as usize] &= cpu.registers_general[y as usize];
    });
}

fn xor_8xy3(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, y, cpu|{
        cpu.registers_general[x as usize] ^= cpu.registers_general[y as usize];
    });
}

fn add_8xy4(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, y, cpu|{
        let sum = cpu.registers_general[x as usize] as u16 + cpu.registers_general[y as usize] as u16;
        if 255 - cpu.registers_general[y as usize] < cpu.registers_general[x as usize] {
            cpu.registers_general[0xF as usize] = 1;
        } else {
            cpu.registers_general[0xF as usize] = 0;
        }
        cpu.registers_general[x as usize] = (sum & 0xFF) as u8;
    });
}
 
fn sub_8xy5(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, y, cpu| {
        let sub;
        if cpu.registers_general[x as usize] > cpu.registers_general[y as usize] {
            cpu.registers_general[0xF as usize] = 1;
        } else {
            cpu.registers_general[0xF as usize] = 0;
        }

        if cpu.registers_general[x as usize] < cpu.registers_general[y as usize] {
            sub = (255i16 + (cpu.registers_general[x as usize] as i16 - cpu.registers_general[y as usize] as i16)) as u8;
        } else {
            sub = cpu.registers_general[x as usize] - cpu.registers_general[y as usize];
        }
        cpu.registers_general[x as usize] = sub;
    })
}

fn shiftright_8xy6(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, _y, cpu| {
        cpu.registers_general[0xF as usize] = if cpu.registers_general[x as usize] & 0b1 == 1 { 1 } else { 0 };
        cpu.registers_general[x as usize] >>= 1;
    })
}

fn sub_8xy7(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    let y = get_y(cmd);
    let new_cmd = (8 << 12) | (y << 8) | (x << 4) | (7);
    sub_8xy5(new_cmd, cpu);
}

fn shiftleft_8xyE(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, _y, cpu| {
        cpu.registers_general[0xF as usize] = if cpu.registers_general[x as usize] & (0b1 << 7) == 1 { 1 } else { 0 };
        cpu.registers_general[x as usize] <<= 1;
    })
}

fn set_i_annn(cmd: u16, cpu: &mut Cpu) {
    let addr = cmd & 0x0FFF;
    cpu.register_i = addr;
}

fn jump_bnnn(cmd: u16, cpu: &mut Cpu) {
    let addr = cmd & 0x0FFF;
    cpu.pc = cpu.registers_general[0] as u16 + addr;
}

fn rand_cxkk(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    // it would be better to have a struct and some interface to call rand @SirŠirŠkuta
    let rnd = rand::thread_rng().gen_range(0..=255) as u8; 
    let constant = (cmd & 0x00FF) as u8;
    cpu.registers_general[x as usize] = rnd & constant;
}

fn draw_dxyn(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    let y = get_y(cmd);
    let size = cmd & 0x000F;

    let sprite_data = &cpu.memory[(cpu.register_i as usize)..(cpu.register_i as usize + size as usize)];

    let mut row = y;
    let mut col = x;

    for i in 0..size {
        let color_byte = sprite_data[i as usize];
        row = (row + 1) % 32;
        for j in 0..8 {
            col = (col + 1) % 64;
            let lin_indx = row * 64 + col;

            if (color_byte >> (7 - j)) & 0b1 == 1 {
                if cpu.display[lin_indx as usize] == 0xFFFFFFFF {
                    // collision
                    cpu.registers_general[0xF] = 1;
                }
                cpu.display[lin_indx as usize] ^= 0xFFFFFFFF;
            }
        }
    }
}

fn skip_if_key_Ex9E(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    let val = cpu.registers_general[x as usize];

    if cpu.keypad[val as usize] == 0xFF {
        cpu.pc += 2;
    }
}

fn skip_ifn_key_ExA1(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    let val = cpu.registers_general[x as usize];

    if cpu.keypad[val as usize] != 0xFF {
        cpu.pc += 2;
    }
}

fn set_Fx07(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    cpu.registers_general[x as usize]= cpu.delay_timer;
}

fn store_key_Fx0A(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    let mut found = false;
    for i in 0..16 {
        if cpu.keypad[i] == 0xFF {
            cpu.registers_general[x as usize] = i as u8;
            found = true;
            break;
        }
    }

    if !found { // wait for key, thus reduce pc
        cpu.pc -= 2;
    }
}

fn set_Fx15(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    cpu.delay_timer = cpu.registers_general[x as usize];
}

fn set_Fx18(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    cpu.sound_timer = cpu.registers_general[x as usize];
}

fn set_Fx1E(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    cpu.register_i += cpu.registers_general[x as usize] as u16;
}

fn get_digit_Fx29(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    let digit = cpu.registers_general[x as usize];
    cpu.register_i = super::FONT_START + (5 * digit as u16);
}

fn decimal_Fx33(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd);
    let mut num = cpu.registers_general[x as usize];
    cpu.memory[cpu.register_i as usize] = num / 100;
    num = num % 100;
    cpu.memory[cpu.register_i as usize + 1] = num / 10;
    num = num % 10;
    cpu.memory[cpu.register_i as usize + 2] = num;

}

fn store_Fx55(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd) as usize;
    for i in 0..=x {
        cpu.memory[cpu.register_i as usize + i] = cpu.registers_general[i];
    }
}

fn read_Fx55(cmd: u16, cpu: &mut Cpu) {
    let x = get_x(cmd) as usize;
    for i in 0..=x {
        cpu.registers_general[i] = cpu.memory[cpu.register_i as usize + i];
    }
}
