use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};

type Word = isize;

#[derive(PartialEq, Debug)]
enum Op {
    Add = 1,
    Mul,
    Input,
    Output,
    JNZ,
    JZ,
    LessThan,
    Equal,
    SetRel,
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
            x if x == Op::SetRel as Word => Ok(Op::SetRel),
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
            Op::SetRel => 2,
            Op::Halt => 1,
        }
    }
}

#[derive(PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<Word> for Mode {
    type Error = ();

    fn try_from(v: Word) -> Result<Self, Self::Error> {
        match v {
            x if x == Mode::Position as Word => Ok(Mode::Position),
            x if x == Mode::Immediate as Word => Ok(Mode::Immediate),
            x if x == Mode::Relative as Word => Ok(Mode::Relative),
            _ => Err(()),
        }
    }
}

fn get_op_mode(op: Word, offset: usize) -> Mode {
    ((op / (10_isize.pow(offset as u32 + 1))) % 10)
        .try_into()
        .expect(&format!("Invalid op mode: {}", op))
}

pub struct Machine {
    program: Vec<Word>,
    ptr: usize,
    inputs: VecDeque<Word>,
    relative_base: Word,
}

#[derive(PartialEq)]
pub enum RunRetVal {
    Halted,
    NeedsInput,
    Output(Word),
}

impl Machine {
    pub fn new(program: &[Word], inputs: &[Word]) -> Machine {
        Machine {
            program: program.to_vec(),
            inputs: VecDeque::from(inputs.to_vec()),
            ptr: 0,
            relative_base: 0,
        }
    }

    pub fn run(&mut self) -> RunRetVal {
        loop {
            let res = self.step();
            if res.is_some() {
                return res.unwrap();
            }
        }
    }

    pub fn run_until_output(&mut self) -> Option<Word> {
        let ret = self.run();
        match ret {
            RunRetVal::Halted => None,
            RunRetVal::NeedsInput => panic!("Program requires input!"),
            RunRetVal::Output(w) => Some(w),
        }
    }

    fn step(&mut self) -> Option<RunRetVal> {
        let cur_op = self.program[self.ptr];
        let opcode: Op = (cur_op % 100)
            .try_into()
            .expect(&format!("Invalid opcode {}@{}", cur_op, self.ptr));
        let mut jumped = false;
        let mut output = None;
        //println!(
        //    "{:?}: {:?}",
        //    opcode,
        //    &self.program[self.ptr..self.ptr + opcode.size()]
        //);
        match opcode {
            Op::Add => {
                let val = self.get_value(cur_op, 1) + self.get_value(cur_op, 2);
                self.set_value(cur_op, 3, val);
            }
            Op::Mul => {
                let val = self.get_value(cur_op, 1) * self.get_value(cur_op, 2);
                self.set_value(cur_op, 3, val);
            }
            Op::Input => {
                if self.inputs.is_empty() {
                    return Some(RunRetVal::NeedsInput);
                }
                let input = self.inputs.pop_front().unwrap();
                self.set_value(cur_op, 1, input);
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
                self.set_value(cur_op, 3, if lt { 1 } else { 0 });
            }
            Op::Equal => {
                let eq = self.get_value(cur_op, 1) == self.get_value(cur_op, 2);
                self.set_value(cur_op, 3, if eq { 1 } else { 0 });
            }
            Op::SetRel => {
                self.relative_base += self.get_value(cur_op, 1);
            }
            Op::Halt => {
                return Some(RunRetVal::Halted);
            }
        };
        if !jumped {
            self.ptr += opcode.size();
        }
        if output.is_some() {
            return Some(RunRetVal::Output(output.unwrap()));
        } else {
            return None;
        }
    }

    fn set_value(&mut self, cur_op: Word, offset: usize, value: Word) {
        let param = self.program[self.ptr + offset];
        match get_op_mode(cur_op, offset) {
            Mode::Position => self.set_memory(param as usize, value),
            Mode::Immediate => panic!("Attempted to set in immediate mode"),
            Mode::Relative => self.set_memory((self.relative_base + param) as usize, value),
        }
    }

    fn get_value(&self, cur_op: Word, offset: usize) -> Word {
        let param = self.program[self.ptr + offset];
        match get_op_mode(cur_op, offset) {
            Mode::Position => self.get_memory(param as usize),
            Mode::Immediate => param,
            Mode::Relative => self.get_memory((self.relative_base + param) as usize),
        }
    }

    pub fn set_memory(&mut self, idx: usize, val: Word) {
        if idx >= self.program.len() {
            self.program.resize(idx + 1, 0);
        }
        self.program[idx] = val;
    }

    pub fn get_memory(&self, idx: usize) -> Word {
        if idx < self.program.len() {
            self.program[idx]
        } else {
            0
        }
    }

    pub fn push_input(&mut self, value: Word) {
        self.inputs.push_back(value);
    }
}

pub fn read_input(in_str: &str) -> Vec<Word> {
    in_str
        .split(',')
        .map(|n| Word::from_str_radix(n, 10).unwrap())
        .collect()
}
