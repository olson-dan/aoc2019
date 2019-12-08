enum Arg {
    Immediate(isize),
    Parameter(usize),
}

impl Arg {
    fn new(instruction: isize, arg: isize, position: usize) -> Arg {
        let divisor = match position {
            1 => 100,
            2 => 1000,
            3 => 10000,
            _ => unimplemented!(),
        };
        match (instruction / divisor) % 10 {
            0 => Arg::Immediate(arg),
            1 => Arg::Parameter(arg as usize),
            _ => unimplemented!(),
        }
    }
    fn resolve_in(&self, memory: &[isize]) -> isize {
        match *self {
            Arg::Immediate(x) => x,
            Arg::Parameter(x) => memory[x],
        }
    }
    fn resolve_out(&self) -> usize {
        match *self {
            Arg::Immediate(x) => unimplemented!(),
            Arg::Parameter(x) => x,
        }
    }
}

enum Instruction {
    Halt,
    Add(Arg, Arg, Arg),
    Mul(Arg, Arg, Arg),
    Input(Arg),
    Output(Arg),
    JNZ(Arg, Arg),
    JZ(Arg, Arg),
    LessThan(Arg, Arg, Arg),
    Equals(Arg, Arg, Arg),
}

impl Instruction {
    fn arg_count(&self) -> usize {
        match self {
            Instruction::Halt => 0,
            Instruction::Add(_, _, _) => 3,
            Instruction::Mul(_, _, _) => 3,
            Instruction::Input(_) => 1,
            Instruction::Output(_) => 1,
            Instruction::JNZ(_, _) => 2,
            Instruction::JZ(_, _) => 2,
            Instruction::LessThan(_, _, _) => 3,
            Instruction::Equals(_, _, _) => 3,
        }
    }
}

pub struct IntCode {
    memory: Vec<isize>,
    ip: usize,
    input_index: usize,
    halted: bool,
}

impl IntCode {
    pub fn new(input: &[isize]) -> IntCode {
        IntCode {
            memory: input.to_vec(),
            ip: 0,
            input_index: 0,
            halted: false,
        }
    }

    pub fn decode(&self) -> Instruction {
        let inst = self.memory[self.ip];
        let arg1 = Arg::new(inst, self.memory[self.ip + 1], 1);
        let arg2 = Arg::new(inst, self.memory[self.ip + 2], 2);
        let arg3 = Arg::new(inst, self.memory[self.ip + 3], 3);
        match inst % 100 {
            99 => Instruction::Halt,
            1 => Instruction::Add(arg1, arg2, arg3),
            2 => Instruction::Mul(arg1, arg2, arg3),
            3 => Instruction::Input(arg1),
            4 => Instruction::Output(arg1),
            5 => Instruction::JNZ(arg1, arg2),
            6 => Instruction::JZ(arg1, arg2),
            7 => Instruction::LessThan(arg1, arg2, arg3),
            8 => Instruction::Equals(arg1, arg2, arg3),
            _ => unimplemented!(),
        }
    }

    pub fn execute(&mut self, instruction: &Instruction, input: &[isize], output: &mut Vec<isize>) {
        match instruction {
            Instruction::Halt => {
                self.halted = true;
                return;
            }
            Instruction::Add(x, y, z) => {
                let x = x.resolve_in(&self.memory);
                let y = y.resolve_in(&self.memory);
                let z = z.resolve_out();
                self.memory[z] = x + y;
            }
            Instruction::Mul(x, y, z) => {
                let x = x.resolve_in(&self.memory);
                let y = y.resolve_in(&self.memory);
                let z = z.resolve_out();
                self.memory[z] = x * y;
            }
            Instruction::Input(x) => {
                if self.input_index >= input.len() {
                    // Stall waiting for input.
                    return;
                }
                let x = x.resolve_out();
                self.memory[x] = input[self.input_index];
                self.input_index += 1;
            }
            Instruction::Output(x) => {
                let x = x.resolve_in(&self.memory);
                output.push(x);
            }
            Instruction::JNZ(x, y) => {
                let x = x.resolve_in(&self.memory);
                let y = y.resolve_in(&self.memory);
                if x != 0 {
                    self.ip = y as usize;
                    return;
                }
            }
            Instruction::JZ(x, y) => {
                let x = x.resolve_in(&self.memory);
                let y = y.resolve_in(&self.memory);
                if x == 0 {
                    self.ip = y as usize;
                    return;
                }
            }
            Instruction::LessThan(x, y, z) => {
                let x = x.resolve_in(&self.memory);
                let y = y.resolve_in(&self.memory);
                let z = z.resolve_out();
                self.memory[z] = if x < y { 1 } else { 0 };
            }
            Instruction::Equals(x, y, z) => {
                let x = x.resolve_in(&self.memory);
                let y = y.resolve_in(&self.memory);
                let z = z.resolve_out();
                self.memory[z] = if x == y { 1 } else { 0 };
            }
        }
        self.ip += instruction.arg_count();
    }
}
