use crate::_reader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

pub fn solve() {
    let file = File::open("./input/day03.csv").unwrap();
    let input = _reader::read_wires(file);
    println!("Solution 1: {:?}", p_one(&input));
    println!("Solution 2: {:?}", p_two(&input));
}

fn p_one(input: &Vec<Vec<(char, i32)>>) -> i32 {
    let mut wire_one: HashSet<(i32, i32)> = HashSet::new();
    let mut wire_two: HashSet<(i32, i32)> = HashSet::new();

    parse_path(&input[0], &mut wire_one);
    parse_path(&input[1], &mut wire_two);

    let mut result = wire_one
        .intersection(&wire_two)
        .map(distance)
        .collect::<Vec<i32>>();
    result.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    result[0]
}

fn p_two(input: &Vec<Vec<(char, i32)>>) -> i32 {
    let mut wire_one: HashSet<(i32, i32)> = HashSet::new();
    let mut wire_two: HashSet<(i32, i32)> = HashSet::new();

    let steps_one = parse_path(&input[0], &mut wire_one);
    let steps_two = parse_path(&input[1], &mut wire_two);

    let mut result = wire_one
        .intersection(&wire_two)
        .map(|t| steps_one[t] + steps_two[t])
        .collect::<Vec<i32>>();
    result.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    result[0]
}

fn parse_path(wire: &Vec<(char, i32)>, set: &mut HashSet<(i32, i32)>) -> HashMap<(i32, i32), i32> {
    let mut pos = (0, 0);
    let mut steps = 0;
    let mut step_map: HashMap<(i32, i32), i32> = HashMap::new();
    for instruction in wire {
        match instruction.0 {
            'U' => {
                for _ in 0..instruction.1 {
                    pos.1 = pos.1 + 1;
                    set.insert(pos);
                    steps = steps + 1;
                    step_map.insert(pos, steps);
                }
            }
            'D' => {
                for _ in 0..instruction.1 {
                    pos.1 = pos.1 - 1;
                    set.insert(pos);
                    steps = steps + 1;
                    step_map.insert(pos, steps);
                }
            }
            'L' => {
                for _ in 0..instruction.1 {
                    pos.0 = pos.0 - 1;
                    set.insert(pos);
                    steps = steps + 1;
                    step_map.insert(pos, steps);
                }
            }
            'R' => {
                for _ in 0..instruction.1 {
                    pos.0 = pos.0 + 1;
                    set.insert(pos);
                    steps = steps + 1;
                    step_map.insert(pos, steps);
                }
            }
            _ => println!("f"),
        }
    }

    step_map
}

fn distance(point: &(i32, i32)) -> i32 {
    point.0.abs() + point.1.abs()
}
