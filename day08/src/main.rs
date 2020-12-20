use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Op {
    fn parse(s: String) -> Op {
        let ss: Vec<&str> = s.split(" ").collect();
        let param: i32 = ss.get(1).unwrap().parse().unwrap();
        let opcode = *ss.get(0).unwrap();
        match opcode {
            "nop" => Op::Nop(param),
            "acc" => Op::Acc(param),
            "jmp" => Op::Jmp(param),
            _ => panic!("unsupported opcode"),
        }
    }
}

struct Cpu {
    acc: i32,
    pos: usize,
    program: Vec<Op>,
}

impl Cpu {
    fn new(program: Vec<Op>) -> Cpu {
        Cpu {
            acc: 0,
            pos: 0,
            program,
        }
    }

    fn step(&mut self) {
        let op = self.program.get(self.pos).unwrap();
        self.pos += 1;
        match op {
            Op::Acc(value) => self.acc += value,
            Op::Jmp(value) => self.pos = (self.pos as i32 + value - 1) as usize,
            _ => {}
        }
    }

    fn run(&mut self) -> usize {
        let mut executed = vec![false; self.program.len()];
        while self.pos < self.program.len() && !executed[self.pos] {
            let p = self.pos;
            self.step();
            executed[p] = true;
        }
        self.pos
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Specify a file to run on");
    }

    let file = File::open(args.get(1).unwrap()).unwrap();

    let program: Vec<Op> = BufReader::new(file)
        .lines()
        .filter_map(|r| match r {
            Ok(line) => Some(Op::parse(line)),
            _ => None,
        })
        .collect();

    let mut cpu = Cpu::new(program.clone());
    cpu.run();

    println!("Part 1: acc: {}", cpu.acc);

    for i in 0..program.len() {
        let fixed = match program.get(i).unwrap() {
            Op::Nop(value) => {
                let mut fixed = program.clone();
                fixed[i] = Op::Jmp(*value);
                fixed
            }
            Op::Jmp(value) => {
                let mut fixed = program.clone();
                fixed[i] = Op::Nop(*value);
                fixed
            }
            Op::Acc(_) => program.clone(),
        };

        let mut cpu = Cpu::new(fixed);
        let exit = cpu.run();
        if exit == program.len() {
            println!("Part 2: Fixed by modifying pos {} - acc: {}", i, cpu.acc);
            break;
        }
    }
}
