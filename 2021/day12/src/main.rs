use std::env;
use std::fs;
use std::str;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CaveType {
    Start,
    End,
    Big,
    Small
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cave {
    name: String,
    kind: CaveType
}

#[derive(Clone, Debug)]
struct CaveSystem {
    caves: Vec<Cave>,
    tunnels: Vec<Vec<usize>>
}

impl Cave {
    fn new(s: &str) -> Cave {
        if s == "start" {
            Cave { name: s.to_string(), kind: CaveType::Start }
        }
        else if s == "end" {
            Cave { name: s.to_string(), kind: CaveType::End }
        }
        else if s == s.to_lowercase() {
            Cave { name: s.to_string(), kind: CaveType::Small }
        }
        else {
            Cave { name: s.to_uppercase().to_string(), kind: CaveType::Big }
        }
    }
}

impl CaveSystem {
    fn new() -> CaveSystem {
        CaveSystem { caves: Vec::new(), tunnels: Vec::new()}
    }

    fn get_idx(&self, c: &Cave) -> Option<usize> {
        for i in 0..self.caves.len() {
            if self.caves[i] == *c {
                return Some(i)
            }
        }
       None 
    }

    fn get_start_idx(&self) -> Option<usize> {
        for i in 0..self.caves.len() {
            if let CaveType::Start = self.caves[i].kind {
                return Some(i)
            }
        }
        None
    }

    fn add_tunnel(&mut self, a: Cave, b: Cave) {
        if !self.caves.contains(&a) {
            self.caves.push(a.clone());
            self.tunnels.push(Vec::new());
        }
        if !self.caves.contains(&b) {
            self.caves.push(b.clone());
            self.tunnels.push(Vec::new());
        }

        let idx_a = self.get_idx(&a).unwrap();
        let idx_b = self.get_idx(&b).unwrap();

        self.tunnels.get_mut(idx_a).unwrap().push(idx_b);
        self.tunnels.get_mut(idx_b).unwrap().push(idx_a);
    }

    fn display_path(&self, path: &Vec<usize>) {

        let names: Vec<&str> = path.iter()
                .map(|&i| self.caves[i].name.as_str())
                .collect();

        println!("{}", names.join(","));
    }

    fn traverse(&self, allow_second_visit: bool) -> Vec<Vec<usize>> {
        let start = self.get_start_idx().expect("No start in CaveSystem");

        let mut path: Vec<usize> = Vec::new();
        let mut paths: Vec<Vec<usize>> = Vec::new();

        path.push(start);

        if allow_second_visit {
            self.traverse_step_second(&mut path, false, &mut paths);
        }
        else {
            self.traverse_step(&mut path, &mut paths);
        }

        paths
    }

    fn traverse_step(&self, path: &mut Vec<usize>,
                     paths: &mut Vec<Vec<usize>>) {
        let idx: usize = *path.last().unwrap();
        if let CaveType::End = self.caves[idx].kind {
            return
        }

        for &jdx in self.tunnels[idx].iter() {
            match self.caves[jdx].kind {
                CaveType::Start => {continue},
                CaveType::End => {
                    path.push(jdx);
                    paths.push(path.clone());
                    self.display_path(path);
                    path.pop();
                },
                CaveType::Big => {
                    path.push(jdx);
                    self.traverse_step(path, paths);
                    path.pop();
                },
                CaveType::Small => {
                    if !path.contains(&jdx) {
                        path.push(jdx);
                        self.traverse_step(path, paths);
                        path.pop();
                    }
                },
            }
        }
    }

    fn traverse_step_second(&self, path: &mut Vec<usize>,
                            used_second: bool,
                            paths: &mut Vec<Vec<usize>>) {
        let idx: usize = *path.last().unwrap();
        if let CaveType::End = self.caves[idx].kind {
            return
        }

        for &jdx in self.tunnels[idx].iter() {
            match self.caves[jdx].kind {
                CaveType::Start => {continue},
                CaveType::End => {
                    path.push(jdx);
                    paths.push(path.clone());
                    self.display_path(path);
                    path.pop();
                },
                CaveType::Big => {
                    path.push(jdx);
                    self.traverse_step_second(path, used_second, paths);
                    path.pop();
                },
                CaveType::Small => {
                    if !path.contains(&jdx) {
                        path.push(jdx);
                        self.traverse_step_second(path, used_second, paths);
                        path.pop();
                    }
                    else if !used_second {
                        path.push(jdx);
                        self.traverse_step_second(path, true, paths);
                        path.pop();
                    }
                },
            }
        }
    }
}

impl str::FromStr for Cave {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Cave, Self::Err> {
        Ok(Cave::new(s))
    }
}

impl str::FromStr for CaveSystem {
    
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<CaveSystem, Self::Err> {
        let mut caves = CaveSystem::new();

        for line in s.lines() {
            let words: Vec<&str> = line.split('-').collect();
            caves.add_tunnel(words[0].parse()?, words[1].parse()?);
        }

        Ok(caves)
    }
}

fn parse_input(filename: &str) -> CaveSystem {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    contents.parse().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let cs = parse_input(&filename);

    println!("{:?}", &cs);

    let paths = cs.traverse(false);
    let paths2 = cs.traverse(true);

    println!("Found {} paths.", paths.len());
    println!("Found {} second paths.", paths2.len());
}
