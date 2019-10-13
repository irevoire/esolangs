mod grid;
mod vm;

use std::io;

fn main() {
    let reader = io::stdin();
    let reader = reader.lock();
    let grid = grid::Grid::from(reader);
    let mut vm = vm::Vm::new(grid);

    loop {
        vm.cycle();
    }
}
