use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../../input/day03.csv");
    let wires = parse_input(input);
    println!("Solution 1: {:?}", p_one(&wires));
    println!("Solution 2: {:?}", p_two(&wires));
}

struct Instruction {
    direction: char,
    distance: i32,
}

impl Instruction {
    fn new(direction: char, distance: i32) -> Instruction {
        Instruction {
            direction,
            distance,
        }
    }
}

#[derive(Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn p_one(wires: &Vec<Vec<Instruction>>) -> i32 {
    let mut wire_one: HashSet<Point> = HashSet::new();
    let mut wire_two: HashSet<Point> = HashSet::new();

    parse_path(&wires[0], &mut wire_one);
    parse_path(&wires[1], &mut wire_two);

    let mut result = wire_one
        .intersection(&wire_two)
        .map(distance)
        .collect::<Vec<i32>>();
    result.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    result[0]
}

fn p_two(wires: &Vec<Vec<Instruction>>) -> i32 {
    let mut wire_one: HashSet<Point> = HashSet::new();
    let mut wire_two: HashSet<Point> = HashSet::new();

    let steps_one = parse_path(&wires[0], &mut wire_one);
    let steps_two = parse_path(&wires[1], &mut wire_two);

    let mut result = wire_one
        .intersection(&wire_two)
        .map(|t| steps_one[t] + steps_two[t])
        .collect::<Vec<i32>>();
    result.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    result[0]
}

fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line| line.split(",").map(|s| parse_path_instruction(s)).collect())
        .collect()
}

fn parse_path_instruction(instruction: &str) -> Instruction {
    let direction = instruction.chars().next().unwrap();
    let distance = instruction[1..].parse().unwrap();
    Instruction::new(direction, distance)
}

fn parse_path<'a>(wire: &Vec<Instruction>, mut set: &mut HashSet<Point>) -> HashMap<Point, i32> {
    let mut pos = Point::new(0, 0);
    let mut steps = 0;
    let mut step_map: HashMap<Point, i32> = HashMap::new();
    for instruction in wire {
        match instruction.direction {
            'U' => {
                for _ in 0..instruction.distance {
                    execute(&mut pos.y, 1, &mut steps);
                    insert(pos, steps, &mut set, &mut step_map);
                }
            }
            'D' => {
                for _ in 0..instruction.distance {
                    execute(&mut pos.y, -1, &mut steps);
                    insert(pos, steps, &mut set, &mut step_map);
                }
            }
            'L' => {
                for _ in 0..instruction.distance {
                    execute(&mut pos.x, -1, &mut steps);
                    insert(pos, steps, &mut set, &mut step_map);
                }
            }
            'R' => {
                for _ in 0..instruction.distance {
                    execute(&mut pos.x, 1, &mut steps);
                    insert(pos, steps, &mut set, &mut step_map);
                }
            }
            _ => println!("f"),
        }
    }

    step_map
}

fn execute(dimension: &mut i32, difference: i32, steps: &mut i32) {
    *dimension += difference;
    *steps += 1;
}

fn insert(
    position: Point,
    steps: i32,
    set: &mut HashSet<Point>,
    step_map: &mut HashMap<Point, i32>,
) {
    set.insert(position);
    step_map.insert(position, steps);
}

fn distance(point: &Point) -> i32 {
    point.x.abs() + point.y.abs()
}
