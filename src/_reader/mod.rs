use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn read_lines(input: File) -> Vec<i32> {
    let br = BufReader::new(input);
    br.lines()
        .filter_map(io::Result::ok)
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn read_program(input: File) -> Vec<i32> {
    let br = BufReader::new(input);
    let lines: Vec<Vec<i32>> = br
        .lines()
        .filter_map(io::Result::ok)
        .map(|line| {
            line.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    lines[0].clone()
}

pub fn read_wires(input: File) -> Vec<Vec<(char, i32)>> {
    let br = BufReader::new(input);
    br.lines()
        .filter_map(io::Result::ok)
        .map(|line| {
            line.split(",")
                .map(|s| parse_path_instruction(s))
                .collect()
        })
        .collect()
}

fn parse_path_instruction(instruction: &str) -> (char, i32) {
    let direction = instruction.chars().next().unwrap();
    let distance = instruction[1..].parse().unwrap();
    (direction, distance)
}
