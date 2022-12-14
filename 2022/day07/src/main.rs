mod util;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_a(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let tree = build_tree(&input);
        let result = calc_total_size_lt(&tree, 100_000);
        assert_eq!(result, answer);
    }

    fn run_b(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let tree = build_tree(&input);
        let result = find_best_size(&tree, 70_000_000, 30_000_000);
        assert_eq!(result, answer);
    }

    #[test]
    fn test1() {
        run_a("example.txt", 95437);
    }
    
    #[test]
    fn test2() {
        run_b("example.txt", 24933642);
    }
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

#[derive(Debug)]
struct Dir {
    name: String,
    size: usize,
    contents: Vec<File>,
}

#[derive(Debug)]
struct DirTree {
    dirs: Vec<Dir>,
    children: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>
}

impl File {
    fn new(name: &str, size: usize) -> File {
        File { _name: name.to_string(), size: size}
    }
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir { name: name.to_string(), size: 0, contents: Vec::new()}
    }

    fn add_file(&mut self, f: File) {
        self.contents.push(f);
    }
}

impl DirTree {
    fn new() -> DirTree {
        DirTree { dirs: vec![Dir::new("/")], children: vec![Vec::new()],
                    parents: vec![None] }
    }

    fn add_dir(&mut self, p_idx: usize, d: Dir) -> usize {
        let n = self.dirs.len();
        self.dirs.push(d);
        self.children.push(Vec::new());
        self.parents.push(Some(p_idx));
        self.children[p_idx].push(n);
        n
    }

    fn add_file(&mut self, idx: usize, f: File) {
        self.dirs[idx].add_file(f);
    }

    fn calc_sizes(&mut self, idx: usize) {
        let mut size = 0;
        
        for f in self.dirs[idx].contents.iter() {
            size += f.size;
        }

        let children = self.children[idx].clone();

        for c_idx in children.iter() {
            self.calc_sizes(*c_idx);
            size += self.dirs[*c_idx].size;
        }

        self.dirs[idx].size = size;
    }

    fn navigate(&self, cwd: usize, dest: &str) -> usize{
        if dest == "/" {
            return 0;
        }
        if dest == ".." {
            return self.parents[cwd].unwrap();
        }

        for c_idx in self.children[cwd].iter() {
            if dest == self.dirs[*c_idx].name {
                return *c_idx;
            }
        }

        panic!("destination doesn't exist!");
    }
}

fn build_tree(input: &String) -> DirTree {

    let mut lines = input.lines();

    let mut tree = DirTree::new();
    
    let root_idx  = 0;
    let mut cwd = root_idx;

    let mut line = lines.next().unwrap();

    let mut done = false;

    loop {
        let toks: Vec<&str> = line.split_whitespace().collect();

        //println!("{}", &line);

        if toks[0] != "$" {
            panic!("input not a command!");
        }
       
        if toks[1] == "ls" {
            loop {
                line = match lines.next() {
                    Some(s) => s,
                    None => {done = true; break},
                };
                let out_toks: Vec<&str> = line.split_whitespace().collect();
                if out_toks[0] == "$" {
                    break;
                }

                if out_toks[0] == "dir" {
                    tree.add_dir(cwd, Dir::new(out_toks[1]));
                }
                else {
                    let size: usize = out_toks[0].parse().unwrap();
                    tree.add_file(cwd, File::new(out_toks[1], size));
                }
            }
        }
        else if toks[1] == "cd" {

            cwd = tree.navigate(cwd, toks[2]);

            //println!("now in {}", tree.dirs[cwd].name);

            line = match lines.next() {
                Some(s) => s,
                None => break,
            };
        }
        else {
            panic!("Unknown command");
        }

        if done {
            break;
        }
    }

    tree.calc_sizes(0);

    tree
}


fn calc_total_size_lt(tree: &DirTree, limit: usize) -> usize {

    let mut tot_size = 0;
    for idx in 0..tree.dirs.len() {
        if tree.dirs[idx].size <= limit {
            tot_size += tree.dirs[idx].size;
        }
    }

    tot_size
}

fn find_best_size(tree: &DirTree, cap: usize, needed: usize) -> usize {
    let used = tree.dirs[0].size;

    let avail = cap - used;

    let target = needed - avail;

    let mut best_size = cap;

    for idx in 0..tree.dirs.len() {
        let d_size = tree.dirs[idx].size;

        if d_size >= target && d_size < best_size {
            best_size = d_size;
        }
    }

    println!("Used: {}", used);
    println!("Avail: {}", avail);
    println!("Target: {}", target);
    println!("Best: {}", best_size);

    best_size
}


fn main() {

    let input = util::get_input(2022, 7);
    let tree = build_tree(&input);
    let size_a = calc_total_size_lt(&tree, 100_000);

    println!("Total size <= 100_000: {}", size_a);

    let size_b = find_best_size(&tree, 70_000_000, 30_000_000);

    println!("Best dir to delete has size: {}", size_b);
}
