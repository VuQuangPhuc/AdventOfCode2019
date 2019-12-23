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
    value: i32,
}

impl Param {
    pub fn new(value: i32) -> Param {
        Param {
            mode: Mode::POSITION,
            value,
        }
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
    // instruction_set: Vec<i32>,
    computer: Computer,
}

impl Solver {
    pub fn new() -> Solver {
        let input = include_str!("../../input/day02.csv");
        let instruction_set = Solver::parse_input(&mut input.to_string());

        Solver {
            computer: Computer::new(&instruction_set, vec![12, 2]),
            // instruction_set,
        }
    }

    pub fn solve(&mut self) -> () {
        let result = self.computer.execute();
        println!("Res: {:?}", result);
    }

    fn parse_input(mut input: &mut String) -> Vec<i32> {
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
    program: Vec<i32>,
    input: Vec<i32>,
    ip: usize,
    output: Option<i32>
}

impl Computer {
    pub fn new(program: &Vec<i32>, input: Vec<i32>) -> Computer {
        let mut c = Computer {
            input,
            program: program.clone(),
            ip: 0,
            output: None,
        };
        c.set_input();
        c
    }

    pub fn execute(&mut self) -> i32 {
        loop {
            let instruction = self.get_instruction();
            match instruction {
                Instruction::STOP => {
                    return self.program[0];
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
                },
                Instruction::INPUT(target) => {
                    self.set_value(target, 3);
                },
                Instruction::OUTPUT(target) => {
                    self.output = Some(self.get_value(target));
                }
            }
        }
    }

    fn get_instruction(&mut self) -> Instruction {
        let opcode = self.program[self.ip];
        match opcode {
            1 => Instruction::ADD(self.get_params_3()),
            2 => Instruction::MULT(self.get_params_3()),
            3 => Instruction::INPUT(self.get_params_1()),
            4 => Instruction::OUTPUT(self.get_params_1()),
            99 => Instruction::STOP,
            _ => panic!("Unknown Instruction"),
        }
    }

    fn get_params_1(&mut self) -> Param {
        self.ip += 2;
        Param::new(self.program[self.ip])
    }

    fn get_params_3(&mut self) -> (Param, Param, Param) {
        let base = self.ip;
        self.ip += 4;
        (
            Param::new(self.program[base + 3]),
            Param::new(self.program[base + 1]),
            Param::new(self.program[base + 2]),
        )
    }

    fn get_value(&self, param: Param) -> i32 {
        match param.mode {
            Mode::IMMEDIATE => param.value,
            Mode::POSITION => self.program[param.value as usize],
        }
    }

    fn set_value(&mut self, target: Param, value: i32) -> () {
        self.program[target.value as usize] = value;
    }

    fn set_input(&mut self) -> () {
        for i in 0..self.input.len() {
            self.program[i + 1] = self.input[i]
        }
    }
}
