use aocf::Aoc;
use day08::part_one;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(8))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let answer = part_one(&input);
        println!("Part I: {}", answer);
    }
}

