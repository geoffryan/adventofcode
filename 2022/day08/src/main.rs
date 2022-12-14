mod util;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_1(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let map = TreeMap::new(&input);
        let result = map.count_visible();
        assert_eq!(result, answer);
    }
    
    fn run_2(filename: &str, answer: usize) {
        let input = util::get_input_from_file(filename);
        let map = TreeMap::new(&input);
        let result = map.find_best_score();
        assert_eq!(result, answer);
    }

    fn check_2(filename: &str, i: usize, j: usize, answer: usize) {
        let input = util::get_input_from_file(filename);
        let map = TreeMap::new(&input);
        let result = map.check_score(i, j);
        assert_eq!(result, answer);
    }

    #[test]
    fn test1() {
        run_1("example.txt", 21);
    }
    
    #[test]
    fn test2a() {
        check_2("example.txt", 2, 1, 4);
    }
    
    #[test]
    fn test2b() {
        check_2("example.txt", 2, 3, 8);
    }
    
    #[test]
    fn test2() {
        run_2("example.txt", 8);
    }
}

struct TreeMap {
    nx: usize,
    ny: usize,
    trees: Vec<i32>,
}

impl TreeMap {
    fn new(input: &String) -> TreeMap {
        let mut trees = Vec::new();
        let mut ny = 0;

        for line in input.lines() {
            ny += 1;

            for c in line.chars() {
                trees.push(c.to_digit(10).unwrap() as i32);
            }
        }

        let nx = trees.len() / ny;

        TreeMap { nx: nx, ny: ny, trees: trees }
    }

    fn count_visible(&self) -> usize {

        let mut visible = Vec::new();
        visible.resize(self.nx*self.ny, false);

        let sy = self.ny;
        let sx = 1;

        for i in 0..self.nx {
            let mut lowest = -1;
            for j in 0..self.ny {
                let h = self.trees[sx*i+sy*j];
                if h > lowest {
                    lowest = h;
                    visible[sx*i+sy*j] = true;
                }
            }
            lowest = -1;
            for j in (0..self.ny).rev() {
                let h = self.trees[sx*i+sy*j];
                if h > lowest {
                    lowest = h;
                    visible[sx*i+sy*j] = true;
                }
            }
        }
        
        for j in 0..self.ny {
            let mut lowest = -1;
            for i in 0..self.nx {
                let h = self.trees[sx*i+sy*j];
                if h > lowest {
                    lowest = h;
                    visible[sx*i+sy*j] = true;
                }
            }
            lowest = -1;
            for i in (0..self.nx).rev() {
                let h = self.trees[sx*i+sy*j];
                if h > lowest {
                    lowest = h;
                    visible[sx*i+sy*j] = true;
                }
            }
        }

        let mut count = 0;
        for &v in visible.iter() {
            if v {
                count += 1;
            }
        }
        count
    }

    fn check_score(&self, x: usize, y: usize) -> usize {
        let sx = 1;
        let sy = self.ny;

        let h = self.trees[sx*x+sy*y];

        let mut score = 1;
        
        let mut count = 0;
        
        for i in (x+1)..self.nx {
            count += 1;
            if self.trees[sx*i+sy*y] >= h {
                break;
            }
        }
        score *= count;
        
        count = 0;
        for i in (0..x).rev() {
            count += 1;
            if self.trees[sx*i+sy*y] >= h {
                break;
            }
        }
        score *= count;
        
        count = 0;
        for j in (y+1)..self.ny {
            count += 1;
            if self.trees[sx*x+sy*j] >= h {
                break;
            }
        }
        score *= count;
        
        count = 0;
        for j in (0..y).rev() {
            count += 1;
            if self.trees[sx*x+sy*j] >= h {
                break;
            }
        }
        score *= count;

        score
    }

    fn find_best_score(&self) -> usize {

        let mut best = 0;

        for i in 1..(self.nx-1) {
            for j in 1..(self.ny-1) {
                let score = self.check_score(i, j);
                if score > best {
                    best = score;
                }
            }
        }

        best
    }
}

fn main() {
    let input = util::get_input(2022, 8);

    let map = TreeMap::new(&input);
    let count = map.count_visible();

    println!("There are {} visible trees.", count);

    let best = map.find_best_score();
    
    println!("The best score is {}.", best);
}
