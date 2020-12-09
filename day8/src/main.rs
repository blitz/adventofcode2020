#[macro_use] extern crate lazy_static;

use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
use regex::Regex;
use std::io::{self, BufRead};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, Clone)]
struct InstructionParseError;

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "invalid instruction string")
    }
}

impl From<ParseIntError> for InstructionParseError {

    fn from(_: ParseIntError) -> Self {
	InstructionParseError {}
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> std::result::Result<Self, InstructionParseError> {
	lazy_static! {
	    static ref RE: Regex = Regex::new(r"^(\S+) ([+-]\d+)$").expect("a valid regex");
	}

	match RE.captures(s) {
	    Some(c) => {
		let arg = c[2].parse::<i32>()?;

		match &c[1] {
		    "nop" => Ok(Instruction::Nop(arg)),
		    "jmp" => Ok(Instruction::Jmp(arg)),
		    "acc" => Ok(Instruction::Acc(arg)),
		    _ => Err(InstructionParseError)
		}
	    }
	    None => Err(InstructionParseError)
	}
    }
}

fn input_from<T: BufRead>(input: &mut T) -> Vec<String>
{
    input
	.lines()
	.map(|l| l.expect("no I/O error"))
	.collect()
}

fn execute1(instr: &Instruction, ip: i32, acc: i32) -> (i32, i32)
{
    match instr {
	Instruction::Nop(_) => (ip + 1, acc),
	Instruction::Acc(i) => (ip + 1, acc + i),
	Instruction::Jmp(i) => (ip + i, acc),
    }
}

// Execute a program and return the accumulator value.
fn execute(program: &Vec<Instruction>) -> (i32, i32) {
    let mut acc: i32 = 0;
    let mut ip: i32 = 0;
    let mut visited_ip = BTreeSet::<i32>::new();

    while !visited_ip.contains(&ip) && (ip as usize) < program.len() {
	let (new_ip, new_acc) = execute1(&program[ip as usize], ip, acc);

	visited_ip.insert(ip);

	ip = new_ip;
	acc = new_acc;
    }

    (ip, acc)
}

fn variants(program: &Vec<Instruction>) -> Vec<Vec<Instruction>> {
    (0..program.len()).map(|p| {
	let mut mod_program: Vec<Instruction> = program.clone();

	match program[p] {
	    Instruction::Nop(i) => mod_program[p] = Instruction::Jmp(i),
	    Instruction::Jmp(i) => mod_program[p] = Instruction::Nop(i),
	    _ => {}
	}

	mod_program
    }).collect()
}

fn main() {
    let instructions : Vec<Instruction> = input_from(&mut io::stdin().lock())
	.iter()
	.map(|s| s.parse().expect("a valid instruction"))
	.collect();

    println!("acc = {}", execute(&instructions).1);

    let fixed_executions: Vec<i32> = variants(&instructions).iter()
	.map(execute)
	.filter(|(ip, _)| *ip as usize == instructions.len())
	.map(|(_, acc)| acc)
	.collect();

    println!("{:?}", fixed_executions);
}
