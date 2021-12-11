use std::env;
use std::fs;
use std::str;

#[derive(Debug)]
enum ChunkResult {
    Done,
    EndOfInput,
    Incomplete(String),
    Closer(char),
}

fn get_closer(c: char) -> char {
    if c == '(' {
        return ')'
    }
    else if c == '[' {
        return ']'
    }
    else if c == '{' {
        return '}'
    }
    else if c == '<' {
        return '>'
    }

    '-'
}

fn get_err_score(c: char) -> usize {
    if c == ')' {
        return 3
    }
    else if c == ']' {
        return 57
    }
    else if c == '}' {
        return 1197
    }
    else if c == '>' {
        return 25137
    }

    0
}

fn get_comp_char_score(c: char) -> usize {
    if c == ')' {
        return 1
    }
    else if c == ']' {
        return 2
    }
    else if c == '}' {
        return 3
    }
    else if c == '>' {
        return 4
    }

    0
}

fn get_comp_score(s: &str) -> usize {

    let score = s.chars().fold(0, 
                    |score, c| score*5 + get_comp_char_score(c));
    score
}

fn read_chunk(s: &mut str::Chars) -> Result<ChunkResult, char> {

    match s.next() {
        Some(c) => {
            //println!("{}", c);
            if c == ')' || c == ']' || c == '}' || c == '>' {
                return Ok(ChunkResult::Closer(c));
            }
            let closer = get_closer(c);

            loop {
                let cr = read_chunk(s)?;
                match cr {
                    ChunkResult::Done => {continue},
                    ChunkResult::EndOfInput => {
                        return Ok(ChunkResult::Incomplete(
                                    closer.to_string()))},
                    ChunkResult::Incomplete(s) => {
                        return Ok(ChunkResult::Incomplete(
                                  format!("{}{}",s,closer)))},
                    ChunkResult::Closer(c1) => {
                        if c1 == closer {
                            return Ok(ChunkResult::Done);
                        }
                        return Err(c1);
                    },
                }
            }
        }
        None => {return Ok(ChunkResult::EndOfInput)}
    }
}

fn parse_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let mut out: Vec<String> = Vec::new();
    contents.lines().for_each(|s| out.push(String::from(s)));
    out
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let lines = parse_input(&filename);

    let mut err_score = 0;
    let mut comp_scores: Vec<usize> = Vec::new();

    for line in lines {
        let mut char_iter = line.chars();
        loop {
            match read_chunk(&mut char_iter) {
                Ok(ChunkResult::Incomplete(s)) => {
                    comp_scores.push(get_comp_score(&s));
                    println!("{:?}", ChunkResult::Incomplete(s));
                    break},
                Err(c) => {
                    err_score += get_err_score(c);
                    println!("Err: {}", c);
                    break;},
                Ok(ChunkResult::EndOfInput) => {println!("EndOfInput");
                                                break;},
                Ok(ChunkResult::Done) => {continue}
                Ok(ChunkResult::Closer(c)) => {
                    err_score += get_err_score(c);
                    println!("Err in 1: {}", c);
                    break;
                }
            }
        }
    }

    println!("Error Score: {}", err_score);
    println!("Comp Scores: {:?}", comp_scores);
    comp_scores.sort();
    println!("Comp Score: {}", comp_scores.get(comp_scores.len()/2).unwrap());
}
