#[derive(Clone)]
enum Mode {
    POSITION,
    IMMEDIATE,
}

#[derive(Clone)]
struct Param {
    mode: Mode,
    value: i64,
}

impl Param {
    pub fn new(value: i64, mode: i64) -> Param {
        let mode = match mode {
            0 => Mode::POSITION,
            1 => Mode::IMMEDIATE,
            _ => panic!("Unknown mode: {}", mode),
        };

        Param { mode, value }
    }
}

enum Instruction {
    Add((Param, Param, Param)),
    Mult((Param, Param, Param)),
    Input(Param),
    Output(Param),
    JumpIfTrue((Param, Param)),
    JumpIfFalse((Param, Param)),
    LessThan((Param, Param, Param)),
    Equal((Param, Param, Param)),
    Stop,
}

pub struct Computer {
    program: Vec<i64>,
    init: Vec<i64>,
    input: Vec<i64>,
    ip: usize,
    output: Option<i64>,
}

impl Computer {
    pub fn new(program: &Vec<i64>, init: Vec<i64>, input: Vec<i64>) -> Computer {
        let mut c = Computer {
            program: program.clone(),
            init,
            input,
            ip: 0,
            output: None,
        };
        c.set_input();
        c
    }

    pub fn execute(&mut self) -> () {
        loop {
            let instruction = self.get_instruction();
            match instruction {
                Instruction::Stop => {
                    return;
                }
                Instruction::Add((target, lhs, rhs)) => {
                    let lhs = self.get_value(lhs);
                    let rhs = self.get_value(rhs);
                    self.set_value(target, lhs + rhs);
                }
                Instruction::Mult((target, lhs, rhs)) => {
                    let lhs = self.get_value(lhs);
                    let rhs = self.get_value(rhs);
                    self.set_value(target, lhs * rhs);
                }
                Instruction::Input(target) => {
                    self.set_value(target, self.input.clone().pop().unwrap());
                }
                Instruction::Output(target) => {
                    self.output = Some(self.get_value(target));
                }
                Instruction::JumpIfTrue((target, eval)) => {
                    if self.get_value(eval) != 0 {
                        self.ip = self.get_value(target) as usize;
                    };
                }
                Instruction::JumpIfFalse((target, eval)) => {
                    if self.get_value(eval) == 0 {
                        self.ip = self.get_value(target) as usize;
                    };
                }
                Instruction::LessThan((target, lhs, rhs)) => {
                    let lhs = self.get_value(lhs);
                    let rhs = self.get_value(rhs);
                    if lhs < rhs {
                        self.set_value(target, 1);
                    } else {
                        self.set_value(target, 0);
                    };
                }
                Instruction::Equal((target, lhs, rhs)) => {
                    let lhs = self.get_value(lhs);
                    let rhs = self.get_value(rhs);
                    if lhs == rhs {
                        self.set_value(target, 1);
                    } else {
                        self.set_value(target, 0);
                    };
                }
            }
        }
    }

    pub fn emit_output(&self) -> () {
        println!("Output: {}", self.output.unwrap());
    }

    fn fetch_params_1(&mut self, instruction: i64) -> Param {
        let base = self.ip;
        self.ip += 2;
        let mode = (instruction / 100) % 10;
        Param::new(self.program[base + 1], mode)
    }

    fn fetch_params_2(&mut self, instruction: i64) -> (Param, Param) {
        let base = self.ip;
        self.ip += 3;
        let mode = instruction / 100;
        let op1 = Param::new(self.program[base + 1], mode % 10);
        let t = Param::new(self.program[base + 2], (mode / 10) % 10);
        (t, op1)
    }

    fn fetch_params_3(&mut self, instruction: i64) -> (Param, Param, Param) {
        let base = self.ip;
        self.ip += 4;
        let mode = instruction / 100;
        let op1 = Param::new(self.program[base + 1], mode % 10);
        let op2 = Param::new(self.program[base + 2], (mode / 10) % 10);
        let t = Param::new(self.program[base + 3], (mode / 100) % 10);
        (t, op1, op2)
    }

    fn get_instruction(&mut self) -> Instruction {
        let instruction = self.program[self.ip];
        let opcode = instruction % 100;
        match opcode {
            1 => Instruction::Add(self.fetch_params_3(instruction)),
            2 => Instruction::Mult(self.fetch_params_3(instruction)),
            3 => Instruction::Input(self.fetch_params_1(instruction)),
            4 => Instruction::Output(self.fetch_params_1(instruction)),
            5 => Instruction::JumpIfTrue(self.fetch_params_2(instruction)),
            6 => Instruction::JumpIfFalse(self.fetch_params_2(instruction)),
            7 => Instruction::LessThan(self.fetch_params_3(instruction)),
            8 => Instruction::Equal(self.fetch_params_3(instruction)),
            99 => Instruction::Stop,
            _ => panic!("Unknown Instruction {} with opcode {}", instruction, opcode),
        }
    }

    fn get_value(&self, param: Param) -> i64 {
        match param.mode {
            Mode::IMMEDIATE => param.value,
            Mode::POSITION => self.program[param.value as usize],
        }
    }

    fn set_input(&mut self) -> () {
        for i in 0..self.init.len() {
            self.program[i + 1] = self.init[i]
        }
    }

    fn set_value(&mut self, target: Param, value: i64) -> () {
        self.program[target.value as usize] = value;
    }
}