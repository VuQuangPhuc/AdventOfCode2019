pub fn solve() -> () {
    let input = include_str!("../../input/day02.csv");
    let program = parse_input(&mut input.to_string());
    println!("First solution: {}", p_one(program.clone()));
    println!("Second solution: {:?}", p_two(program.clone()));
}

fn p_one(mut input: Vec<i32>) -> i32 {
    execute(init_memory(&mut input, 12, 2))
}

fn p_two(input: Vec<i32>) -> (i32, i32) {
    let mut tmp = 0;
    for i in 0..100 {
        let mut base = input.clone();
        let mut memory = init_memory(&mut base, i, 0);
        if execute(&mut memory) > 19690720 {
            tmp = i - 1;
            break;
        }
    }
    for j in 0..100 {
        let mut base = input.clone();
        let mut memory = init_memory(&mut base, tmp, j);
        if execute(&mut memory) == 19690720 {
            return (tmp, j);
        }
    }

    return (0, 0);
}

fn parse_input(mut input: &mut String) -> Vec<i32> {
    trim_newline(&mut input);
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
                input[t] = input[lhs] + input[rhs];
            }
            2 => {
                let (t, lhs, rhs) = get_addresses(&input, ip);
                input[t] = input[lhs] * input[rhs];
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

fn init_memory(input: &mut Vec<i32>, noun: i32, verb: i32) -> &mut Vec<i32> {
    input[1] = noun;
    input[2] = verb;
    input
}
