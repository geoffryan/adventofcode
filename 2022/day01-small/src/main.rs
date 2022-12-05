use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Couldn't read file");

    let mut cals: Vec<i64> = Vec::new();
    let mut sum = 0;

    for line in contents.lines() {
        if line.is_empty() {
            cals.push(sum);
            sum = 0;
            continue;
        }

        sum += line.parse::<i64>().unwrap();
    }
    cals.push(sum);

    cals.sort();

    let n = cals.len();

    println!("Max load: {}", cals[n-1]);
    println!("Max 3 loads: {}", cals[n-1] + cals[n-2] + cals[n-3]);
}
