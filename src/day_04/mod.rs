pub fn solve() {
    println!("Solution 1: {:?}", p_one());
}

fn p_one() -> i32 {
    brute_force()
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
