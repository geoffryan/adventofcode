use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct Inventory {
    items: Vec<Vec<i64>>
}

impl Inventory {

    fn get_totals(&self) -> Vec<i64> {
        let mut tot = Vec::new();

        for inv in self.items.iter() {
            let mut sum = 0;
            for cal in inv.iter() {
                sum += cal;
            }
            tot.push(sum);
        }

        tot
    }

    fn max_cals(&self) -> i64 {
        let mut max: i64 = 0;

        let tot_cals = self.get_totals();

        for &cals in tot_cals.iter() {
            if cals > max {
                max = cals
            }
        }

        max
    }

    fn top_three_sum(&self) -> i64 {
        let mut one = 0;
        let mut two = 0;
        let mut three = 0;

        let tot_cals = self.get_totals();

        for &cals in tot_cals.iter() {
            if cals > one {
                three = two;
                two = one;
                one = cals;
            }
            else if cals > two {
                three = two;
                two = cals;
            }
            else if cals > three {
                three = cals
            }
        }

        one + two + three
    }
}

fn parse_input(filename: &str) -> Inventory {

    let contents = fs::read_to_string(filename).expect("Couldn't open file");
    let lines = contents.lines();

    let mut items: Vec<Vec<i64>> = Vec::new();

    let mut single: Vec<i64> = Vec::new();

    for line in lines {
        if line.is_empty() {
            items.push(single);
            single = Vec::new();
            continue;
        }
        single.push(line.parse::<i64>().unwrap());
    }
    items.push(single);

    Inventory{items}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let inv = parse_input(filename);

    let max_cals = inv.max_cals();

    println!("The maximum load is: {}", max_cals);
    
    let top_three = inv.top_three_sum();
    
    println!("The total load of the top three is: {}", top_three);
}
