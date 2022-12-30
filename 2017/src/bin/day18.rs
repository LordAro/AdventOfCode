use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Machine<'a> {
    pc: i64,
    program: &'a Vec<(String, char, String)>,
    regs: HashMap<char, i64>,
    sender: mpsc::Sender<i64>,
    receiver: mpsc::Receiver<i64>,
}

impl<'a> Machine<'a> {
    fn new(
        program: &'a Vec<(String, char, String)>,
        sender: mpsc::Sender<i64>,
        receiver: mpsc::Receiver<i64>,
    ) -> Machine<'a> {
        Machine {
            pc: 0,
            program,
            regs: HashMap::new(),
            sender,
            receiver,
        }
    }

    fn get_val(&mut self, val: &str) -> i64 {
        val.parse()
            .unwrap_or(*self.regs.entry(val.chars().next().unwrap()).or_insert(0))
    }

    fn get_val_c(&mut self, val: char) -> i64 {
        if val.is_ascii_digit() {
            val.to_digit(10).unwrap() as i64
        } else {
            *self.regs.entry(val).or_insert(0)
        }
    }

    fn insert(&mut self, reg: char, val: i64) {
        self.regs.insert(reg, val);
    }

    fn run(&mut self) -> usize {
        let mut send_count = 0;
        while self.pc >= 0 && (self.pc as usize) < self.program.len() {
            let ins = &self.program[self.pc as usize];
            let val = match ins.0.as_ref() {
                "snd" | "rcv" | "jgz" => self.get_val_c(ins.1),
                _ => self.get_val(&ins.2),
            };
            match ins.0.as_ref() {
                "snd" => {
                    send_count += 1;
                    self.sender.send(val).unwrap();
                }
                "set" => {
                    self.regs.insert(ins.1, val);
                }
                "add" => *self.regs.entry(ins.1).or_insert(0) += val,
                "mul" => *self.regs.entry(ins.1).or_insert(0) *= val,
                "mod" => *self.regs.entry(ins.1).or_insert(0) %= val,
                "rcv" => {
                    if let Ok(res) = self.receiver.recv_timeout(Duration::from_millis(1)) {
                        *self.regs.entry(ins.1).or_insert(0) = res;
                    } else {
                        return send_count;
                    }
                }
                "jgz" => {
                    if val > 0 {
                        self.pc += self.get_val(&ins.2) - 1;
                    }
                }
                _ => unreachable!(),
            }
            self.pc += 1;
        }
        send_count
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input: Vec<(String, char, String)> =
        BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
            .lines()
            .map(|l| {
                let v: Vec<_> = l
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                (
                    v[0].clone(),
                    v[1].chars().next().unwrap(),
                    if v.len() == 3 {
                        v[2].clone()
                    } else {
                        "".to_string()
                    },
                )
            })
            .collect();

    // Part 1
    let (sender1, receiver1) = mpsc::channel();
    let (_, receiver2) = mpsc::channel();
    {
        let input = input.clone();
        thread::spawn(move || {
            let mut machine = Machine::new(&input, sender1, receiver2);
            machine.run();
        });
    }
    let mut last_val = 0;
    while let Ok(val) = receiver1.recv_timeout(Duration::from_millis(1)) {
        last_val = val;
    }
    println!("Recovered sound: {}", last_val);

    let (sender3, receiver3) = mpsc::channel();
    let (sender4, receiver4) = mpsc::channel();
    {
        let input = input.clone();
        thread::spawn(move || {
            let mut machine = Machine::new(&input, sender3, receiver4);
            machine.insert('p', 0);
            machine.run();
        });
    }
    let mut machine1 = Machine::new(&input, sender4, receiver3);
    machine1.insert('p', 1);
    let sends = machine1.run();
    println!("Program 1 send count: {}", sends);
}
