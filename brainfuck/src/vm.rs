use std::io::{Read, Write};

pub struct Vm<'a> {
    instructions: &'a [char],
    instruction_pointer: usize,
    tape: crate::tape::Tape,
    memory_pointer: i32,
}

impl<'a> Vm<'a> {
    pub fn new(instructions: &'a [char]) -> Self {
        Vm {
            instructions: instructions,
            instruction_pointer: 0,
            tape: crate::tape::Tape::new(),
            memory_pointer: 0,
        }
    }

    pub fn cycle(&mut self) {
        let op = self.instructions[self.instruction_pointer];
        match op {
            '>' => self.right(),
            '<' => self.left(),
            '+' => self.increment(),
            '-' => self.decrement(),
            '.' => self.output(),
            ',' => self.input(),
            '[' => self.jump_right(),
            ']' => self.jump_left(),
            _ => self.instruction_pointer += 1, // comment
        }
    }

    pub fn finished(&self) -> bool {
        self.instruction_pointer >= self.instructions.len()
    }

    /// Move the memory pointer to the right
    fn right(&mut self) {
        self.memory_pointer += 1;
        self.instruction_pointer += 1;
    }

    /// Move the memory pointer to the left
    fn left(&mut self) {
        self.memory_pointer -= 1;
        self.instruction_pointer += 1;
    }

    /// Increment the memory cell under the memory pointer
    fn increment(&mut self) {
        let v = &mut self.tape[self.memory_pointer];
        *v = v.wrapping_add(1);
        self.instruction_pointer += 1;
    }

    /// Decrement the memory cell under the memory pointer
    fn decrement(&mut self) {
        let v = &mut self.tape[self.memory_pointer];
        *v = v.wrapping_sub(1);
        self.instruction_pointer += 1;
    }

    /// Output the character signified by the cell at the memory pointer
    fn output(&mut self) {
        write!(
            std::io::stdout(),
            "{}",
            self.tape[self.memory_pointer] as u8 as char // because I can’t cast i8 to char
        )
        .unwrap();
        self.instruction_pointer += 1;
    }

    /// Input a character and store it in the cell at the memory pointer
    /// **Here we decide to send 0 to the brainfuck program when we get EOF**
    fn input(&mut self) {
        self.tape[self.memory_pointer] =
            std::io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap() as i8;
        self.instruction_pointer += 1;
    }

    /// Jump past the matching ] if the cell under the memory pointer is 0
    fn jump_right(&mut self) {
        if self.tape[self.memory_pointer] != 0 {
            self.instruction_pointer += 1;
            return;
        }
        let mut matching = 1;
        loop {
            self.instruction_pointer += 1;
            if self.instructions[self.instruction_pointer] == '[' {
                matching += 1;
            }
            if self.instructions[self.instruction_pointer] == ']' {
                matching -= 1;
                if matching == 0 {
                    break;
                }
            }
        }
        self.instruction_pointer += 1;
    }

    /// Jump back to the matching [ if the cell under the memory pointer is nonzero
    fn jump_left(&mut self) {
        if self.tape[self.memory_pointer] == 0 {
            self.instruction_pointer += 1;
            return;
        }
        let mut matching = 1;
        loop {
            self.instruction_pointer -= 1;
            if self.instructions[self.instruction_pointer] == ']' {
                matching += 1;
            }
            if self.instructions[self.instruction_pointer] == '[' {
                matching -= 1;
                if matching == 0 {
                    break;
                }
            }
        }
    }
}
