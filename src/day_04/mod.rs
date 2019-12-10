use std::collections::HashSet;

pub fn solve() {
    println!("Solution 1: {:?}", not_brute_force());
}


fn p_two() -> i32 {
    more_brute_force()
}

fn not_brute_force() -> i32 {
    let limit = 576723;
    let mut hits = 0;
    let mut password = 109165;
    while password < limit {
        let mut digits: Vec<u32> = password
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        make_inc(&mut digits);
        password = back(&digits);
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

fn make_inc(digits: &mut Vec<u32>) {
    for i in (1..6).rev() {
        if digits[i] < digits[i - 1] {
            for j in i..6 {
                digits[j] = digits[i - 1];
            }
        }
    }
}

fn has_double(digits: &Vec<u32>) -> bool {
    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            return true;
        };
    }
    false
}

fn back(digits: &Vec<u32>) -> u32 {
    digits[0] * 100000
        + digits[1] * 10000
        + digits[2] * 1000
        + digits[3] * 100
        + digits[4] * 10
        + digits[5]
}

fn more_brute_force() -> i32 {
    let mut hits = 0;
    for i in 109165..576723 {
        let arr: Vec<u32> = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut not_decreasing = true;
        let mut adjacent = false;
        let mut seen: HashSet<u32> = HashSet::new();
        for j in 0..5 {
            if arr[j] > arr[j + 1] {
                not_decreasing = false;
            }
            if arr[j] == arr[j + 1] {
                if j != 5 && seen.insert(arr[j]) {
                    let mut count = 1;
                    for k in j + 1..6 {
                        if arr[k] == arr[j] {
                            count = count + 1;
                        }
                    }
                    if count == 2 {
                        adjacent = true;
                    }
                }
            }
        }
        if not_decreasing && adjacent {
            hits = hits + 1;
        }
    }

    hits
}
