use aocf::Aoc;
use day10::part_one;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(10))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let joltages: Vec<u32> = input.lines()
            .map(|line| line.parse().expect("Failed to parse a line of input"))
            .collect();

        let answer = part_one(&joltages);
        println!("Part I: {}", answer);
    }
}
