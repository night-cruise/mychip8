mod mem;
mod cpu;
mod op;
mod chip8;

use chip8::CHIP8;

fn main() {
    if let Err(err) = CHIP8::run() {
        println!("Application Error: {}", err);
        if let Some(source) = err.source() {
            println!("Source: {}", source);
        }
        std::process::exit(1);
    }
}