use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};

type Word = isize;

#[derive(PartialEq)]
enum Op {
    Add = 1,
    Mul,
    Input,
    Output,
    JNZ,
    JZ,
    LessThan,
    Equal,
    Halt = 99,
}

impl TryFrom<Word> for Op {
    type Error = ();

    fn try_from(v: Word) -> Result<Self, Self::Error> {
        match v {
            x if x == Op::Add as Word => Ok(Op::Add),
            x if x == Op::Mul as Word => Ok(Op::Mul),
            x if x == Op::Input as Word => Ok(Op::Input),
            x if x == Op::Output as Word => Ok(Op::Output),
            x if x == Op::JNZ as Word => Ok(Op::JNZ),
            x if x == Op::JZ as Word => Ok(Op::JZ),
            x if x == Op::LessThan as Word => Ok(Op::LessThan),
            x if x == Op::Equal as Word => Ok(Op::Equal),
            x if x == Op::Halt as Word => Ok(Op::Halt),
            _ => Err(()),
        }
    }
}

impl Op {
    fn size(&self) -> usize {
        match self {
            Op::Add => 4,
            Op::Mul => 4,
            Op::Input => 2,
            Op::Output => 2,
            Op::JNZ => 3,
            Op::JZ => 3,
            Op::LessThan => 4,
            Op::Equal => 4,
            Op::Halt => 1,
        }
    }
}

fn is_immediate(op: Word, offset: usize) -> bool {
    (op / (10_isize.pow(offset as u32 + 1))) % 10 != 0
}

pub struct Machine {
    program: Vec<Word>,
    ptr: usize,
    inputs: VecDeque<Word>,
}

impl Machine {
    pub fn new(program: &[Word], inputs: &[Word]) -> Machine {
        Machine {
            program: program.to_vec(),
            inputs: VecDeque::from(inputs.to_vec()),
            ptr: 0,
        }
    }

    pub fn run(&mut self) -> Option<Word> {
        loop {
            let (res, halted) = self.step();
            if halted || res.is_some() {
                return res;
            }
        }
    }

    fn step(&mut self) -> (Option<Word>, bool) {
        let cur_op = self.program[self.ptr];
        let opcode: Op = (cur_op % 100)
            .try_into()
            .expect(&format!("Invalid opcode {}@{}", cur_op, self.ptr));
        let mut jumped = false;
        let mut output = None;
        match opcode {
            Op::Add => {
                let val = self.get_value(cur_op, 1) + self.get_value(cur_op, 2);
                self.set_value(3, val);
            }
            Op::Mul => {
                let val = self.get_value(cur_op, 1) * self.get_value(cur_op, 2);
                self.set_value(3, val);
            }
            Op::Input => {
                let input = self.inputs.pop_front().unwrap();
                self.set_value(1, input);
            }
            Op::Output => {
                output = Some(self.get_value(cur_op, 1));
            }
            Op::JNZ => {
                if self.get_value(cur_op, 1) != 0 {
                    self.ptr = self.get_value(cur_op, 2) as usize;
                    jumped = true;
                }
            }
            Op::JZ => {
                if self.get_value(cur_op, 1) == 0 {
                    self.ptr = self.get_value(cur_op, 2) as usize;
                    jumped = true;
                }
            }
            Op::LessThan => {
                let lt = self.get_value(cur_op, 1) < self.get_value(cur_op, 2);
                self.set_value(3, if lt { 1 } else { 0 });
            }
            Op::Equal => {
                let eq = self.get_value(cur_op, 1) == self.get_value(cur_op, 2);
                self.set_value(3, if eq { 1 } else { 0 });
            }
            Op::Halt => {}
        };
        if !jumped {
            self.ptr += opcode.size();
        }
        return (output, opcode == Op::Halt);
    }

    fn set_value(&mut self, offset: usize, value: Word) {
        let param = self.program[self.ptr + offset];
        self.program[param as usize] = value;
    }

    fn get_value(&self, cur_op: Word, offset: usize) -> Word {
        let param = self.program[self.ptr + offset];
        if is_immediate(cur_op, offset) {
            param
        } else {
            self.program[param as usize]
        }
    }

    pub fn get_memory(&self, idx: usize) -> Word {
        self.program[idx]
    }
}

pub fn read_input(in_str: &str) -> Vec<Word> {
    in_str
        .split(',')
        .map(|n| Word::from_str_radix(n, 10).unwrap())
        .collect()
}
