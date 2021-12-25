use super::Cpu;

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
    let x = cmd & 0x0F00 >> 8;
    let y = cmd & 0x00F0 >> 4;
    let new_cmd = (8 << 12) | (y << 8) | (x << 4) | (7);
    sub_8xy5(new_cmd, cpu);
}

fn shiftleft_8xyE(cmd: u16, cpu: &mut Cpu) {
    op_8(cmd, cpu, |x, _y, cpu| {
        cpu.registers_general[0xF as usize] = if cpu.registers_general[x as usize] & (0b1 << 7) == 1 { 1 } else { 0 };
        cpu.registers_general[x as usize] <<= 1;
    })
}