use std::io::Error;
use std::io::prelude::*;
use std::io;
use std::fs::File;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Forwards(usize),
    Backwards(usize),
    Increment(u8),
    Decrement(u8),
    LoopStart(usize),
    LoopEnd(usize),
    Output,
    Input,
    Skip,
}

impl Instruction {
    fn from_sym_rep(sym: char, rep: usize) -> Instruction {
        use Instruction::*;
        match sym {
            '<' => Backwards(rep),
            '>' => Forwards(rep),
            '+' => Increment(rep as u8),
            '-' => Decrement(rep as u8),
            _ => unreachable!(),
        }
    }

    fn is_start(&self) -> bool {
        match *self {
            Instruction::LoopStart(_) => true,
            _ => false,
        }
    }
}

fn open_file(name: &str, s: &mut String) -> Result<usize, Box<Error>> {
    let mut f = File::open(name)?;
    let r = f.read_to_string(s)?;
    Ok(r)
}

#[inline(always)]
fn forwards(ptr: &mut usize, rep: usize) {
    *ptr += rep;
}

#[inline(always)]
fn backwards(ptr: &mut usize, rep: usize) {
    *ptr -= rep;
}

#[inline(always)]
fn increment(cell: &mut u8, rep: u8) {
    *cell = cell.wrapping_add(rep);
}

#[inline(always)]
fn decrement(cell: &mut u8, rep: u8) {
    *cell = cell.wrapping_sub(rep);
}

#[inline(always)]
fn input(cell: &mut u8) {
    *cell = io::stdin()
        .bytes()
        .next()
        .and_then(|res| res.ok())
        .expect("Failed to read a byte. Time to change careers.");
}

fn output(cell: &u8) {
    print!("{}", *cell as char);
}

fn reduce_similar(tape: &[char]) -> usize {
    let mut idx = 1;
    while tape[idx] == tape[0] {
        idx += 1;
    }
    idx
}

fn parse(src: &str) -> Vec<Instruction> {
    use Instruction::*;

    let tape = src.chars().collect::<Vec<char>>();
    let mut instructions: Vec<Instruction> = Vec::with_capacity(src.len());
    let mut idx: usize = 0;
    let mut open_loops = Vec::new();

    // TODO: Catch-all should skip invalid instruction
    while idx != src.len() {
        let ins = match tape[idx] {
            s @ '<' | s @ '>' | s @ '+' | s @ '-' => {
                let rep = reduce_similar(&tape[idx..]);
                idx += rep;
                idx -= 1;
                Instruction::from_sym_rep(s, rep)
            }
            '[' => {
                open_loops.push(instructions.len());
                LoopStart(0)
            }
            ']' => {
                let matching_brace = open_loops.pop().unwrap();
                // assert!(instructions[matching_brace].is_start());
                instructions[matching_brace] = LoopStart(instructions.len());
                LoopEnd(matching_brace)
            }
            '.' => Output,
            ',' => Input,
            _ => Skip,
        };
        idx += 1;
        instructions.push(ins);
        // println!("idx: {} {:?}", instructions.len(), open_loops);
        // println!("{:?}", instructions);
    }
    instructions
}


fn main() {
    let mut source = String::new();
    match open_file("mandelbrot.bf", &mut source) {
        Ok(_) => {}
        Err(e) => println!("Error opening file: {}", e),
    }

    let instructions = parse(&source);
    // println!("{:?}", instructions);

    let mut i_ptr: usize = 0;
    let mut d_ptr: usize = 0;
    let mut data = [0u8; 30_000];


    // TODO: Get rid of Skip variant
    while i_ptr != instructions.len() {
        use Instruction::*;
        match instructions[i_ptr] {
            Forwards(rep) => forwards(&mut d_ptr, rep),
            Backwards(rep) => backwards(&mut d_ptr, rep),
            Increment(rep) => increment(&mut data[d_ptr], rep),
            Decrement(rep) => decrement(&mut data[d_ptr], rep),
            LoopStart(matching) => {
                if data[d_ptr] == 0 {
                    i_ptr = matching;
                }
            }
            LoopEnd(matching) => {
                if data[d_ptr] != 0 {
                    i_ptr = matching;
                }
            }
            Input => input(&mut data[d_ptr]),
            Output => output(&data[d_ptr]),
            Skip => {}          
        }
        i_ptr += 1;
    }
}
