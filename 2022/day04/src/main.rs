mod util;

fn main() {
    let pairs = util::get_input(2022, 4);

    let totally_contained = pairs.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(&[',', '-'][..])
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>())
        .filter(|v| (v[2] >= v[0] && v[3] <= v[1]) 
                    || (v[0] >= v[2] && v[1] <= v[3]))
        .count();

    println!("Number of totally contained pairs: {}",
            totally_contained);
    
    let partially_contained = pairs.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(&[',', '-'][..])
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>())
        .filter(|v| !(v[1] < v[2] || v[3] < v[0]))
        .count();
    
    println!("Number of partially contained pairs: {}",
            partially_contained);
    
}
