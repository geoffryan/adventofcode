use std::collections::HashSet;
use std::env;
use std::fs;
use std::num;
use std::str;

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize)
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<(usize, usize)>,
    xsize: usize,
    ysize: usize
}

impl Paper {

    fn fold(&mut self, f: &Fold) {
        match *f {
            Fold::X(xf) => self.fold_horizontal(xf),
            Fold::Y(yf) => self.fold_vertical(yf)
        }
    }

    fn fold_horizontal(&mut self, xf: usize) {
        let mut new_dots: HashSet<(usize, usize)> = HashSet::new();

        for &(x, y) in self.dots.iter() {
            if x < xf {
                new_dots.insert((x, y));
            }
            else {
                new_dots.insert((2*xf-x, y));
            }
        }

        self.dots = new_dots;
        self.xsize = xf;
    }
    fn fold_vertical(&mut self, yf: usize) {
        let mut new_dots: HashSet<(usize, usize)> = HashSet::new();

        for &(x, y) in self.dots.iter() {
            if y < yf {
                new_dots.insert((x, y));
            }
            else {
                new_dots.insert((x, 2*yf-y));
            }
        }
        self.dots = new_dots;
        self.ysize = yf;
    }

    fn size(&self) -> usize {
        self.dots.len()
    }

    fn print(&self) {
        let mut c: Vec<Vec<char>> = Vec::new();

        for j in 0..self.ysize {
            c.push(Vec::new());
            for _ in 0..self.xsize {
                c[j].push('.');
            }
        }

        for &(i, j) in self.dots.iter() {
            c[j][i] = '#';
        }

        for j in 0..self.ysize {
            println!("{}", c[j].iter().collect::<String>());
        }
    }
}

impl str::FromStr for Fold {
    
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Fold, Self::Err> {
        let word = s.split_whitespace().last().unwrap();

        let parts: Vec<&str> = word.split("=").collect();
        let dir = parts[0];
        let val: usize = parts[1].parse()?;
        match dir {
            "x" => Ok(Fold::X(val)),
            "y" => Ok(Fold::Y(val)),
            _ => panic!("bad str for Fold")
        }
    }
}

impl str::FromStr for Paper {
    
    type Err = num::ParseIntError;
    
    fn from_str(s: &str) -> Result<Paper, Self::Err> {
        let mut dots: HashSet<(usize, usize)> = HashSet::new();
        let mut xmax: usize = 0;
        let mut ymax: usize = 0;

        for line in s.lines() {
            if line.len() < 2 {
                break;
            }

            let words: Vec<&str> = line.split(",").collect();
            let x: usize = words[0].parse()?;
            let y: usize = words[1].parse()?;
            if x > xmax {
                xmax = x
            }
            if y > ymax {
                ymax = y
            }
            dots.insert((x, y));
        }

        Ok(Paper{dots: dots, xsize: xmax+1, ysize: ymax+1})
    }
}

fn parse_input(filename: &str) -> (Paper, Vec<Fold>) {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let paper: Paper = contents.parse().unwrap();

    let mut folds: Vec<Fold> = Vec::new();

    contents.lines()
        .for_each(|s| if s.len() > 0 && s.chars().nth(0).unwrap() == 'f' {
            folds.push(s.parse().unwrap())});

    (paper, folds)
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (mut paper, folds) = parse_input(&filename);

    paper.print();
    println!("The paper has {} dots.\n", paper.size());
   
    for f in folds.iter() {
        paper.fold(f);
        paper.print();
        println!("The paper has {} dots.\n", paper.size());
    }
        
    paper.print();

}
