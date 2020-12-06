use aocf::Aoc;
use std::collections::HashSet;

fn two_sum(v: Vec<i32>, target: i32) -> (i32, i32) {
    let mut set = HashSet::new();    

    for val in v {
        let complement = target - val;

        if set.contains(&complement) {
            return (val, complement);
        } else {
            set.insert(val);
        }
    }

    (-1, -1)
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(1))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        // Part I
        let expenses: Vec<i32> = input.lines()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let (x, y) = two_sum(expenses, 2020);
        let answer = x * y;
        
        println!("{}", answer);
    }
}
