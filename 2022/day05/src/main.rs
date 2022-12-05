mod util;

struct Stacks {
    crates: Vec<Vec<char>>
}

impl Stacks {
    fn from(config: &Vec<Vec<char>>) -> Stacks {
        let depth = config.len();
        let n = (config[0].len() + 1) / 4;

        let mut crates: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            crates.push(Vec::new());
        }

        for i in (0..(depth-1)).rev() {
            for j in 0..n {
                let idx = 4*j+1;
                
                if config[i][idx] == ' ' {
                    continue;
                }

                crates[j].push(config[i][idx]);
            }
        }

        Stacks { crates }
    }

    fn use_crane_9000(&mut self, num: usize, from: usize, to: usize) {
        for _ in 0..num {
            let c = self.crates[from-1].pop().unwrap();
            self.crates[to-1].push(c);
        }
    }

    fn use_crane_9001(&mut self, num: usize, from: usize, to: usize) {
        let mut buf = Vec::new();

        for _ in 0..num {
            buf.push(self.crates[from-1].pop().unwrap());
        }
        for _ in 0..num {
            self.crates[to-1].push(buf.pop().unwrap());
        }
    }

    fn top_crates(&self) -> String {

        self.crates.iter()
            .map(|stack| stack[stack.len()-1])
            .collect::<String>()
    }
}

fn main() {
    let input = util::get_input(2022, 5);

    let mut stack_config = Vec::new();
    let mut move_list = Vec::new();

    let mut lines = input.lines();

    let mut line = lines.next().unwrap();
    while !line.trim().is_empty() {
        stack_config.push(line.chars().collect::<Vec<char>>());
        line = lines.next().unwrap();
    }


    for line in lines {
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        let num = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap();
        let to = words[5].parse::<usize>().unwrap();
        move_list.push((num, from, to));
    }

    let mut stacks = Stacks::from(&stack_config);

    for &(num, from, to) in move_list.iter() {
        stacks.use_crane_9000(num, from, to);
    }

    let tops = stacks.top_crates();

    println!("Top crates 9000 are: {}", tops);

    stacks = Stacks::from(&stack_config);

    for &(num, from, to) in move_list.iter() {
        stacks.use_crane_9001(num, from, to);
    }

    let tops = stacks.top_crates();

    println!("Top crates 9001 are: {}", tops);
}
