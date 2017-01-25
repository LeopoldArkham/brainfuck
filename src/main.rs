use std::io::Error;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Forwards(usize),
    Backwards(usize),
    Increment(u8),
    Decrement(u8),
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

fn open_file(name: &str, s: &mut String) -> Result<usize, Box<Error>> {
    let mut f = File::open(name)?;
    let r = f.read_to_string(s)?;
    Ok(r)
}

fn forwards(ptr: &mut usize, rep: usize) {
    *ptr += rep;
}

fn backwards(ptr: &mut usize, rep: usize) {
    *ptr -= rep;
}

fn increment(cell: &mut u8, rep: u8) {
    *cell += rep;
}

fn decrement(cell: &mut u8, rep: u8) {
    *cell -= rep;
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

fn parse(src: &str) -> (Vec<Instruction>, HashMap<usize, usize>) {
    use Instruction::*;

    let source = src.chars().collect::<Vec<_>>();
    let mut instructions: Vec<Instruction> = Vec::with_capacity(source.len());
    let mut loop_map = HashMap::new();
    let mut idx_stack = Vec::new();
    let mut idx: usize = 0;

    while idx != source.len() {
        let cur = source[idx];

        let ins = match cur {
            '>' | '<' | '+' | '-' => {
                let mut local_idx = idx + 1;
                while source[local_idx] == cur {
                    local_idx += 1;
                }
                let t = match cur {
                    '>' => {
                        let i = local_idx - idx;
                        Some(Forwards(i))
                    }
                    '<' => {
                        let i = local_idx - idx;
                        Some(Backwards(i))
                    }
                    '+' => {
                        let i = local_idx - idx;
                        Some(Increment((i) as u8))
                    }
                    '-' => {
                        let i = local_idx - idx;
                        Some(Decrement((i) as u8))
                    }
                    _ => None,
                };
                idx = local_idx - 1;
                t
            }
            '.' => Some(Output),
            ',' => Some(Input),
            '[' => Some(LoopStart),
            ']' => Some(LoopEnd),
            _ => None,
        };

        if let Some(instruction) = ins {
            instructions.push(instruction);
        }

        if ins == Some(LoopStart) {
            idx_stack.push(instructions.len() - 1);
        } else if ins == Some(LoopEnd) {
            let start = idx_stack.pop().unwrap();
            let end = instructions.len() - 1;
            loop_map.insert(start, end);
            loop_map.insert(end, start);
        }
        idx += 1;
    }
    (instructions, loop_map)
}

fn main() {
    let mut source = String::new();
    match open_file("mandelbrot.bf", &mut source) {
        Ok(_) => {}
        Err(e) => println!("Error opening file: {}", e),
    }

    let (instructions, loop_map) = parse(&source);

    let mut i_ptr: usize = 0;
    let mut d_ptr: usize = 0;
    let mut data = [0u8; 30_000];

    while i_ptr != instructions.len() {
        use Instruction::*;
        match instructions[i_ptr] {
            Forwards(rep) => forwards(&mut d_ptr, rep),
            Backwards(rep) => backwards(&mut d_ptr, rep),
            Increment(rep) => increment(&mut data[d_ptr], rep),
            Decrement(rep) => decrement(&mut data[d_ptr], rep),
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
