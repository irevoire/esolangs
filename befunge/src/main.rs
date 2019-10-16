mod grid;
mod vm;

use std::fs::File;
use std::io::{stdin, BufReader};

fn main() {
    let grid;
    let mut args = std::env::args();
    let arg = args.nth(1);
    if let Some(filename) = arg {
        let file = File::open(filename);
        if let Err(e) = file {
            println!("The file specified is invalid: {}", e);
            return;
        }
        let reader = BufReader::new(file.unwrap()); // we can unwrap safely
        grid = grid::Grid::from(reader);
    } else {
        println!("Expect the source code on stdin");
        let reader = stdin();
        let reader = reader.lock();
        grid = grid::Grid::from(reader);
    }

    let mut vm = vm::Vm::new(grid);

    loop {
        vm.cycle();
    }
}
