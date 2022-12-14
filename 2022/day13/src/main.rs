use std::collections::VecDeque;
use std::cmp::Ordering;

mod util;

#[derive(Debug, PartialEq, Eq)]
enum El {
    Int(i64),
    List(Vec<El>),
}

impl El {
    fn from_str(s: &str) -> El {
        let mut vc = s.chars().collect();
        El::from_chars(&mut vc).unwrap()
    }

    fn from_chars(s: &mut VecDeque<char>) -> Option<El> {

        if s.is_empty() {
            return None;
        }

        let c = s.pop_front().unwrap();

        if c == ']' {
            return None
        }
        else if c == '[' {
            let mut l = Vec::new();

            while let Some(e) = El::from_chars(s) {
                l.push(e);
                let cnext = s.pop_front().unwrap();
                if cnext == ']' {
                    break;
                }
            }

            return Some(El::List(l));
        }
        else if c.is_digit(10) {
            let mut cn = Vec::new();
            cn.push(c);

            while let Some(cc) = s.pop_front() {
                if cc.is_digit(10) {
                    cn.push(cc);
                }
                else {
                    s.push_front(cc);
                    break;
                }
            }

            return Some(El::Int(cn.iter()
                                .collect::<String>()
                                .parse().unwrap()));
        }

        None
    }
}

impl PartialOrd for El {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for El {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (El::Int(i1), El::Int(i2)) => {i1.cmp(i2)},
            (El::Int(i), El::List(_)) => {
                El::List(vec![El::Int(*i)]).cmp(other)
            },
            (El::List(_), El::Int(i)) => {
                self.cmp(&El::List(vec![El::Int(*i)]))
            },
            (El::List(v1), El::List(v2)) => {
                let mut ord = Ordering::Equal;

                let n1 = v1.len();
                let n2 = v2.len();
                let n = n1.min(n2);

                for i in 0..n {
                    ord = ord.then(v1[i].cmp(&v2[i]));
                }

                ord.then(n1.cmp(&n2))
            },
        }
    }
}

fn main() {

    let input = util::get_input(2022, 13);

    let mut lines = input.lines();

    let mut idx = 1;
    let mut score = 0;

    while let Some(s1) = lines.next() {
        let s2 = lines.next().unwrap();

        let el1 = El::from_str(s1);
        let el2 = El::from_str(s2);

        if el1.cmp(&el2) == Ordering::Less {
            score += idx;
            // println!("Correct!");
        }
        else {
            // println!("Wrong!");
        }

        idx += 1;

        lines.next();
    }

    println!("Sum of correct indices is: {}", score);

    let mut packets: Vec<El> = input.lines()
                                .filter(|s| s.len() > 0)
                                .map(|s| El::from_str(s))
                                .collect();

    packets.push(El::from_str("[[2]]"));
    packets.push(El::from_str("[[6]]"));

    packets.sort();

    let div1 = El::from_str("[[2]]");
    let div2 = El::from_str("[[6]]");

    let idx1 = packets.binary_search(&div1).unwrap() + 1;
    let idx2 = packets.binary_search(&div2).unwrap() + 1;

    println!("Decoder packet 1 is at index: {}", idx1);
    println!("Decoder packet 2 is at index: {}", idx2);
    println!("Distress decoder key {}", idx1*idx2); 
}
