use crate::_reader;
use std::fs::File;

pub fn solve() -> () {
    let file = File::open("./input/day02.csv").unwrap();
    let input = _reader::read_line(file);
    println!("First solution: {}", p_one(&input));
    println!("Second solution: {:?}", p_two(&input));
}

fn p_one(input: &Vec<Vec<i32>>) -> i32 {
    execute(&mut init_memory(input, 12, 2))
}

fn p_two(input: &Vec<Vec<i32>>) -> (i32, i32) {
    for i in 0..100 {
        for j in 0..100 {
            let mut memory = init_memory(&input.clone(), i, j);
            if execute(&mut memory) == 19690720 {
                return (i, j);
            }
        }
    }

    return (0, 0);
}

fn execute(input: &mut Vec<i32>) -> i32 {
    let mut ip = 0;
    loop {
        let opcode = input[ip];
        match opcode {
            99 => {
                return input[0];
            }
            1 => {
                let (t, lhs, rhs) = get_addresses(&input, ip);
                input[t as usize] = input[lhs as usize] + input[rhs as usize];
            }
            2 => {
                let (t, lhs, rhs) = get_addresses(&input, ip);
                input[t as usize] = input[lhs as usize] * input[rhs as usize];
            }
            _ => println!("FUCK"),
        }
        ip = ip + 4;
    }
}

fn get_addresses(input: &Vec<i32>, ip: usize) -> (usize, usize, usize) {
    (
        input[ip + 3] as usize,
        input[ip + 1] as usize,
        input[ip + 2] as usize,
    )
}

fn init_memory(input: &Vec<Vec<i32>>, noun: i32, verb: i32) -> Vec<i32> {
    let mut memory = input[0].clone();
    memory[1] = noun;
    memory[2] = verb;
    memory
}
