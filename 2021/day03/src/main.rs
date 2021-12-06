use std::env;
use std::str;
use std::fs;

/*
fn load_arr<T>(filename: &str) -> Vec<T> 
    where T: str::FromStr { //, <T as str::FromStr>::Err: fmt::Debug {

    let contents = fs::read_to_string(filename)
        .expect("Couldn't read the file.");
    let arr = contents.lines().map(
                |line| match line.parse::<T>() {
                        Ok(x) => x,
                        _ => panic!("Couldn't parse val: {}", line)})
                .collect();

    arr
}
*/

fn parse_input(filename: &str) -> (usize, Vec<u16>) {
    
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read the file.");
    let arr = contents.lines().map(
                |line| u16::from_str_radix(line, 2).unwrap()).collect();

    let n = contents.lines().fold(0, |m, line| usize::max(m, line.len()));

    (n, arr)
}

fn count_bit(x: &[u16], b: usize) -> usize {

    let m: u16 = 1 << b;
    println!("{}", m);

    let count = x.iter().fold(0, |c, val| c + ((val & m) >> b) as usize);

    count
}

fn get_gamma_epsilon(x: &[u16], n: usize) -> (u32, u32) {
    
    let half_size = x.len() / 2;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for b in 0..n {
        println!("bit: {}", b);
        let c = count_bit(x, b);
        println!("{}", c);

        let val: u16 = 1 << b;

        if c > half_size {
            gamma += val as u32;
        }
        else {
            epsilon += val as u32;
        }
    }

    (gamma, epsilon)
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let (n, arr) = parse_input(filename);

    println!("{:?}", arr);
    println!("{}", n);

    let (gamma, epsilon) = get_gamma_epsilon(&arr, n);

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);

    println!("power: {}", gamma*epsilon);
}
