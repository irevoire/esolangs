mod tape;
mod vm;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut args = std::env::args();
    let exe = args.next().unwrap();
    let file = match args.next() {
        Some(f) => f,
        None => {
            println!("usage:\n\t{} [filename]", exe);
            return;
        }
    };

    let mut file = File::open(file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let buffer = buffer.iter().map(|&a| a as char).collect::<Vec<char>>();
    let mut vm = vm::Vm::new(&buffer);
    loop {
        vm.cycle();
    }
}
