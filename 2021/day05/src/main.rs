use std::env;
use std::fs;
use std::str;
use std::num;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VentLine {
    a: Point,
    b: Point
}

#[derive(Debug, Clone)]
struct Ledger {
    record: HashMap<Point, usize>
}

#[derive(Debug, Clone)]
struct PointIter {
    a: Point,
    b: Point,
    cur: Option<Point>,
    dx: i32,
    dy: i32
}

impl VentLine {
    fn is_hori_or_vert(&self) -> bool {
        if self.a.x == self.b.x || self.a.y == self.b.y {
            return true
        }
        false
    }

    fn pts(&self) -> PointIter {
        PointIter::new(self.a, self.b)
    }
}

impl str::FromStr for Point {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pts: Vec<&str> = s.split(",").collect();

        let x: i32 = pts[0].parse()?;
        let y: i32 = pts[1].parse()?;

        Ok(Point{x: x, y: y})
    }
}

impl str::FromStr for VentLine {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pts: Vec<&str> = s.split_whitespace().collect();

        let a: Point = pts[0].parse()?;
        let b: Point = pts[2].parse()?;

        Ok(VentLine{a: a, b: b})
    }
}

impl PointIter {
    fn new(a: Point, b: Point) -> PointIter {

        let mut dx: i32 = 0;
        if (b.x - a.x).abs() > 0 {
            dx = (b.x - a.x) / (b.x - a.x).abs();
        }
        let mut dy: i32 = 0;
        if (b.y - a.y).abs() > 0 {
            dy = (b.y - a.y) / (b.y - a.y).abs();
        }

        PointIter { a: a, b: b, cur: None, dx: dx, dy: dy }
    }
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Some(pt) => {if pt == self.b {
                            self.cur = None;
                        }
                        else {
                            self.cur = Some(Point{x:pt.x+self.dx,
                                                  y:pt.y+self.dy});
                        }
            },
            None => {self.cur = Some(self.a);}
        }

        self.cur
    }
}

impl Ledger {
    fn new() -> Ledger {
        Ledger { record: HashMap::new() }
    }

    fn add(&mut self, pt: Point) {
        let count = self.record.entry(pt).or_insert(0);
        *count += 1;
    }

    fn count_with_at_least(&self, threshold: usize) -> usize {
        let mut count = 0; 

        for (_, val) in self.record.iter() {
            if *val >= threshold {
                count += 1;
            }
        }

        count
    }
}


fn parse_input(filename: &str) -> Vec<VentLine> {
    let contents = fs::read_to_string(filename)
                        .expect("Couldn't read file");

    let lines = contents.lines().map(|x| x.parse().unwrap()).collect();

    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = parse_input(&filename);

    let mut book1 = Ledger::new();
    let mut book2 = Ledger::new();

    for i in 0..lines.len(){
        if lines[i].is_hori_or_vert() {
            for pt in lines[i].pts() {
                book1.add(pt);
            }
        }
        for pt in lines[i].pts() {
            book2.add(pt);
        }
    }

    let count_hv_tot = book1.count_with_at_least(1);
    let count_hv_danger = book1.count_with_at_least(2);

    println!("Total Vents (Only hor or vert): {}", count_hv_tot);
    println!("Dangerous Vents (Only hor or vert): {}", count_hv_danger);
    
    let count_tot = book2.count_with_at_least(1);
    let count_danger = book2.count_with_at_least(2);

    println!("Total Vents: {}", count_tot);
    println!("Dangerous Vents: {}", count_danger);
}
