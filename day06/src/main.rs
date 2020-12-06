use aocf::Aoc;
use counter::Counter;
use std::collections::HashSet;

fn part_one(input: &str) -> usize {
    input.split("\n\n")
        .fold(0, |acc, group| {
            acc + group.chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
}

fn part_two(input: &str) -> usize {
    input.split("\n\n")
        .fold(0, |acc, group| {
            let nlines = group.split('\n').count();
            let char_counts = group.split('\n')
                .flat_map(|s| s.chars())
                .collect::<Counter<_>>();

            acc + char_counts.values()
                .filter(|v| **v == nlines)
                .count()
        })
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(6))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let answer = part_one(&input.trim());
        println!("{}", answer);

        let answer = part_two(&input.trim());
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

#[test]
fn test_part_two() {
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
    assert_eq!(part_two(input), 6);

    let input = "kn
nk
nuk";
    assert_eq!(part_two(input), 2);
}
