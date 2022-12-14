use std::collections::VecDeque;

mod util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = util::get_input_from_file("example.txt");
        let mut barrel = Barrel::new(&input, 3);
        for _ in 0..20 {
            barrel.run_round()
        }
        let mb = barrel.monkey_business();
        assert_eq!(mb, 10605);
    }

    #[test]
    fn test2() {
        let input = util::get_input_from_file("example.txt");
        let mut barrel = Barrel::new(&input, 1);
        for _ in 0..10_000 {
            barrel.run_round()
        }
        let mb = barrel.monkey_business();
        assert_eq!(mb, 2713310158);
    }
}

#[derive(Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    op: Op,
    divisor: u64,
    targ_true: usize,
    targ_false: usize,
    num_inspections: usize,
}

#[derive(Debug)]
struct Barrel {
    monkey: Vec<Monkey>,
    items: Vec<VecDeque<u64>>,
    relief: u64,
    base: u64,
}

impl Op {
    fn run(&self, item: u64) -> u64 {
        match self {
            Op::Add(val) => item + val,
            Op::Mul(val) => item * val,
            Op::Square => item * item,
        }
    }
}

impl Monkey {
    fn new(op: Op, divisor: u64, targ_true: usize,
           targ_false: usize) -> Monkey {
        Monkey { op: op, divisor: divisor,
            targ_true: targ_true, targ_false: targ_false, num_inspections: 0}
    }

    fn inspect(&mut self, item: u64, relief: u64, base: u64) 
            -> (usize, u64) {
        self.num_inspections += 1;
        let mut val = self.op.run(item) % base;
        if relief > 1 {
            val /= relief;
        }
        if val % self.divisor == 0 {
            return (self.targ_true, val)
        }
        (self.targ_false, val)
    }

}

impl Barrel {
    fn new(input: &str, relief: u64) -> Barrel {
        let mut lines = input.lines();

        lines.next();

        let mut monkeys = Vec::new();
        let mut items = Vec::new();
        let mut idx = 0;
        let mut base = 1;

        loop {

            items.push(VecDeque::new());

            let item_line = lines.next().unwrap();
            let toks = item_line.split(": ")
                .collect::<Vec<&str>>()[1]
                .split(", ");
            for tok in toks {
                items[idx].push_back(tok.parse().unwrap());
            }

            let op_line = lines.next().unwrap();
            let op_str = op_line.split(" = ").collect::<Vec<&str>>()[1];
            let op_toks: Vec<&str> = op_str.split(" ").collect();

            let op;
            if op_toks[1] == "+" {
                op = Op::Add(op_toks[2].parse().unwrap());
            }
            else {
                if op_toks[2] == "old" {
                    op = Op::Square;
                }
                else {
                    op = Op::Mul(op_toks[2].parse().unwrap());
                }
            }
            
            let div_line = lines.next().unwrap();
            let divisor = div_line.split_whitespace()
                .collect::<Vec<&str>>()[3].parse().unwrap();
            
            let tt_line = lines.next().unwrap();
            let targ_true = tt_line.split_whitespace()
                .collect::<Vec<&str>>()[5].parse().unwrap();
            
            let tf_line = lines.next().unwrap();
            let targ_false = tf_line.split_whitespace()
                .collect::<Vec<&str>>()[5].parse().unwrap();
            
            monkeys.push(Monkey::new(op, divisor, targ_true, targ_false));
            idx += 1;
            base *= divisor;

            let opt_line = lines.next();

            if opt_line == None {
                break;
            }

            lines.next();
        }

        Barrel { monkey: monkeys, items: items, relief: relief, base: base}
    }
    fn run_turn(&mut self, idx: usize) {
        while self.items[idx].len() > 0 {
            let item = self.items[idx].pop_front().unwrap();
            let (targ, val) = self.monkey[idx]
                .inspect(item, self.relief, self.base);
            self.items[targ].push_back(val);
        }
    }

    fn run_round(&mut self) {
        for idx in 0..self.monkey.len() {
            self.run_turn(idx);
        }
    }

    fn monkey_business(&self) -> usize {
        let mut nums = Vec::new();

        for m in self.monkey.iter() {
            nums.push(m.num_inspections);
        }

        nums.sort();

        let n = self.monkey.len();

        nums[n-1] * nums[n-2]
    }
}


fn main() {

    let input = util::get_input(2022, 11);
    let mut barrel = Barrel::new(&input, 3);

    for _ in 0..20 {
        barrel.run_round();
    }

    println!("Monkey business is: {}", barrel.monkey_business());
    
    let mut barrel2 = Barrel::new(&input, 1);

    println!("{}: {:?}", 0, barrel2.items);
    for _i in 0..10000 {
        barrel2.run_round();
    }

    println!("{:?}", barrel2.monkey.iter()
                    .map(|m| m.num_inspections)
                    .collect::<Vec<usize>>());

    println!("Oh no. Monkey business is now: {}", barrel2.monkey_business());

}
