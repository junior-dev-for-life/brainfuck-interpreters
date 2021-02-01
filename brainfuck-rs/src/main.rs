use std::char;
use std::io;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let program: String = contents.chars().filter(|c| !c.is_whitespace()).collect();
    interpret(&program);
}

fn interpret(program: &str) {
    let mut tape: Vec<i32> = vec![0; 100];
    let mut stack: Vec<usize> = vec![];
    let mut ptr: usize = 0;
    let mut is_looping = false;
    let mut inner_loops = 0;
    
    let chars: Vec<char> = program.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        if is_looping {
            match c {
                '[' => inner_loops = inner_loops + 1,
                ']' => {
                    if inner_loops == 0 {
                        is_looping = false;
                    } else {
                        inner_loops = inner_loops - 1;
                    }
                },
                _ => ()
            }
            continue;
        }

        match c {
            '+' => tape[ptr] = tape[ptr] + 1,
            '-' => tape[ptr] = tape[ptr] - 1,
            '>' => ptr = ptr + 1,
            '<' => ptr = ptr - 1,
            '.' => {
                if let Some(c) = char::from_u32(tape[ptr] as u32) {
                    print!("{}", c);
                } else {
                    panic!("Not a valid character");
                }
            },
            ',' => {
                let input = read_input();
                tape[ptr] = input;
            }
            '[' => {
                if tape[ptr] != 0 {
                    stack.push(i);
                } else {
                    is_looping = true;
                }
            },
            ']' => {
                if tape[ptr] != 0 {
                    if let Some(n) = stack.pop() {
                        i = n;
                        continue;
                    } else {
                        panic!("] found without appropriate ]");
                    }
                } else {
                    stack.pop();
                }
            }
            _ => panic!("Error")
        }

        i = i + 1;
    }
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read user input");
    let trimmed = input.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(_) => panic!("Error reading user input")
    }
}