use std::io::Read;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Down,
    Up,
    Right,
}

use Direction::*;

impl rand::distributions::Distribution<Direction> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Left,
            1 => Down,
            2 => Right,
            _ => Up,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new() -> Self {
        Coord { x: 0, y: 0 }
    }

    pub fn from(x: usize, y: usize) -> Self {
        Coord { x, y }
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
        // println!("op: {}", op as u8 as char);
        // println!("coord: {}, {}", self.ptr.x, self.ptr.y);
        // println!("stack: {:?}", self.stack);
        match op as u8 as char {
            // flow control
            '>' => self.dir = Right,
            '<' => self.dir = Left,
            '^' => self.dir = Up,
            'v' => self.dir = Down,
            '?' => self.dir = rand::random(),
            '_' => self.right_if(),
            '|' => self.down_if(),
            '"' => self.string(),
            '#' => self.bridge(),
            '@' => self.quit(),

            // stack
            ':' => self.duplicate(),
            '\\' => self.swap(),
            '$' => self.delete(),
            '+' => self.add(),
            '-' => self.sub(),
            '*' => self.mul(),
            '/' => self.div(),
            '%' => self.modulo(),
            '!' => self.not(),
            '`' => self.greater_than(),
            'p' => self.put(),
            'g' => self.get(),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.stack.push(op - '0' as i32);
            }

            // I/O
            '.' => self.print_int(),
            ',' => self.print(),
            '&' => self.ask_num(),
            '~' => self.ask_char(),

            _ => (),
        }
        self.ptr += self.dir; // move to the next instruction
    }

    /// end program execution
    fn quit(&mut self) {
        std::process::exit(0);
    }

    /// Pop a value; move right if value=0, left otherwise
    fn right_if(&mut self) {
        let val = self.stack.pop().unwrap_or(0);
        match val {
            0 => self.dir = Right,
            _ => self.dir = Left,
        }
    }

    /// Pop a value; move down if value=0, up otherwise
    fn down_if(&mut self) {
        let val = self.stack.pop().unwrap_or(0);
        match val {
            0 => self.dir = Down,
            _ => self.dir = Up,
        }
    }

    /// Start string mode: push each character's ASCII value all the way up to the next "
    fn string(&mut self) {
        self.ptr += self.dir;
        while self.grid[self.ptr] != '"' as i32 {
            self.stack.push(self.grid[self.ptr]);
            self.ptr += self.dir;
        }
    }

    /// Bridge: Skip next cell
    fn bridge(&mut self) {
        self.ptr += self.dir;
    }

    /// duplicate top value on stack
    fn duplicate(&mut self) {
        let val = self.stack.pop().unwrap_or(0);
        self.stack.push(val);
        self.stack.push(val);
    }

    /// Swap two values on top of the stack
    fn swap(&mut self) {
        let v1 = self.stack.pop().unwrap_or(0);
        let v2 = self.stack.pop().unwrap_or(0);
        self.stack.push(v2);
        self.stack.push(v1);
    }

    /// delete top value off stack
    fn delete(&mut self) {
        self.stack.pop();
    }

    /// Addition: Pop a and b, then push a + b
    fn add(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);
        self.stack.push(a + b);
    }

    /// Subtraction: Pop a and b, then push b - a
    fn sub(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);
        self.stack.push(b - a);
    }

    /// Multiplication: Pop a and b, then push a * b
    fn mul(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);
        self.stack.push(b * a);
    }

    /// Integer division: Pop a and b, then push b / a, rounded towards 0.
    fn div(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);
        self.stack.push(b / a);
    }

    /// Modulo: Pop a and b, then push the remainder of the integer division of b / a.
    fn modulo(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);
        self.stack.push(b % a);
    }

    /// Logical NOT: Pop a value. If the value is zero, push 1; otherwise, push zero.
    fn not(&mut self) {
        let val = self.stack.pop().unwrap_or(0);
        match val {
            0 => self.stack.push(1),
            _ => self.stack.push(0),
        }
    }

    /// Greater than: Pop a and b, then push 1 if b > a, otherwise zero.
    fn greater_than(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);

        if b > a {
            self.stack.push(1);
        } else {
            self.stack.push(0);
        }
    }

    /// A "put" call (a way to store a value for later use). Pop y, x, and v,
    /// then change the character at (x,y) in the program to the character with
    /// ASCII value v
    fn put(&mut self) {
        if self.stack.len() < 3 {
            println!("Not enough element in the stack");
            std::process::exit(0);
        }
        let x = self.stack.pop().unwrap();
        let y = self.stack.pop().unwrap();
        let v = self.stack.pop().unwrap();

        self.grid[Coord::from(x as usize, y as usize)] = v;
    }

    /// A "get" call (a way to retrieve data in storage). Pop y and x, then
    /// push ASCII value of the character at that position in the program
    fn get(&mut self) {
        if self.stack.len() < 2 {
            println!("Not enough element in the stack");
            std::process::exit(0);
        }
        let x = self.stack.pop().unwrap();
        let y = self.stack.pop().unwrap();

        self.stack
            .push(self.grid[Coord::from(x as usize, y as usize)]);
    }

    /// Pop value and output as an integer followed by a space
    fn print_int(&mut self) {
        if self.stack.len() < 1 {
            println!("Not enough element in the stack");
            std::process::exit(0);
        }
        print!("{} ", self.stack.pop().unwrap());
    }

    /// Pop value and output as ASCII character
    fn print(&mut self) {
        if self.stack.len() < 1 {
            println!("Not enough element in the stack");
            std::process::exit(0);
        }
        print!("{}", self.stack.pop().unwrap() as u8 as char);
    }

    /// Ask user for a number and push it
    fn ask_num(&mut self) {
        println!("Input a number: ");
        let val = std::io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap() as i32;
        self.stack.push(val);
    }

    /// Ask user for a number and push it
    fn ask_char(&mut self) {
        let val = std::io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap() as i32;
        self.stack.push(val);
    }
}
