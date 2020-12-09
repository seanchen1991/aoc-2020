use std::str::FromStr;
use std::num::ParseIntError;

const SPLIT_INDEX: usize = 4;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    NOP,
    ACC(i32),
    JMP(i32),
}

#[derive(Clone, Debug)]
pub struct VM {
    pub ip: i32,
    pub accumulator: i32,
    pub instructions: Vec<Instruction>,
    pub terminated: bool,
}

#[derive(Debug)]
pub enum InstructionError {
    MissingOperation,
    MissingArgument,
    InvalidInstruction(String),
    InvalidArgumentValue(ParseIntError),
}

impl VM {
    pub fn step(&mut self) {
        let mut jmp = 1;

        match self.instructions[self.ip as usize] {
            Instruction::ACC(arg) => self.accumulator += arg,
            Instruction::JMP(arg) => jmp = arg,
            Instruction::NOP      => {},
        }

        self.ip += jmp;
        self.terminated = (self.ip as usize) >= self.instructions.len();
    }
}

impl FromStr for Instruction {
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inst, arg) = s.split_at(SPLIT_INDEX);
        let arg = arg.trim().parse::<i32>().map_err(InstructionError::InvalidArgumentValue)?;

        match inst.trim() {
            "nop" => Ok(Instruction::NOP),
            "acc" => Ok(Instruction::ACC(arg)),
            "jmp" => Ok(Instruction::JMP(arg)),
            op    => Err(InstructionError::InvalidInstruction(op.to_string())),
        }
    }
}

impl FromStr for VM {
    type Err = InstructionError;

    fn from_str(program: &str) -> Result<Self, Self::Err> {
        let instructions = program.lines()
            .map(|inst| Instruction::from_str(inst))
            .collect::<Result<Vec<_>, InstructionError>>()?;

        Ok(VM {
            ip: 0,
            accumulator: 0,
            instructions: instructions,
            terminated: false,
        })
    }
}

#[cfg(test)]
#[test]
fn test_parse_instructions() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    
    let expected = vec![
        Instruction::NOP,
        Instruction::ACC(1),
        Instruction::JMP(4),
        Instruction::ACC(3),
        Instruction::JMP(-3),
        Instruction::ACC(-99),
        Instruction::ACC(1),
        Instruction::JMP(-4),
        Instruction::ACC(6),
    ];

    let vm = VM::from_str(input).expect("Failed to parse input in `test_parse_instructions`");

    assert_eq!(vm.instructions, expected);
}

