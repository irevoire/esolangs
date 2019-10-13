use std::io::Read;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Down,
    Up,
    Right,
}

use Direction::*;

#[derive(Copy, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new() -> Self {
        Coord { x: 0, y: 0 }
    }

    pub fn start(&self) -> bool {
        self.x == self.y && self.x == 0
    }
}

/// we are defining addition between a coordinate and a direction
impl std::ops::Add<Direction> for Coord {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        match dir {
            Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Up => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl std::ops::AddAssign<Direction> for Coord {
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}

pub struct Vm {
    grid: crate::grid::Grid,
    ptr: Coord,
    dir: Direction,
    stack: Vec<i32>,
}

impl Vm {
    pub fn new(grid: crate::grid::Grid) -> Self {
        Vm {
            grid,
            ptr: Coord::new(),
            dir: Direction::Right,
            stack: Vec::new(),
        }
    }

    pub fn cycle(&mut self) {
        let op = self.grid[self.ptr];
        match op as u8 as char {
            // flow control
            'h' => self.dir = Left,
            'j' => self.dir = Down,
            'k' => self.dir = Up,
            'l' => self.dir = Right,
            'H' => self.jump(Left),
            'J' => self.jump(Down),
            'K' => self.jump(Up),
            'L' => self.jump(Right),
            'x' => self.right_turn(),
            'X' => self.left_turn(),
            'q' => self.quit(),

            // stack
            's' => self.store(Down),
            'S' => self.store(Up),
            'd' => self.duplicate(),
            'D' => self.delete(),
            'a' => self.add(Down),
            'A' => self.add(Up),
            'r' => self.reduce(Down),
            'R' => self.reduce(Up),
            'f' => self.fetch(Down),
            'F' => self.fetch(Up),

            // I/O
            'p' => self.print(Down),
            'P' => self.print(Up),
            'g' => self.read(Down), // get
            'G' => self.read(Up),   // get
            'e' => self.eof(Down),
            'E' => self.eof(Up),

            // miscellaneous
            '#' => self.sha_bang(),

            _ => (),
        }
        self.ptr += self.dir; // move to the next instruction
    }

    /// jump (move instruction pointer) [dir] to the next cell whose value
    /// matches the value on top of the stack, and set execution direction to
    /// the specified direction
    fn jump(&mut self, dir: Direction) {
        self.dir = dir;

        if let Some(&val) = self.stack.last() {
            while self.grid[self.ptr] != val {
                self.ptr += self.dir;
            }
        }
    }

    /// if the value on top of the stack is positive, turn the execution
    /// direction 90 degrees to the right
    fn right_turn(&mut self) {
        if let Some(val) = self.stack.last() {
            if val.is_positive() {
                self.dir = match self.dir {
                    Left => Up,
                    Down => Left,
                    Up => Right,
                    Right => Down,
                }
            }
        }
    }

    /// if the value on top of the stack is negative, turn the execution
    /// direction 90 degrees to the left
    fn left_turn(&mut self) {
        if let Some(val) = self.stack.last() {
            if val.is_negative() {
                self.dir = match self.dir {
                    Left => Down,
                    Down => Right,
                    Up => Left,
                    Right => Up,
                }
            }
        }
    }

    /// end program execution
    fn quit(&mut self) {
        std::process::exit(0);
    }

    /// store (push) the value of the cell on the indicated side of the current
    /// cell to stack
    fn store(&mut self, dir: Direction) {
        self.stack.push(self.grid[self.ptr + dir] as i32);
    }

    /// duplicate top value on stack
    fn duplicate(&mut self) {
        if let Some(&val) = self.stack.last() {
            self.stack.push(val);
        }
    }

    /// delete top value off stack
    fn delete(&mut self) {
        self.stack.pop();
    }

    /// add value of the cell on the indicated direction aside from the current
    /// cell to the value on top of the stack
    fn add(&mut self, dir: Direction) {
        if let Some(val) = self.stack.last_mut() {
            *val += self.grid[self.ptr + dir];
        }
    }

    /// reduce the value on top of the stack by the value of the selected cell
    fn reduce(&mut self, dir: Direction) {
        if let Some(val) = self.stack.last_mut() {
            *val -= self.grid[self.ptr + dir];
        }
    }

    /// fetch (pop) value from top of stack and store to selected cell
    fn fetch(&mut self, dir: Direction) {
        if let Some(val) = self.stack.last() {
            self.grid[self.ptr + dir] = *val;
        }
    }

    /// send value of selected cell to stdout
    fn print(&mut self, dir: Direction) {
        print!("{}", self.grid[self.ptr + dir] as u8 as char);
    }

    /// get one byte from stdin and store in selected cell
    fn read(&mut self, dir: Direction) {
        self.grid[self.ptr + dir] =
            std::io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap() as i32;
    }

    /// insert value of system EOF in selected cell
    fn eof(&mut self, dir: Direction) {
        self.grid[self.ptr + dir] = 0; // EOF
    }

    /// behaves just like 'j', but only if its position in the code/data
    /// array is 0,0 (the left/top corner) and only if there is a '!' in
    /// the cell on its right side.
    /// The '#' instruction was introduced to make Argh! programs executable
    /// by puting "#!/path/to/argh" in the first line on systems which
    /// know this kind of magic.
    /// Anyway '#' is a valid, though undocumented, instruction and the
    /// sha-bang (#!) line becomes part of the code/data array, so any
    /// Argh!-implementation must understand '#'
    fn sha_bang(&mut self) {
        if !self.ptr.start() {
            return;
        }
        // the file start by a #
        if self.grid[self.ptr + Right] as u8 == '!' as u8 {
            self.dir = Down;
        }
    }
}
