mod util;

fn calc_priority(item: char) -> i64 {
    let mut val = item as i64;

    if val <= 90 {
        val += -65 + 27;
    }
    else {
        val += -97 + 1;
    }

    val
}

fn find_match(comp1: &str, comp2: &str) -> char {

    for c1 in comp1.chars() {
        if comp2.contains(c1) {
            return c1
        }
    }
    unreachable!("There wasn't a matching char");
}

fn main() {

    let rucksacks = util::get_input(2022, 3);

    let mut sum = 0;

    for sack in rucksacks.lines() {
        let n = sack.len();
        let item = find_match(&sack[0..n/2], &sack[n/2..n]);
        sum += calc_priority(item);
    }
    
    println!("Total priority of shared items: {}", sum);

    let sacks: Vec<String> = rucksacks.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string()).collect();
    let num_groups = sacks.len() / 3;
    
    let mut badge_sum = 0;

    for i in 0..num_groups {

        let sack1 = &sacks[3*i];
        let sack2 = &sacks[3*i+1];
        let sack3 = &sacks[3*i+2];

        let badges: Vec<char> = sack1.chars()
            .filter(|&c| sack2.contains(c))
            .filter(|&c| sack3.contains(c))
            .collect();

        badge_sum += calc_priority(badges[0]);
    }
    
    println!("Total priority of badges: {}", badge_sum);
}
