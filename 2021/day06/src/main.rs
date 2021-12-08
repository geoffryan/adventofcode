use std::env;
use std::fmt;
use std::fs;
use std::num;
use std::str;

struct Ecosystem {
    fish_pops: [u64; 9]
}

impl Ecosystem {
    fn total_count(&self) -> u64 {
        let mut count = 0;
        for i in 0..9 {
            count += self.fish_pops[i];
        }
        count
    }

    fn evol_day(&mut self) {
        let breeders = self.fish_pops[0];
        for i in 0..8 {
            self.fish_pops[i] = self.fish_pops[i+1];
        }
        self.fish_pops[6] += breeders;
        self.fish_pops[8] = breeders;
    }
}

impl fmt::Display for Ecosystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fish_pops.iter().map(|x| (*x).to_string())
                            .reduce(|s, x| s + " " + &x).unwrap())
    }
}

impl str::FromStr for Ecosystem {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pops: [u64; 9] = [0; 9];
        let timers = s.trim().split(",").map(|x| x.parse::<usize>())
                        .collect::<Vec<Result<usize, Self::Err>>>();
        for rt in timers {
            pops[rt?] += 1;
        }
        Ok(Ecosystem { fish_pops: pops })
    }
}


fn parse_input(filename: &str) -> Ecosystem {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    contents.parse().expect("Couldn't parse into Ecosystem")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut ocean = parse_input(&filename);

    println!("Initial state: {}", ocean);
    for i in 0..256 {
        ocean.evol_day();
        println!("After {} days: {}", i+1, ocean);
    }
    println!("Total count: {}", ocean.total_count());
}
