use std::env;
use std::fs;

fn parse_input(filename: &str) -> Vec<i64> {

    let contents = fs::read_to_string(filename)
        .expect("Couldn't read the file!");

    /*
    let mut arr: Vec<i64> = Vec::new();

    for line in contents.lines() {
        arr.push(line.parse::<i64>().unwrap());
    }
    */

    let arr: Vec<i64> = contents.lines().map(
        |line| line.parse::<i64>().unwrap()).collect();

    arr
}

fn count_increases(depths: &[i64]) -> i64 {
    let mut count: i64 = 0;

    for i in 1..depths.len() {
        //println!("{} {} {}", i, depths[i-1], depths[i]);
        if depths[i] > depths[i-1] {
            count += 1;
        }
    }

    count
}

fn window_sum(size: usize, arr: &[i64]) -> Vec<i64> {

    let mut out: Vec<i64> = Vec::new();

    for i in 0..arr.len()-size+1 {
        let mut sum = 0;
        for j in 0..size {
            sum += arr[i+j];
        }
        out.push(sum)
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Gonna read {}", &filename);

    let arr = parse_input(&filename);

    let count = count_increases(&arr);
    println!("There are {} increases.", count);
    
    let arr1 = window_sum(1, &arr);
    let count1 = count_increases(&arr1);
    
    println!("There are {} increases after window-1", count1);
    
    let arr3 = window_sum(3, &arr);
    let count3 = count_increases(&arr3);
    
    println!("There are {} increases after window-3", count3);
    
}
