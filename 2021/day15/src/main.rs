use std::env;
use std::fs;
use std::num;
use std::str;

struct ChitonMap {
    data: Vec<usize>,
    dist: Vec<usize>,
    nx: usize,
    ny: usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize
}

impl ChitonMap {
    fn get_risk(&self, p: Point) -> usize {
        if p.x >= self.nx || p.y >= self.ny {
            return 10000000
        }
        self.data[self.nx*p.y + p.x]
    }
    fn get_dist(&self, p: Point) -> usize {
        if p.x >= self.nx || p.y >= self.ny {
            return 10000000
        }
        self.dist[self.nx*p.y + p.x]
    }
    fn set_dist(&mut self, p: Point, val: usize) {
        if p.x < self.nx && p.y < self.ny {
            self.dist[self.nx*p.y + p.x] = val
        }
    }

    fn get_neighbours(&self, p: Point) -> Vec<Point> {
        let mut neighbours = Vec::new();

        if p.x > 0 {
            neighbours.push(Point{x: p.x-1, y: p.y});
        }
        if p.x < self.nx-1 {
            neighbours.push(Point{x: p.x+1, y: p.y});
        }
        if p.y > 0 {
            neighbours.push(Point{x: p.x, y: p.y-1});
        }
        if p.y < self.ny-1 {
            neighbours.push(Point{x: p.x, y: p.y+1});
        }

        neighbours
    }

    fn find_min_risk(&self) -> usize {

        let mut risk = usize::MAX;
        let start = Point{x: 0, y: 0};
        let end = Point{x: self.nx-1, y: self.ny-1};
        let mut path = vec![start,];
        let mut iterations = 0;

        self.search_step(&mut path, end, 0, &mut risk, &mut iterations);

        println!("Took {} iterations", iterations);

        risk
    }

    fn search_step(&self, path: &mut Vec<Point>, target: Point,
                   current_risk: usize, min_total_risk: &mut usize,
                   iterations: &mut usize)
            -> Option<usize> {
        *iterations += 1;
        let n = path.len();
        let p = path[n-1];

        println!("Iter {}: path {} - loc {:?}", *iterations, n, p);

        if p == target {
            return Some(current_risk);
        }

        let mut found = false;
        let mut current_min = usize::MAX;

        let mut neighbours = self.get_neighbours(p);
        neighbours.sort_by(|&a, &b| 
                           self.get_risk(a)
                           .partial_cmp(&self.get_risk(b)).unwrap());

        for q in neighbours {
            
            if path.contains(&q) {
                continue;
            }
            
            let r = self.get_risk(q);

            if current_risk + r >= *min_total_risk {
                continue;
            }

            path.push(q);

            if let Some(x) = self.search_step(path, target, current_risk+r,
                                              min_total_risk, iterations) {
                found = true;
                if x < current_min {
                    current_min = x;
                }
                if x < *min_total_risk {
                    *min_total_risk = x;
                    println!("Risk: {}", x);
                }
            }

            path.pop();

        }
        
        if found {
            return Some(current_min)
        }

        None
    }

    fn djikstra_search(&mut self, start: Point) {
        for i in 0..(self.nx*self.ny) {
            self.dist[i] = usize::MAX;
        }
        self.set_dist(start, 0);
        
        let mut active = self.get_neighbours(start);
        active.push(start);

        let mut done = Vec::new();

        while active.len() > 0 {
            active.sort_by(|&a, &b| self.get_dist(b).cmp(&self.get_dist(a)));

            let p = active.pop().unwrap();

            for q in self.get_neighbours(p) {
                if done.contains(&q) {
                    continue;
                }

                if !active.contains(&q) {
                    active.push(q);
                }

                let path_dist = self.get_dist(p) + self.get_risk(q);

                if path_dist < self.get_dist(q) {
                    self.set_dist(q, path_dist);
                }
            }
            
            done.push(p);
            println!("Done {} of {}.", done.len(), self.nx*self.ny);
        }
    }

    fn expand(&self, factor: usize) -> ChitonMap {

        let mut risk = Vec::new();
        let nx = factor*self.nx;
        let ny = factor*self.ny;

        for jj in 0..factor {
            for j in 0..self.ny {
                for ii in 0..factor {
                    for i in 0..self.nx {
                        let mut val = self.get_risk(Point{x: i, y: j});
                        val += ii + jj;
                        while val > 9 {
                            val -= 9
                        }
                        risk.push(val);
                    }
                }
            }
        }

        ChitonMap{data: risk, nx: nx, ny: ny, dist: vec![usize::MAX; nx*ny]}
    }
}

impl str::FromStr for ChitonMap {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<ChitonMap, Self::Err> {
        let mut data: Vec<usize> = Vec::new();
        let mut nx = 0;
        let mut ny = 0;
        
        for line in s.lines() {
            nx = line.len();
            ny += 1;
            for c in line.chars() {
                data.push(c.to_string().parse()?);
            }
        }

        Ok(ChitonMap { data: data, nx: nx, ny: ny,
                       dist: vec![usize::MAX; nx*ny]})
    }
}

fn parse_input(filename: &str) -> ChitonMap {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    contents.parse().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut map = parse_input(&filename);

    let start = Point{x: 0, y: 0};
    let end = Point{x: map.nx-1, y: map.ny-1};

    map.djikstra_search(start);
    println!("Minimum risk: {}", map.get_dist(end));

    let mut map2 = map.expand(5);
    let end2 = Point{x: map2.nx-1, y: map2.ny-1};
    map2.djikstra_search(start);
    println!("Minimum risk: {}", map2.get_dist(end2));

    //let risk = map.find_min_risk();
    //println!("Minimum risk: {}", risk);
}
