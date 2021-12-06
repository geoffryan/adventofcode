use std::env;
use std::str;
use std::fs;


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

    let count = x.iter().fold(0, |c, val| c + ((val & m) >> b) as usize);

    count
}

fn get_gamma_epsilon(x: &[u16], n: usize) -> (u32, u32) {
    
    let half_size = x.len() / 2;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for b in 0..n {
        let c = count_bit(x, b);

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

fn get_oxygen_rating(x: &[u16], n: usize) -> u32 {
   
    let mut y: Vec<u16> = x.iter().cloned().collect();

    for b in 0..n {
        let c = count_bit(&y, n-b-1);
        let m = (1 << n-b-1) as u16;
        let target = if 2*c >= y.len() {m} else {0};
        
        y = y.into_iter().filter(|val| m & val == target).collect::<Vec<u16>>();

        if y.len() == 1 {
            break;
        }
    }
    
    y[0] as u32
}

fn get_co2_rating(x: &[u16], n: usize) -> u32 {
   
    let mut y: Vec<u16> = x.iter().cloned().collect();

    for b in 0..n {
        let c = count_bit(&y, n-b-1);
        let m = (1 << n-b-1) as u16;
        let target = if 2*c < y.len() {m} else {0};
        
        y = y.into_iter().filter(|val| m & val == target)
                .collect::<Vec<u16>>();
    
        if y.len() == 1 {
            break;
        }
    }
    
    y[0] as u32
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let (n, arr) = parse_input(filename);

    println!("entry size: {}", n);

    let (gamma, epsilon) = get_gamma_epsilon(&arr, n);

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);

    println!("power: {}", gamma*epsilon);

    let oxy = get_oxygen_rating(&arr, n);
    let co2 = get_co2_rating(&arr, n);
    
    println!("oxygen: {}", oxy);
    println!("c02: {}", co2);

    println!("life support: {}", oxy*co2);
}
