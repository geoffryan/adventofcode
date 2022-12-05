mod util;

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

    println!("{:?}", stack_config);

    for line in lines {
        println!("{}", line);
        let mut words = line.trim().split_whitespace();
        println!("{:?}", words);
        let num = words.nth(1).unwrap().parse::<i64>().unwrap();
        let from = words.nth(3).unwrap().parse::<i64>().unwrap();
        let to = words.nth(5).unwrap().parse::<i64>().unwrap();
        for _ in 0..num {
            move_list.push((from, to));
        }
    }
    println!("{:?}", move_list);

}
