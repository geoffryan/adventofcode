use std::env;
use std::fs;

struct HeightMap {
    height: Vec<i32>,
    ng: usize,
    nx: usize,
    ny: usize
}

struct LowPointIter<'a> {
    heightmap: &'a HeightMap,
    i_cur: Option<i64>,
    j_cur: Option<i64>
}

impl HeightMap {
    fn new(h: Vec<i32>, nx: usize, ny: usize) -> HeightMap {
        let mut height: Vec<i32> = Vec::new();
        let nx_tot: usize = nx + 2;
        let high = 100;
        for i in 0..nx_tot {
            height.push(high);
        }
        for j in 0..ny {
            height.push(high);
            for i in 0..nx {
                height.push(*h.get(nx*j+i).unwrap());
            }
            height.push(high);
        }
        for i in 0..nx_tot {
            height.push(high);
        }

        HeightMap { height: height, ng: 1, nx: nx, ny: ny }
    }

    fn iter_low(&self) -> LowPointIter {
        LowPointIter { heightmap: &self, i_cur: None, j_cur: None }
    }

    fn get(&self, i: i64, j: i64) -> i32 {
        let nx_tot = (self.nx + 2*self.ng) as i64;
        let ng = self.ng as i64;
        
        *self.height.get(((j+ng)*nx_tot + i+ng) as usize).unwrap()
    }

    fn measure_basin(&self, i0: i64, j0: i64) -> usize {

        let mut sites: Vec<(i64, i64)> = Vec::new();

        let mut active: Vec<(i64, i64)> = Vec::new();

        active.push((i0, j0));

        while active.len() > 0 {
            let s = active.pop().unwrap();

            if sites.contains(&s) || self.get(s.0, s.1) >= 9 {
                continue
            }

            sites.push(s);
            active.push((s.0-1, s.1));
            active.push((s.0+1, s.1));
            active.push((s.0, s.1-1));
            active.push((s.0, s.1+1));
        }

        sites.len()
    }
}

impl<'a> Iterator for LowPointIter<'a> {
    type Item = (i32, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {

        let (mut i0, j0) = match (self.i_cur, self.j_cur) {
            (Some(x), Some(y)) => (x, y),
            _ => (0, 0)
        };

        i0 += 1;

        for j in j0..(self.heightmap.ny as i64) {
            for i in i0..(self.heightmap.nx as i64) {

                let v = self.heightmap.get(i, j);
                let vl = self.heightmap.get(i-1, j);
                let vr = self.heightmap.get(i+1, j);
                let vb = self.heightmap.get(i, j-1);
                let vt = self.heightmap.get(i, j+1);

                if (v < vl) && (v < vr) && (v < vt) && (v < vb) {
                    self.i_cur = Some(i);
                    self.j_cur = Some(j);
                    return Some((v, i, j));
                }
            }
            if j == j0 {
                i0 = 0;
            }
        }

        self.i_cur = None;
        self.j_cur = None;
        None
    }
}

fn parse_input(filename: &str) -> HeightMap {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let mut h: Vec<i32> = Vec::new();

    let mut nx: usize = 0;
    let mut ny: usize = 0;

    for line in contents.lines() {
        nx = line.len();
        ny += 1;

        for c in line.chars() {
            h.push( c.to_string().parse().unwrap() );
        }
    }

    HeightMap::new(h, nx, ny)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let hm = parse_input(&filename);

    let mut total_risk = 0;

    let mut sizes: Vec<usize> = Vec::new();

    for lp in hm.iter_low() {
        let (v, i, j) = lp;
        total_risk += v+1;

        let basin_size = hm.measure_basin(i, j);
        println!("Basin Size: {}", basin_size);
        sizes.push(basin_size);
    }

    sizes.sort();
    let n = sizes.len();

    let s3 = sizes.get(n-3).unwrap();
    let s2 = sizes.get(n-2).unwrap();
    let s1 = sizes.get(n-1).unwrap();

    println!("Total Risk: {}", total_risk);
    println!("Biggest basins: {} {} {}", s3, s2, s1);
    println!("Basin score: {}", s3*s2*s1);
}
