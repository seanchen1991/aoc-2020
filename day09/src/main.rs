use aocf::Aoc;
use day09::{part_one, part_two};

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(9))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let nums: Vec<i64> = input.lines()
            .map(|n| n.parse::<i64>().expect("Failed to parse a line of input"))
            .collect();

        let answer = part_one(&nums);
        println!("{}", answer);

        let answer = part_two(&nums, answer);
        println!("{}", answer);
    }
}
