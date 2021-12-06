use std::fs;
use std::env;

struct Card {
    grid: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
    won: bool
}

impl Card {

    fn new(grid: &[[u32; 5]; 5]) -> Card {
        let mut mygrid: [[u32; 5]; 5] = [[0; 5]; 5];
        let mymarks = [[false; 5]; 5];
        mygrid.copy_from_slice(grid);

        Card { grid: mygrid, marks: mymarks, won: false}
    }

    fn mark(&mut self, x: u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.grid[i][j] == x{
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn check_win(&mut self) {
        for i in 0..5 {
            self.won = true;
            for j in 0..5 {
                if !self.marks[i][j]{
                    self.won = false;
                    break;
                }
            }
            if self.won {
                return
            }
        }
        for j in 0..5 {
            self.won = true;
            for i in 0..5 {
                if !self.marks[i][j]{
                    self.won = false;
                    break;
                }
            }
            if self.won {
                return
            }
        }
    }

    fn calc_score(&self, x: u32) -> u32 {
        let mut score = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i][j] {
                    score += self.grid[i][j];
                }
            }
        }
        score * x
    }
}

struct Game {
    cards: Vec<Card>,
    numbys: Vec<u32>
}

impl Game {
    fn play(&mut self) {
        for x in &self.numbys {
            println!("Playing {}", x);
            let mut done = true;
            for (i, c) in self.cards.iter_mut().enumerate() {
                if c.won {
                    continue;
                }
                c.mark(*x);
                c.check_win();

                if c.won {
                    println!("BINGO! -- Card {}", i);
                    let score = c.calc_score(*x);
                    println!("Score: {}", score);
                }
                else {
                    done = false;
                }
            }
            if done {
                break;
            }
        }
    }
}

fn parse_input(filename: &str) -> Game {
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read the file");

    let mut lines = contents.lines();

    let line0 = lines.next().unwrap();

    let numbys: Vec<u32> = line0.split(",").map(
                                |x| x.parse::<u32>().unwrap()).collect();

    let mut cards: Vec<Card> = Vec::new();

    while !lines.next().is_none() {
        let mut grid = [[0; 5]; 5];
        for i in 0..5 {
            let line: Vec<u32> = lines.next().unwrap().split_whitespace()
                                    .map(|x| x.parse::<u32>().unwrap())
                                    .collect();
            for j in 0..5 {
                grid[i][j] = line[j];
            }
        }
        cards.push(Card::new(&grid));
    }

    Game { cards: cards, numbys: numbys}
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut g = parse_input(&filename);

    g.play();
}
