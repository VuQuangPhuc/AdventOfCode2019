use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve() {
    println!("Solution 1: {:?}", p_one());
    println!("Solution 2: {:?}", p_two());
}

fn p_one() -> i32 {
    let limit = 576723;
    let mut hits = 0;
    let mut password = 109165;
    while password < limit {
        let mut digits: Vec<u32> = split(password);
        make_inc(&mut digits);
        password = revert_to_number(&digits);
        if password > limit {
            break;
        }
        if has_double(&digits) {
            hits = hits + 1;
        }
        password = password + 1;
    }

    hits
}

fn p_two() -> i32 {
    let limit = 576723;
    let mut hits = 0;
    let mut password = 109165;
    while password < limit {
        let mut digits: Vec<u32> = split(password);
        make_inc(&mut digits);
        password = revert_to_number(&digits);
        if password > limit {
            break;
        }
        let occurences: Vec<usize> = get_occurences(&digits);
        if occurences.iter().filter(|o| **o == 2).count() >= 1 {
            hits = hits + 1;
        }
        password = password + 1;
    }

    hits
}

fn get_occurences(digits: &Vec<u32>) -> Vec<usize> {
    let unique: HashSet<&u32> = HashSet::from_iter(digits.iter().clone());
    unique
        .iter()
        .map(|e| digits.iter().filter(|d| **d == **e).count())
        .collect()
}

fn has_double(digits: &Vec<u32>) -> bool {
    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            return true;
        };
    }
    false
}

fn make_inc(digits: &mut Vec<u32>) {
    for i in (1..6).rev() {
        if digits[i] < digits[i - 1] {
            for j in i..6 {
                digits[j] = digits[i - 1];
            }
        }
    }
}

fn revert_to_number(digits: &Vec<u32>) -> u32 {
    digits[0] * 100000
        + digits[1] * 10000
        + digits[2] * 1000
        + digits[3] * 100
        + digits[4] * 10
        + digits[5]
}

fn split(password: u32) -> Vec<u32> {
    password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

