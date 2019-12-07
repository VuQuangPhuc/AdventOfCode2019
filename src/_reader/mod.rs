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

pub fn read_line(input: File) -> Vec<Vec<i32>> {
    let br = BufReader::new(input);
    br.lines()
        .filter_map(io::Result::ok)
        .map(|line| {
            line.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}
