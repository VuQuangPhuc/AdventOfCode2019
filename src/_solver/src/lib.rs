extern crate _computer;
use _computer::Computer;

pub struct Solver {
    computer: Computer,
}

impl Solver {
    pub fn new(init: &str, input: Vec<i64>) -> Solver {
        let instruction_set = Solver::parse_input(&mut init.to_string());

        Solver {
            computer: Computer::new(&instruction_set, vec![], input),
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