use std::env;
use std::fs;

fn fuel2(x: i32, s: i32) -> i32 {

    let d = (x - s).abs();
    (d*(d+1))/2
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(&filename).expect("Couldn't read file");

    let mut positions: Vec<i32> = contents.trim().split(",")
        .map(|x| x.parse().unwrap()).collect();

    positions.sort();

    let n = positions.len();

    let median = positions[n/2];

    let mut fuel = 0;
    for x in positions.iter() {
        fuel += (x-median).abs();
    }

    println!("Best Position: {}", median);
    println!("Best Fuel Consumption: {}", fuel);

    let s: Vec<i32> = (0..(*(positions.iter().max().unwrap()))).collect();

    println!("Computing ALL the fuels.");
    let F: Vec<i32> = s.iter().map(
        |&si| positions.iter().fold(0, |tot, &x| tot + fuel2(x, si)))
        .collect();

    println!("Finding the min.");
    println!("Min fuel: {}", F.iter().min().unwrap());
}
