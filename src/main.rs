use std::io::Error;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::collections::HashMap;

fn open_file(name: &str, s: &mut String) -> Result<usize, Box<Error>> {
    let mut f = File::open(name)?;
    let r = f.read_to_string(s)?;
    Ok(r)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Forwards,
    Backwards,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

fn parse_to_instruction(c: &char) -> Option<Instruction> {
    use Instruction::*;
    match *c {
        '>' => Some(Forwards),
        '<' => Some(Backwards),
        '+' => Some(Increment),
        '-' => Some(Decrement),
        '.' => Some(Output),
        ',' => Some(Input),
        '[' => Some(LoopStart),
        ']' => Some(LoopEnd),
        _ => None,
    }
}

fn forwards(ptr: &mut usize) {
    *ptr += 1;
}

fn backwards(ptr: &mut usize) {
    *ptr -= 1;
}

fn increment(cell: &mut u8) {
    *cell += 1;
}

fn decrement(cell: &mut u8) {
    *cell -= 1;
}

fn input(cell: &mut u8) {
    *cell = io::stdin()
        .bytes()
        .next()
        .and_then(|res| res.ok())
        .expect("Failed to read a byte");
}

fn output(cell: &u8) {
    print!("{}", *cell as char);
}

fn parse(source: &str) -> (Vec<Instruction>, HashMap<usize, usize>) {
	let mut instructions: Vec<Instruction> = Vec::with_capacity(source.len());
	let mut loop_map = HashMap::new();
	let mut idx_stack = Vec::new();

    for chr in source.chars() {
        if let Some(ins) = parse_to_instruction(&chr) {
            instructions.push(ins);
            if ins == Instruction::LoopStart {
            	idx_stack.push(instructions.len() - 1);
            } else if ins == Instruction::LoopEnd {
            	let start = idx_stack.pop().unwrap();
            	let end = instructions.len() - 1;
            	loop_map.insert(start, end);
            	loop_map.insert(end, start);
            }
        }
    }
    (instructions, loop_map)
}

fn main() {
    let mut source = String::new();
    match open_file("mandelbrot.bf", &mut source) {
        Ok(_) => {}
        Err(e) => println!("Error opening file: {}", e),
    }

    let(instructions, loop_map) = parse(&source);

    let mut i_ptr: usize = 0;
    let mut d_ptr: usize = 0;
    let mut data = [0u8; 30_000];

    while i_ptr != instructions.len() {
        use Instruction::*;
        match instructions[i_ptr] {
            Forwards => forwards(&mut d_ptr),
            Backwards => backwards(&mut d_ptr),
            Increment => increment(&mut data[d_ptr]),
            Decrement => decrement(&mut data[d_ptr]),
            Input => input(&mut data[d_ptr]),
            Output => output(&data[d_ptr]),
            LoopStart => {
                if data[d_ptr] == 0 {
                    i_ptr = loop_map[&i_ptr];
                }
            }
            LoopEnd => {
                if data[d_ptr] != 0 {
                	i_ptr = loop_map[&i_ptr];
                }
            }
        }
        i_ptr += 1;
    }
}
