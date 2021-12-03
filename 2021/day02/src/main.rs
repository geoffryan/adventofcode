use std::env;
use std::fs;

#[derive(Debug)]
enum Leg {
    Up(i64),
    Down(i64),
    Forward(i64)
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

    for p in path {
        println!("{:?}", p);
    }


}
