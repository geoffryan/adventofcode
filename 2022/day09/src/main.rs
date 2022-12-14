use std::cmp;
use std::collections::HashSet;

mod util;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_1(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let path = run_path(&input, 1);
        let result = count_unique(&path);
        assert_eq!(result, answer);
    }

    fn run_2(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let path = run_path(&input, 9);
        let result = count_unique(&path);
        assert_eq!(result, answer);
    }
    
    #[test]
    fn test1() {
        run_1("example.txt", 13);
    }
    
    #[test]
    fn test2a() {
        run_2("example.txt", 1);
    }
    
    #[test]
    fn test2b() {
        run_2("example_b.txt", 36);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    p: Vec<Point>,
    size: usize,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Move {
    fn parse(s: &str) -> (Move, usize) {
        let tok: Vec<&str> = s.split_whitespace().collect();

        let count = tok[1].parse().unwrap();

        if tok[0] == "U" {
            return (Move::Up, count);
        }
        else if tok[0] == "D" {
            return (Move::Down, count);
        }
        else if tok[0] == "L" {
            return (Move::Left, count);
        }
        else if tok[0] == "R" {
            return (Move::Right, count);
        }

        panic!("Move not UDLR");
    }
}

impl Point {
    fn new(x: i32, y:i32) -> Point {
        Point{ x: x, y: y}
    }
}

impl Rope {
    fn new(size: usize) -> Rope {
        let mut p = Vec::new();
        for _ in 0..(size+1) {
            p.push(Point::new(0, 0));
        }
        Rope { p: p, size: size}
    }

    fn do_move(&mut self, m: &Move) {
        match m {
            Move::Left => {
                self.p[0].x -= 1;
                self.respond();
            },
            Move::Right => {
                self.p[0].x += 1;
                self.respond();
            },
            Move::Up => {
                self.p[0].y += 1;
                self.respond();
            },
            Move::Down => {
                self.p[0].y -= 1;
                self.respond();
            },
        }
    }

    fn length(&mut self, seg: usize) -> i32 {
        cmp::max((self.p[seg+1].x-self.p[seg].x).abs(),
                 (self.p[seg+1].y-self.p[seg].y).abs())
    }

    fn respond(&mut self) {
        for seg in 0..self.size {
            self.respond_at(seg);
        }
    }

    fn respond_at(&mut self, seg: usize) {
        if self.length(seg) <= 1 {
            return;
        }

        if self.p[seg].x < self.p[seg+1].x
                && self.p[seg].y < self.p[seg+1].y {
            self.p[seg+1].x -= 1;
            self.p[seg+1].y -= 1;
        }
        else if self.p[seg].x < self.p[seg+1].x 
                && self.p[seg].y == self.p[seg+1].y {
            self.p[seg+1].x -= 1;
        }
        else if self.p[seg].x < self.p[seg+1].x 
                && self.p[seg].y > self.p[seg+1].y {
            self.p[seg+1].x -= 1;
            self.p[seg+1].y += 1;
        }
        else if self.p[seg].x == self.p[seg+1].x 
                && self.p[seg].y > self.p[seg+1].y {
            self.p[seg+1].y += 1;
        }
        else if self.p[seg].x > self.p[seg+1].x 
                && self.p[seg].y > self.p[seg+1].y {
            self.p[seg+1].x += 1;
            self.p[seg+1].y += 1;
        }
        else if self.p[seg].x > self.p[seg+1].x 
                && self.p[seg].y == self.p[seg+1].y {
            self.p[seg+1].x += 1;
        }
        else if self.p[seg].x > self.p[seg+1].x 
                && self.p[seg].y < self.p[seg+1].y {
            self.p[seg+1].x += 1;
            self.p[seg+1].y -= 1;
        }
        else {
            self.p[seg+1].y -= 1;
        }
    }

    fn tail(&self) -> Point {
        let t = self.size;
        Point::new(self.p[t].x, self.p[t].y)
    }
}

fn run_path(input: &String, size: usize) -> Vec<Point> {

    let mut r = Rope::new(size);

    let mut path = Vec::new();
    path.push(r.tail());

    for line in input.lines() {

        let (m, count) = Move::parse(line);

        for _ in 0..count {
            r.do_move(&m);
            path.push(r.tail());
        }
    }

    path
}

fn count_unique(path: &Vec<Point>) -> usize {
    let mut sites = HashSet::new();

    for &s in path.iter() {
        sites.insert(s);
    }

    sites.len()
}


fn main() {
    let input = util::get_input(2022, 9);

    let path = run_path(&input, 1);
    let site_count = count_unique(&path);

    println!("Total sites visited (len 1): {}", site_count);
    
    let path = run_path(&input, 9);
    let site_count = count_unique(&path);

    println!("Total sites visited (len 9): {}", site_count);
}
