use std::env;
use std::fs;

#[derive(Debug)]
enum Leg {
    Up(i64),
    Down(i64),
    Forward(i64)
}

#[derive(Debug)]
struct Position {
    depth: i64,
    horizontal: i64,
    aim: i64,
}

impl Position {
    fn mov_bad(&mut self, leg: &Leg) {
        match leg {
            Leg::Up(x) => self.depth -= x,
            Leg::Down(x) => self.depth += x,
            Leg::Forward(x) => self.horizontal += x,
        }
    }

    fn mov(&mut self, leg: &Leg) {
        match leg {
            Leg::Up(x) => self.aim -= x,
            Leg::Down(x) => self.aim += x,
            Leg::Forward(x) => {self.horizontal += x;
                                self.depth += self.aim * x},
        }
    }
}

fn str_to_path(input: &str) -> Leg {
    let words: Vec<&str> = input.split(' ').collect();

    match words[0] {
        "up" => Leg::Up(words[1].parse().unwrap()),
        "down" => Leg::Down(words[1].parse().unwrap()),
        "forward" => Leg::Forward(words[1].parse().unwrap()),
        _ => panic!("Not a known path: {}", words[0])
    }
}

fn parse_input(filename: &str) -> Vec<Leg> {
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read the file!");

    let path: Vec<Leg> = contents.lines().map(
        |line| str_to_path(line)).collect();

    path
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let path = parse_input(&filename);

    let mut pos1 = Position{depth: 0, horizontal: 0, aim: 0};
    let mut pos2 = Position{depth: 0, horizontal: 0, aim: 0};
    
    for leg in path {
        pos1.mov_bad(&leg);
        pos2.mov(&leg);
    }

    println!("Final position BAD: {:?}", pos1);
    println!("Product: {}", pos1.horizontal * pos1.depth);

    println!("Final position GOOD: {:?}", pos2);
    println!("Product: {}", pos2.horizontal * pos2.depth);
}
