use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PathSeg {
    p: Point,
    d: usize,
    prev: Option<Point>,
}

#[derive(Debug)]
struct ElevationMap {
    height: Vec<i64>,
    nx: usize,
    ny: usize,
    start: Point,
    end: Point,
}

impl PathSeg {
    fn new(p: Point) -> PathSeg {
        PathSeg {p: p, d: usize::MAX, prev: None }
    }
}

impl Ord for PathSeg {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d)
            .then_with(|| self.p.cmp(&other.p))
            .then_with(|| self.prev.cmp(&other.prev))
    }
}

impl PartialOrd for PathSeg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ElevationMap {
    
    fn new(input: &str) -> ElevationMap {
        let mut height = Vec::new();

        let mut s = Point { x: 0, y: 0};
        let mut e = Point { x: 0, y: 0};

        let mut nx = 0;

        let mut y = 0;

        for line in input.lines() {
            let mut x = 0;
            for c in line.chars() {
                if c == 'S' {
                    s = Point { x: x, y: y }
                }
                else if c == 'E' {
                    e = Point { x: x, y: y }
                }

                height.push(ElevationMap::decode_elevation(c));

                x += 1;
            }
            nx = x as usize;
            y += 1;
        }

        let ny = y as usize;

        ElevationMap { height: height, nx: nx, ny: ny, start: s, end: e }
    }

    fn decode_elevation(c: char) -> i64 {
        if c == 'S' {
            return -1;
        }
        else if c == 'E' {
            return 26;
        }

        ((c as u32) - 97) as i64
    }

    fn get_height(&self, p: Point) -> i64 {
        if p.x < 0 || p.x >= self.nx as i64
                || p.y < 0 || p.y >= self.ny as i64 {
            return i64::MAX;
        }
        self.height[(p.y as usize)*self.nx + (p.x as usize)]
    }

    fn get_neighbours(&self, p: &Point) -> Vec<Point> {
        let h = self.get_height(*p);

        let mut n = Vec::new();
        let mut q = Point { x: p.x - 1, y: p.y };
        if self.get_height(q) <= h+1 {
            n.push(q); 
        }
        q.x += 2;
        if self.get_height(q) <= h+1 {
            n.push(q); 
        }
        q.x -= 1;
        q.y -= 1;
        if self.get_height(q) <= h+1 {
            n.push(q); 
        }
        q.y += 2;
        if self.get_height(q) <= h+1 {
            n.push(q); 
        }

        n
    }

    fn points(&self) -> Vec<Point> {
        let mut p = Vec::new();

        for y in 0..(self.ny as i64) {
            for x in 0..(self.nx as i64) {
                p.push(Point{x: x, y: y});
            }
        }

        p
    }

    fn display_path(&self, path: &Vec<Point>) -> String {

        let mut disp = Vec::new();
        for j in 0..self.ny {
            disp.push(Vec::new());
            for _ in 0..self.nx {
                disp[j].push('.');
            }
            disp[j].push('\n');
        }

        for idx in 0..path.len()-1 {
            let p = path[idx];
            let q = path[idx+1];

            let i = p.x as usize;
            let j = p.y as usize;

            if q.x > p.x {
                disp[j][i] = '>';
            }
            else if q.x < p.x {
                disp[j][i] = '<';
            }
            else if q.y > p.y {
                disp[j][i] = 'v';
            }
            else if q.y < p.y {
                disp[j][i] = '^';
            }
            else {
                disp[j][i] = '#'
            }
        }

        let e = path[path.len()-1];

        disp[e.y as usize][e.x as usize] = 'E';

        disp.iter().map(|line| line.iter().collect::<String>()).collect()
    }
}

fn find_shortest_path(map: &ElevationMap, start: &Point, end: &Point) 
        -> Option<Vec<Point>> {

    //Djikstra it up

    let mut distances = HashMap::new();
    for &p in map.points().iter() {
        distances.insert(p, PathSeg::new(p));
    }

    *distances.get_mut(&start).unwrap() = PathSeg {
        p: *start, d: 0, prev: None };

    let mut heap = BinaryHeap::new();

    heap.push(PathSeg { p: *start, d: 0, prev: None });

    while let Some(current) = heap.pop() {

        if current.p == *end {
            break;
        }

        if current.d > distances.get(&current.p).unwrap().d {
            continue;
        }

        let neighbours = map.get_neighbours(&current.p);

        for &n in neighbours.iter() {
            let ps = PathSeg { p: n, d: current.d+1, prev: Some(current.p) };

            if ps > *distances.get(&n).unwrap() {
                *distances.get_mut(&n).unwrap() = ps;
                heap.push(ps);
            }
        }
    }

    if distances.get(end).unwrap().prev == None {
        return None
    }

    let mut rpath = Vec::new();

    rpath.push(*end);

    while let Some(p) = distances.get(&rpath[rpath.len()-1]).unwrap().prev {
        rpath.push(p);
    }

    Some(rpath.into_iter().rev().collect())
}

fn find_best_hiking_path(map: &ElevationMap, start_sym: char, end: &Point) 
        -> Vec<Point> {
    
    let mut best_len = usize::MAX;
    let mut best_path = Vec::new();

    let start_elev = ElevationMap::decode_elevation(start_sym);

    for p in map.points().iter() {
        if map.get_height(*p) != start_elev {
            continue;
        }
        if let Some(path) = find_shortest_path(map, p, end) {

            let len = path.len();

            print!("{}", map.display_path(&path));

            println!("This path has length {}\n", len-1);

            if len < best_len {
                best_path = path;
                best_len = len;
            }
        }
    }

    best_path
}


fn main() {

    let input = util::get_input(2022, 12);
    let map = ElevationMap::new(&input);

    let path = find_shortest_path(&map, &map.start, &map.end).unwrap();
    print!("{}", map.display_path(&path));

    println!("Shortest path has length: {}", path.len()-1);

    let hike_path = find_best_hiking_path(&map, 'a', &map.end);
    print!("\n\n{}", map.display_path(&hike_path));

    println!("Best hike has length: {}", hike_path.len()-1);

}
