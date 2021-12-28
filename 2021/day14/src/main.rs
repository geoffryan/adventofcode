use std::env;
use std::fmt;
use std::fs;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pair {
    val: u32
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DeepPair {
    p: Pair,
    depth: usize
}

impl Pair {
    fn new_from_bytes(a: u8, b: u8) -> Pair {
        Pair { val: ((a as u32) << 8) | (b as u32) }
    }
    fn new_from_u16(p: u16) -> Pair {
        Pair { val: p as u32 }
    }
    fn chars(&self) -> (char, char) {
        (((self.val & 0x0000FF00) >> 8) as u8 as char,
         (self.val & 0x000000FF) as u8 as char)
    }
    fn count_second_char(&self, record: &mut HashMap<char, usize>) {
        let c = self.chars();
        let count_b = record.entry(c.1).or_insert(0);
        *count_b += 1;
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cs = self.chars();
        write!(f, "{}{}", cs.0, cs.1)
    }
}

impl fmt::Debug for DeepPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.p, self.depth)
    }
}

impl fmt::Display for DeepPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.p, self.depth)
    }
}

fn insert_pairs(s0: &str, pairs: &Vec<(u16, u8)>) -> String {

    let s0b = s0.as_bytes();
    let n = s0b.len();

    let mut sb: Vec<u8> = Vec::new();

    for i in 0..(n-1) {
        //let a0 = s0b[i] as char;
        //let b0 = s0b[i+1] as char;

        let ab0 = ((s0b[i] as u16) << 8) + (s0b[i+1] as u16) as u16;

        match pairs.binary_search_by(|probe| (probe.0).cmp(&ab0)) {
            Ok(idx) => {
                sb.push(s0b[i]);
                sb.push(pairs[idx].1);
            },
            _ => {
                sb.push(s0b[i]);
            }
        }

        /*
        let mut found = false;

        for &pair in pairs.iter() {
            if pair.0 == ab0 {
                found = true;
                //s = format!("{}{}{}", s, a0, pair.1);
                sb.push(s0b[i]);
                sb.push(pair.1);
                break;
            }
        }
        if !found {
            //s = format!("{}{}", s, a0);
            sb.push(s0b[i]);
        }
        */
    }
    //s = format!("{}{}", s, s0b[n-1] as char);
    sb.push(s0b[n-1]);

    String::from_utf8(sb).unwrap()
}

fn parse_input(filename: &str) -> (String, Vec<(u16, u8)>) {

    let contents = fs::read_to_string(filename).expect("Couldn't open file");

    let mut lines = contents.lines();
    let template = String::from(lines.next().unwrap());

    let mut subs: Vec<(u16, u8)> = Vec::new();

    lines.next();

    for line in lines {
        let mut cs = line.chars();
        let a = cs.next().unwrap();
        let b = cs.next().unwrap();
        let c = cs.last().unwrap();
        let ab = ((a as u16) << 8) + (b as u16);
        subs.push((ab, c as u8));
    }

    subs.sort_by(|a, b| (a.0).cmp(&b.0));
    //subs.sort();

    println!("{:?}", subs);

    (template, subs)
}

fn count_chars(s: &str) -> HashMap<char, usize> {

    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    counts
}

fn get_minmax_counts(counts: &HashMap<char, usize>) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut max = 0;

    for &count in counts.values() {
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }
    
    (min, max)
}

fn run_longer(s0: &str, pairs0: &Vec<(u16, u8)>, depth: usize) 
        -> HashMap<char, usize> {
    let s0b = s0.as_bytes();

    let mut template: Vec<Pair> = Vec::new();
    for i in 0..(s0b.len()-1) {
        template.push(Pair::new_from_bytes(s0b[i], s0b[i+1]));
    }

    let n = template.len();

    let mut subs: HashMap<Pair, (Pair, Pair)> = HashMap::new();

    for p in pairs0.iter() {
        let key = Pair::new_from_u16(p.0);
        let pa = Pair::new_from_bytes((p.0 >> 8) as u8, p.1);
        let pb = Pair::new_from_bytes(p.1, p.0 as u8);
        subs.insert(key, (pa, pb));
    }

    let mut active: Vec<DeepPair> = Vec::new();
    for i in 0..n {
        active.push(DeepPair { p: template[n-1-i], depth: depth });
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut ledger = HashMap::new();

    while active.len() > 0 {
        let dp = active.pop().unwrap();
        
        /*
        if dp.depth == 0 {
            dp.p.count_first_char(&mut counts);
            continue;
        }

        match subs.get(&dp.p) {
            Some(&(pa, pb)) => {
                active.push(DeepPair { p: pb, depth: dp.depth-1});
                active.push(DeepPair { p: pa, depth: dp.depth-1});
                },
            None => {
                dp.p.count_first_char(&mut counts);
            }
        }
        */

        let subcounts = run_recurse(dp, &subs, &mut ledger);

        for (&c, &val) in subcounts.iter() {
            let count = counts.entry(c).or_insert(0);
            *count += val;
        }

        //println!("{}: {:?}", dp.p, &ledger);

    }
    template[n-1].count_second_char(&mut counts);
    
    println!("{:?}", counts);

    counts
}

fn run_recurse(target: DeepPair,
               subs: &HashMap<Pair, (Pair, Pair)>,
               ledger: &mut HashMap<DeepPair, HashMap<char, usize>>) 
        -> HashMap<char, usize> {

    if let Some(counts) = ledger.get(&target) {
        return counts.clone();
    }

    if target.depth == 0 {
        //let mut counts = HashMap::new();
        //dp.p.count_first_char(&mut counts);
        let counts = HashMap::from([(target.p.chars().0, 1)]);
        ledger.insert(target, counts.clone());
        return counts;
    }

    if let Some(&(pa, pb)) = subs.get(&target.p) {

        let dpa = DeepPair { p: pa, depth: target.depth-1 };
        let dpb = DeepPair { p: pb, depth: target.depth-1 };
        let mut counts = run_recurse(dpa, subs, ledger);
        let counts_b = run_recurse(dpb, subs, ledger);

        for (&c, &val) in counts_b.iter() {
            let count = counts.entry(c).or_insert(0);
            *count += val;
        }

        ledger.insert(target, counts.clone());

        return counts;
    }
    
    let counts = HashMap::from([(target.p.chars().0, 1)]);
    ledger.insert(target, counts.clone());
    return counts;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (template, pairs) = parse_input(&filename);

    println!("{}", template);

    let mut s = template.clone();

    for _ in 0..10 {
        s = insert_pairs(&s, &pairs);
    }

    let counts = count_chars(&s);
    let (min, max) = get_minmax_counts(&counts);
    println!("After 10: {} {} {}", min, max, max-min);
    
    let depth = 40;

    let ledger2 = run_longer(&template, &pairs, depth);
    let (min, max) = get_minmax_counts(&ledger2);
    println!("After {}: {} {} {}", depth, min, max, max-min);
}
