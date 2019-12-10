use std::collections::HashSet;

pub fn solve() {
    println!("Solution 1: {:?}", p_one());
    println!("Solution 2: {:?}", p_two());
}

fn p_one() -> i32 {
    brute_force()
}

fn p_two() -> i32 {
    more_brute_force()
}

fn brute_force() -> i32 {
    let mut hits = 0;
    for i in 109165..576724 {
        let arr: Vec<u32> = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut not_decreasing = true;
        let mut adjacent = false;
        for j in 0..5 {
            if arr[j] > arr[j + 1] {
                not_decreasing = false;
            }
            if arr[j] == arr[j + 1] {
                adjacent = true;
            }
        }
        if not_decreasing && adjacent {
            hits = hits + 1;
        }
    }

    hits
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
                    for k in j + 1..6{
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
