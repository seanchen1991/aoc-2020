use std::collections::HashSet;
use crate::vm::{VM, Instruction};

pub mod vm;

pub fn part_one(vm: &mut VM) -> (i32, bool) {
    let mut looped = false;
    let mut executed = HashSet::new(); 

    while !vm.terminated && !looped {
        vm.step();
        looped = !executed.insert(vm.ip);
    }

    (vm.accumulator, looped)
}

pub fn part_two(vm: &VM) -> (i32, bool) {
    vm.instructions.iter()
        .enumerate()
        .map(|(i, _)| {
            let mut vm_clone = vm.clone();

            vm_clone.instructions[i] = match vm_clone.instructions[i] {
                Instruction::JMP(_) => Instruction::NOP,
                Instruction::NOP => Instruction::JMP(1),
                _ => vm_clone.instructions[i],
            };

            part_one(&mut vm_clone)
        })
        .find(|(_, looped)| !*looped)
        .expect("No fix found for the infinite loop")
}

