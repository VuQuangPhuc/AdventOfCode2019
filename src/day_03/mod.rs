use crate::_reader;
use std::collections::HashSet;
use std::fs::File;

pub fn solve() {
    let file = File::open("./input/day03.csv").unwrap();
    let input = _reader::read_wires(file);
    println!("Solution 1: {:?}", p_one(&input));
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

    return result[0]
}

fn parse_path(wire: &Vec<(char, i32)>, set: &mut HashSet<(i32, i32)>) {
    let mut pos = (0, 0);
    for instruction in wire {
        match instruction.0 {
            'U' => {
                for _ in 0..instruction.1 {
                    pos.1 = pos.1 + 1;
                    set.insert(pos);
                }
            }
            'D' => {
                for _ in 0..instruction.1 {
                    pos.1 = pos.1 - 1;
                    set.insert(pos);
                }
            }
            'L' => {
                for _ in 0..instruction.1 {
                    pos.0 = pos.0 - 1;
                    set.insert(pos);
                }
            }
            'R' => {
                for _ in 0..instruction.1 {
                    pos.0 = pos.0 + 1;
                    set.insert(pos);
                }
            }
            _ => println!("f"),
        }
    }
}

fn distance(point: &(i32, i32)) -> i32 {
    point.0.abs() + point.1.abs()
}
