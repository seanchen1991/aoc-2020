use aocf::Aoc;
use day03::part_one;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(3))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let map: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        let answer = part_one(&map, (3, 1));
        println!("Part I: {}", answer);
    }
}
