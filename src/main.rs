use std::collections::HashMap;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    bf(contents.chars().collect());
}

fn bf(prog: Vec<char>) {
    let mut tape: [i32;255] = [0;255];
    let mut cell_index = 0;
    let mut ip = 0; //index pointer
    let mut loop_table = HashMap::new();
    let mut loop_stack: Vec<usize> = vec![];
    let mut loop_begining_index: usize;
    let mut input = String::new();
    let mut inp = std::collections::VecDeque::new();
    let mut _b1;

    while ip < prog.len() {
        let instruction = prog[ip];
        if instruction == '[' {
            loop_stack.push(ip)
        } else if instruction == ']' {
            loop_begining_index = loop_stack.pop().unwrap();
            loop_table.insert(loop_begining_index, ip);
            loop_table.insert(ip, loop_begining_index);
        }
        ip += 1;
    }

    ip = 0;

    while ip < prog.len() {
        let instruction = prog[ip];

        match instruction {
            '+' => (tape[cell_index] += 1),
            '-' => (tape[cell_index] -= 1),
            '>' => {
                cell_index += 1;
                if cell_index >= 256 {
                    cell_index = 0
                }
            },
            '<' => {
                if cell_index != 0 {
                    cell_index -= 1;
                }
                else{
                    cell_index = 255;
                }
            },
            '[' => {
                if tape[cell_index] == 0 {
                    ip = *loop_table.get(&ip).unwrap()
                }
            },
            ']' => {
                if tape[cell_index] != 0 {
                    ip = *loop_table.get(&ip).unwrap()
                }
            },
            '.' => (print!("{}", tape[cell_index] as u8 as char)),
            ',' => {
                {
                    if inp.is_empty() {
                        _b1 = std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read Input");
                        inp = input.chars().collect()
                    }

                    tape[cell_index] = inp.pop_front().unwrap() as u8 as i32;
                }
            },
            _ => (),
        }

        ip += 1;
    }
}
