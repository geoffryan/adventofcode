mod util;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_a(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let result = find_start_packet(&input);
        assert_eq!(result, answer);
    }

    fn run_b(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let result = find_start_message(&input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test1_a() {
        run_a("ex1.txt", 7);
    }
    #[test]
    fn test2_a() {
        run_a("ex2.txt", 5);
    }
    #[test]
    fn test3_a() {
        run_a("ex3.txt", 6);
    }
    #[test]
    fn test4_a() {
        run_a("ex4.txt", 10);
    }
    #[test]
    fn test5_a() {
        run_a("ex5.txt", 11);
    }

    #[test]
    fn test1_b() {
        run_b("ex1.txt", 19);
    }
    #[test]
    fn test2_b() {
        run_b("ex2.txt", 23);
    }
    #[test]
    fn test3_b() {
        run_b("ex3.txt", 23);
    }
    #[test]
    fn test4_b() {
        run_b("ex4.txt", 29);
    }
    #[test]
    fn test5_b() {
        run_b("ex5.txt", 26);
    }
}

fn test_all_different(s: &[char]) -> bool {
    for i in 1..s.len() {
        for j in 0..i {
            if s[i] == s[j] {
                return false;
            }
        }
    }
    
    true
}

fn find_first_distinct_n(msg: &String, n: usize) -> usize {

    let tokens: Vec<char> = msg.chars().collect();

    let mut offset = n;

    for i in 0..tokens.len()-n {
        if test_all_different(&tokens[i..i+n]) {
            break;
        }
        offset += 1;
    }

    offset
}

fn find_start_packet(msg: &String) -> usize {
    find_first_distinct_n(msg, 4)
}

fn find_start_message(msg: &String) -> usize {
    find_first_distinct_n(msg, 14)
}

fn main() {
    let input = util::get_input(2022, 6);

    println!("Hello world!");

    let offset = find_start_packet(&input);
    println!("Start packet offset: {}", offset);

    let offset = find_start_message(&input);
    println!("Start message offset: {}", offset);
}
