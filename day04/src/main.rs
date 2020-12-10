use aocf::Aoc;
use day04::{part_one, part_two};

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(4))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let passports: Vec<&str> = input.split("\n\n").collect();
        
        let answer = part_one(&passports);
        println!("Part I: {}", answer);

        let answer = part_two(&passports);
        println!("Part II: {}", answer);
    }
}

