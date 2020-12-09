use aocf::Aoc;
use std::str::FromStr;
use day08::{
    vm::VM,
    part_one, 
    part_two
};

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(8))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let mut vm = VM::from_str(&input).expect("Invalid set of instructions");

        let (answer, _) = part_one(&mut vm.clone());
        println!("Part I: {}", answer);

        let (answer, _) = part_two(&mut vm);
        println!("Part II: {}", answer);
    }
}

