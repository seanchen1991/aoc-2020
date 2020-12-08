use crate::vm::VM;
use std::str::FromStr;
use std::collections::HashSet;

pub mod vm;

pub fn part_one(input: &str) -> i32 {
    let mut looped = false;
    let mut executed = HashSet::new(); 
    let mut vm = VM::from_str(&input).expect("Invalid list of instructions");

    while !vm.terminated && !looped {
        vm.step();
        looped = !executed.insert(vm.ip);
    }

    vm.accumulator
}

