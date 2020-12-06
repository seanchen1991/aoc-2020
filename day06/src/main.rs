use aocf::Aoc;
use std::collections::HashSet;

fn part_one(input: &str) -> usize {
    input.split("\n\n")
        .fold(0, |acc, line| {
            acc + line.chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(6))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let answer = part_one(&input);
        println!("{}", answer);
    }
}

#[cfg(test)]
#[test]
fn test_part_one() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a
b";
    assert_eq!(part_one(input), 11);
}
