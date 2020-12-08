use aocf::Aoc;
use day07::{part_one, part_two};

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(7))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let answer = part_one(&input);
        println!("Part 1: {}", answer);

        let answer = part_two(&input);
        println!("Part 2: {}", answer);
    }
}
