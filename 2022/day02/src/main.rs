use std::env;
use std::fs;

fn calc_score(a: char, b: char) -> i64 {
    let shape_score = match b {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Bad input"),
    };
    let match_score = match (a, b) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        _ => 0
    };

    shape_score + match_score
}

fn calc_target(a: char, b: char) -> char {
    match (a, b) {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!("WHAT"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Couldn't read file");

    let mut score = 0;

    contents.lines().map(|line| (line.chars().nth(0).unwrap(),
                                 line.chars().nth(2).unwrap()))
                    .map(|(a, b)| calc_score(a, b))
                    .for_each(|x| score += x);


    println!("Score: {}", score);

    score = 0;

    contents.lines().map(|line| (line.chars().nth(0).unwrap(),
                                 line.chars().nth(2).unwrap()))
                    .map(|(a, b)| calc_score(a, calc_target(a, b)))
                    .for_each(|x| score += x);


    println!("Actual Score: {}", score);
}
