#![feature(str_split_once)]
use either::Either;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Inst {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
struct UnrecognizedInstError;

impl FromStr for Inst {
    type Err = Either<UnrecognizedInstError, std::num::ParseIntError>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inst, arg) = s
            .split_once(' ')
            .ok_or(Either::Left(UnrecognizedInstError))?;
        let arg = arg.parse().map_err(Either::Right)?;
        match inst {
            "acc" => Ok(Inst::Acc(arg)),
            "jmp" => Ok(Inst::Jmp(arg)),
            "nop" => Ok(Inst::Nop(arg)),
            _ => Err(Either::Left(UnrecognizedInstError)),
        }
    }
}

struct Emulator {
    acc: isize,
    pc: isize,
}

impl Emulator {
    fn init() -> Self {
        Self { acc: 0, pc: 0 }
    }

    fn step(&mut self, program: &[Inst]) {
        match program[self.pc as usize] {
            Inst::Acc(arg) => {
                self.acc += arg;
                self.pc += 1;
            }
            Inst::Jmp(arg) => {
                self.pc += arg;
            }
            Inst::Nop(_) => {
                self.pc += 1;
            }
        }
    }

    fn should_terminate(&self, program: &[Inst]) -> bool {
        self.pc as usize == program.len()
    }
}

lazy_static! {
    static ref PROGRAM: Vec<Inst> = include_str!("input.txt")
        .lines()
        .flat_map(|line| line.parse())
        .collect();
}

enum RunResult {
    Looped(isize),
    Terminated(isize),
}

fn run_until_loop_or_termination(program: &[Inst]) -> RunResult {
    let mut instructions_run = BTreeSet::new();
    let mut emu = Emulator::init();
    while !emu.should_terminate(program) {
        if !instructions_run.insert(emu.pc) {
            return RunResult::Looped(emu.acc);
        }
        emu.step(program);
    }
    RunResult::Terminated(emu.acc)
}

fn part1() {
    if let RunResult::Looped(res) = run_until_loop_or_termination(&PROGRAM) {
        println!("*   {}", res);
    }
}

fn part2() {
    for (i, inst) in PROGRAM.iter().enumerate() {
        let mut program = PROGRAM.clone();
        match *inst {
            Inst::Jmp(arg) => program[i] = Inst::Nop(arg),
            Inst::Nop(arg) => program[i] = Inst::Jmp(arg),
            _ => continue,
        }
        if let RunResult::Terminated(res) = run_until_loop_or_termination(&program) {
            println!("**  {}", res);
            return;
        }
    }
}

fn main() {
    part1();
    part2();
}
