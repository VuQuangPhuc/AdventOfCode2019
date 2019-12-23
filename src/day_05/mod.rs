pub fn solve() {
    let mut solver = Solver::new();
    solver.solve();
}

enum Mode {
    POSITION,
    IMMEDIATE,
}

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
    ADD((Param, Param, Param)),
    MULT((Param, Param, Param)),
    INPUT((Param)),
    OUTPUT((Param)),
    STOP,
}

struct Solver {
    // instruction_set: Vec<i64>,
    computer: Computer,
}

impl Solver {
    pub fn new() -> Solver {
        let input = include_str!("../../input/day05.csv");
        let instruction_set = Solver::parse_input(&mut input.to_string());

        Solver {
            computer: Computer::new(&instruction_set, vec![]),
            // instruction_set,
        }
    }

    pub fn solve(&mut self) -> () {
        self.computer.execute();
        self.computer.emit_output();
    }

    fn parse_input(mut input: &mut String) -> Vec<i64> {
        Solver::trim_newline(&mut input);
        input
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.parse().unwrap())
            .collect()
    }

    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
}

struct Computer {
    program: Vec<i64>,
    input: Vec<i64>,
    ip: usize,
    output: Option<i64>,
}

impl Computer {
    pub fn new(program: &Vec<i64>, input: Vec<i64>) -> Computer {
        let mut c = Computer {
            input,
            program: program.clone(),
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
                Instruction::STOP => {
                    return;
                }
                Instruction::ADD((target, lhs, rhs)) => {
                    let lhs = self.get_value(lhs);
                    let rhs = self.get_value(rhs);
                    self.set_value(target, lhs + rhs);
                }
                Instruction::MULT((target, lhs, rhs)) => {
                    let lhs = self.get_value(lhs);
                    let rhs = self.get_value(rhs);
                    self.set_value(target, lhs * rhs);
                }
                Instruction::INPUT(target) => {
                    self.set_value(target, 1);
                }
                Instruction::OUTPUT(target) => {
                    self.output = Some(self.get_value(target));
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

    fn fetch_params_3(&mut self, instruction: i64) -> (Param, Param, Param) {
        let base = self.ip;
        self.ip += 4;
        let mode = instruction / 100;
        let op1 = Param::new(self.program[base + 1], mode % 10);
        let op2 = Param::new(self.program[base + 2], (mode / 10) % 10);
        let t = Param::new(self.program[base + 3], (mode / 10) % 10);
        (t, op1, op2)
    }

    fn get_instruction(&mut self) -> Instruction {
        let instruction = self.program[self.ip];
        let opcode = instruction % 100;
        match opcode {
            1 => Instruction::ADD(self.fetch_params_3(instruction)),
            2 => Instruction::MULT(self.fetch_params_3(instruction)),
            3 => Instruction::INPUT(self.fetch_params_1(instruction)),
            4 => Instruction::OUTPUT(self.fetch_params_1(instruction)),
            99 => Instruction::STOP,
            _ => panic!("Unknown Instruction {}", opcode),
        }
    }

    fn get_value(&self, param: Param) -> i64 {
        match param.mode {
            Mode::IMMEDIATE => param.value,
            Mode::POSITION => self.program[param.value as usize],
        }
    }
    fn set_input(&mut self) -> () {
        for i in 0..self.input.len() {
            self.program[i + 1] = self.input[i]
        }
    }

    fn set_value(&mut self, target: Param, value: i64) -> () {
        self.program[target.value as usize] = value;
    }
}
