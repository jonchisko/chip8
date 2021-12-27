use std::env;
use std::thread;
use std::time::Duration;
use chip8::cpu;
use chip8::cpu::Cpu;
use console_engine::ConsoleEngine;
use console_engine::pixel::Pixel;
use console_engine::{Color, KeyCode, };

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        panic!("Wrong number of input arguments, needs: rom file and execution speed in hz [60].");
    }
    let rom_path = &args[1];
    let hz_speed = &args[2];
    let hz_speed = hz_speed.parse::<u16>().expect("Cannot parse speed into an unsigned integer of size 16 bits!");
    let cycle_time = 1.0 / (hz_speed as f64);

    let mut engine = console_engine::ConsoleEngine::init(64, 32, hz_speed as u32).expect("Problem initing engine!");

    let mut cpu = Cpu::new();
    cpu::load_to_mem(rom_path, &mut cpu);

    loop {

        engine.wait_frame();
        engine.clear_screen();

        process_input(&engine, &mut cpu);

        let cmd = cpu::fetch(&mut cpu);
        cpu::execute(cmd, &mut cpu);
        cpu::decrease_timers(&mut cpu);
        
        draw_screen(&mut engine, &cpu);
        //draw_debug(&cpu);

        /* dumbed down way of slowing down. One other approach would be to have a timer at the beginning, check how much time has passed
        at this line and check the difference. If the difference was negative, go on, otherwise wait for the amount of time left.
         */
        thread::sleep(Duration::from_secs_f64(cycle_time)); 

        if engine.is_key_pressed(KeyCode::Enter) {
            break;
        }

        println!("Cycle complete!");
    }
}

fn draw_debug(cpu: &Cpu) {
    for (i, px) in cpu.get_display().iter().enumerate() {
        let x = i % 64;
        let y = i / 32;
        if px & 0b1 == 1 {
            print!("X");
        } else {
            print!(" ");
        }
        if y > 0 && x == 0 {
            println!("");
        }
    }

}

fn draw_screen(eng: &mut ConsoleEngine, cpu: &Cpu) {
    for (i, px) in cpu.get_display().iter().enumerate() {
        let x = i % 64;
        let y = i / 32;
        if px & 0b1 == 1 {
            eng.set_pxl(x as i32, y as i32, Pixel{bg: Color::Yellow, fg: Color::Yellow, chr: 'P'});
        } else {
            eng.set_pxl(x as i32, y as i32, Pixel{bg: Color::Black, fg: Color::Black, chr: 'W'});
        }
        
    }
    eng.draw();
}

fn process_input(eng: &ConsoleEngine, cpu: &mut Cpu) {
    let keys = ['1', '2', '3', '4',
                        'q', 'w', 'e', 'r',
                        'a', 's', 'd', 'f',
                        'y', 'x', 'c', 'v',];

    cpu.reset_keypad();

    for (i, key) in keys.iter().enumerate() {
        if eng.is_key_pressed(KeyCode::Char(*key)){
            cpu.set_keypad(i);
            break;
        }
    }
}