use aocf::Aoc;
use day09::part_one;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(9))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let answer = part_one(&input);
        println!("{}", answer);
    }
}
