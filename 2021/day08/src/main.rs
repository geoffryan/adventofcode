use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct DisplayPattern {
    input: Vec<u32>,
    output: Vec<u32>
}

impl DisplayPattern {

    fn new(input: Vec<&str>, output: Vec<&str>) -> DisplayPattern {

        let mut in_c: Vec<u32> = Vec::new();
        let mut out_c: Vec<u32> = Vec::new();

        for s in input.iter() {
            in_c.push(Self::str_to_code(s));
        }
        for s in output.iter() {
            out_c.push(Self::str_to_code(s));
        }
        DisplayPattern{input: in_c, output: out_c}
    }

    fn str_to_code(s: &str) -> u32 {
        let mut code: u32 = 0;
        for c in s.chars() {
            code += match c {
                'a' => 1 << 0,
                'b' => 1 << 1,
                'c' => 1 << 2,
                'd' => 1 << 3,
                'e' => 1 << 4,
                'f' => 1 << 5,
                'g' => 1 << 6,
                _ => 0
            };
        }
        code
    }

    fn count_1478_out(&self) -> usize {
        let mut count = 0;
        for c in self.output.iter() {
            let l = c.count_ones();
            if l == 2 || l == 3 || l == 4 || l== 7 {
                count += 1;
            }
        }
        count
    }

    fn output_val(&self) -> usize {
        let mut s = 0;
        for &x in self.output.iter() {
            s = 10*s + (x as usize);
        }
        s
    }

    fn decode(&self) -> DisplayPattern {

        let mut unknown_codes: Vec<u32> = Vec::new();
        for &c in self.input.iter() {
            unknown_codes.push(c);
        }
        //println!("\nStarting\nunknown: {:?}", unknown_codes);

        let mut cn: [u32; 10] = [0; 10];

        // Get 1
        for &c in unknown_codes.iter() {
            if c.count_ones() == 2 {
                cn[1] = c;
                break;
            }
        }
        unknown_codes.retain(|&c| c != cn[1]);

        // Get 4
        for &c in unknown_codes.iter() {
            if c.count_ones() == 4 {
                cn[4] = c;
                break;
            }
        }
        unknown_codes.retain(|&c| c != cn[4]);

        // Get 7
        for &c in unknown_codes.iter() {
            if c.count_ones() == 3 {
                cn[7] = c;
                break;
            }
        }
        unknown_codes.retain(|&c| c != cn[7]);

        // Get 8
        for &c in unknown_codes.iter() {
            if c.count_ones() == 7 {
                cn[8] = c;
                break;
            }
        }
        unknown_codes.retain(|&c| c != cn[8]);
        if unknown_codes.len() > 6 {
            println!("cn: {:?}", cn);
            println!("unknown: {:?}", unknown_codes);
            println!("orig: {:?}", self.input);
            let counts: Vec<u32> = self.input.iter().map(|&c| c.count_ones())
                                    .collect();
            println!("bit counts: {:?}", counts);
        }

        // Remaining are 0, 2, 3, 5, 6, 9
        
        
        // Get 3
        for &c in unknown_codes.iter() {
            if c.count_ones() == 5 && (c & cn[1] == cn[1]) {
                cn[3] = c;
                break;
            }
        }
        unknown_codes.retain(|&c| c != cn[3]);

        // Now to id some individual segments!

        let cen = cn[3] ^ cn[1];    //central horizontal segments
        let top = cn[7] ^ cn[1];    //top segment
        let mid = cen & cn[4];      //middle segment
        let bot = cen ^ (top | mid); //bottom segment

        // Can build 0 & 9!
        cn[0] = cn[8] ^ mid;
        cn[9] = cn[4] | top | bot;
        
        unknown_codes.retain(|&c| c != cn[0]);
        unknown_codes.retain(|&c| c != cn[9]);

        // Just 2, 5, and 6 left

        for &c in unknown_codes.iter() {
            if c.count_ones() == 6 {
                cn[6] = c;
                break;
            }
        }
        unknown_codes.retain(|&c| c != cn[6]);

        //Can build 2 & 5!

        let bl = cn[8] ^ cn[9];
        let tl = (cn[8] ^ cn[3]) ^ bl;
        let tr = cn[8] ^ cn[6];
        let br = cn[1] ^ tr;

        cn[2] = cen | tr | bl;
        cn[5] = cen | tl | br;

        unknown_codes.retain(|&c| c != cn[2]);
        unknown_codes.retain(|&c| c != cn[5]);
        //println!("Remaining codes: {}", unknown_codes.len());
        if unknown_codes.len() > 0 {
            println!("cn: {:?}", cn);
            println!("unknown: {:?}", unknown_codes);
            println!("orig: {:?}", self.input);
            let counts: Vec<u32> = self.input.iter().map(|&c| c.count_ones())
                                    .collect();
            println!("bit counts: {:?}", counts);
        }

        let mut input_decoded: Vec<u32> = Vec::new();
        let mut output_decoded: Vec<u32> = Vec::new();

        for &c in self.input.iter() {
            for j in 0..10 {
                if c == cn[j] {
                    input_decoded.push(j as u32);
                    break;
                }
            }
        }

        for &c in self.output.iter() {
            for j in 0..10 {
                if c == cn[j] {
                    output_decoded.push(j as u32);
                    break;
                }
            }
        }

        DisplayPattern{input: input_decoded, output: output_decoded}
    }
}

fn parse_input(filename: &str) -> Vec<DisplayPattern> {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let mut patterns: Vec<DisplayPattern> = Vec::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split('|').collect();
        let input: Vec<&str> = words[0].trim().split_whitespace().collect();
        let output: Vec<&str> = words[1].trim().split_whitespace().collect();
        patterns.push(DisplayPattern::new(input, output));
    }

    patterns
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let patterns = parse_input(&filename);

    let mut count = 0;
    for p in patterns.iter() {
        count += p.count_1478_out();
    }
    
    println!("There are a total of {} 1478s in the output.", count);

    let mut sum = 0;
    for p in patterns.iter() {
        let dec = p.decode();
        sum += dec.output_val();
    }

    println!("The sum of all output is: {}", sum);
}
