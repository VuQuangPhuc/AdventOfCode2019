use _solver::Solver;

pub fn solve() {
    let input = include_str!("../../input/day05.csv");
    let mut p_one = Solver::new(input, vec![1]);
    p_one.solve();

    let mut p_two = Solver::new(input, vec![5]);
    p_two.solve();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let mut solver = Solver::new(input, vec![5]);
        solver.solve()
    }

    #[test]
    fn second() {
        let input = "3,3,1105,5,9,1101,0,0,12,4,12,99,1";
        let mut solver = Solver::new(input, vec![5]);
        solver.solve()
    }

    #[test]
    fn third() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut solver = Solver::new(input, vec![5]);
        solver.solve()
    }
}
