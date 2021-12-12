use std::env;
use std::fs;
use std::num;
use std::str;

#[derive(Debug, Clone, Copy)]
enum Status {
    Awake,
    Charged,
    Tired
}

#[derive(Debug, Clone)]
struct FireflySquid {
    energy: usize,
    status: Status
}

struct Squad {
    frands: Vec<FireflySquid>,
    nx: usize,
    ny: usize
}

impl FireflySquid {
    fn new(energy: usize) -> FireflySquid {
        if energy > 9 {
            FireflySquid{energy: 10, status: Status::Charged}
        }
        else{
            FireflySquid{energy: energy, status: Status::Awake}
        }
    }
}

impl str::FromStr for FireflySquid {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FireflySquid::new(s.parse()?))
    }
}

impl FireflySquid {
    fn charge(&mut self) {
        if let Status::Awake = self.status {
            self.energy += 1;
            if self.energy == 10 {
                self.status = Status::Charged;
            }
        }
    }

    fn flash(&mut self) -> bool{
        if let Status::Charged = self.status {
            self.status = Status::Tired;
            true
        }
        else {
            false
        }
    }

    fn recover(&mut self)
    {
        if let Status::Tired = self.status {
            self.energy = 0;
            self.status = Status::Awake;
        }
    }
}

impl Squad {

    fn get(&self, i: usize, j: usize) -> &FireflySquid {
        self.frands.get(j*self.nx + i).unwrap()
    }
    
    fn get_mut(&mut self, i: usize, j: usize) -> &mut FireflySquid {
        self.frands.get_mut(j*self.nx + i).unwrap()
    }

    fn display(&self) {
        for j in 0..self.ny {
            let mut line: String = String::from("");
            for i in 0..self.nx {
                line += &format!("{}", self.get(i, j).energy);
            }
            println!("{}", line);
        }
    }

    fn display_status(&self) {
        for j in 0..self.ny {
            let mut line: String = String::from("");
            for i in 0..self.nx {
                match self.get(i, j).status {
                    Status::Awake => line += "o",
                    Status::Charged => line += "*",
                    Status::Tired => line += "-",
                }
            }
            println!("{}", line);
        }
    }

    fn step(&mut self) -> usize {

        let mut flashes = 0;

        for bib in self.frands.iter_mut() {
            bib.charge();
        }

        //println!("");
        //self.display_status();

        let mut active: Vec<(usize, usize)> = Vec::new();

        for i in 0..self.nx {
            for j in 0..self.ny {
                if let Status::Charged = self.get(i, j).status {
                    active.push((i, j));
                }
            }
        }
        
        //println!("{}", active.len());

        while active.len() > 0 {
            let (i, j) = active.pop().unwrap();

            let bib = self.get_mut(i, j);
            
            if bib.flash() {
                flashes += 1;

                let mut neighbours: Vec<(usize, usize)> = Vec::new();
                for dx in (-1 as i64)..2 {
                    for dy in (-1 as i64)..2 {
                        let k = (i as i64) + dx;
                        let l = (j as i64) + dy;
                        if k >= 0 && k < self.nx as i64
                                && l >= 0 && l < self.ny as i64 {
                            neighbours.push((k as usize, l as usize));
                        }
                    }
                }

                for &n in neighbours.iter() {
                    self.get_mut(n.0, n.1).charge();
                    active.push((n.0, n.1));
                }
            }
        }
        
        //println!("");
        //self.display_status();
        
        for bib in self.frands.iter_mut() {
            bib.recover();
        }
        
        //println!("");
        //self.display_status();


        flashes
    }
}

impl str::FromStr for Squad {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut frands: Vec<FireflySquid> = Vec::new();
        let mut nx = 0;
        let mut ny = 0;

        for line in s.lines() {
            nx = line.len();
            ny += 1;

            for c in line.chars() {
                let bib: FireflySquid = c.to_string().parse()?;
                frands.push(bib);
            }
        }
        Ok(Squad {frands: frands, nx: nx, ny: ny})
    }
}

fn parse_input(filename: &str) -> Squad {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    contents.parse().unwrap()
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut school = parse_input(&filename);

    let mut total_flashes = 0;
    let mut flashes = 0;
    let mut step = 0;
    school.display();
    while flashes != school.nx * school.ny {
        flashes = school.step();
        step += 1;
        println!("");
        school.display();

        if step <= 100 {
            total_flashes += flashes;
        }
    }

    println!("There were {} total flashes before step 100",
             total_flashes);
    println!("Synchronized at step: {}", step);
}
