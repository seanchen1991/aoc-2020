use aocf::Aoc;
use day10::{part_one, part_two};

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(10))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let mut joltages: Vec<usize> = input.lines()
            .map(|line| line.parse().expect("Failed to parse a line of input"))
            .collect();

        joltages.push(0);
        joltages.sort();
        joltages.push(joltages.last().unwrap() + 3);

        let answer = part_one(&joltages);
        println!("Part I: {}", answer);

        let answer = part_two(&joltages);
        println!("Part II: {}", answer);
    }
}

