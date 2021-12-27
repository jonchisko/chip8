use std::env;
use std::thread;
use std::time::Duration;
use chip8::cpu;
use chip8::cpu::Cpu;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        panic!("Wrong number of input arguments, needs: rom file and execution speed in hz [60].");
    }
    let rom_path = &args[1];
    let hz_speed = &args[2];
    let hz_speed = hz_speed.parse::<u16>().expect("Cannot parse speed into an unsigned integer of size 16 bits!");
    let cycle_time = 1.0 / (hz_speed as f64);

    let mut cpu = Cpu::new();
    cpu::load_to_mem(rom_path, &mut cpu);

    loop {
        let cmd = cpu::fetch(&mut cpu);
        cpu::execute(cmd, &mut cpu);
        cpu::decrease_timers(&mut cpu);
        
        /* dumbed down way of slowing down. One other approach would be to have a timer at the beginning, check how much time has passed
        at this line and check the difference. If the difference was negative, go on, otherwise wait for the amount of time left.
         */
        thread::sleep(Duration::from_secs_f64(cycle_time)); 
    }
}
