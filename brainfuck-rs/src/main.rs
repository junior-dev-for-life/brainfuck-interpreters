use std::char;

fn main() {
    let program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    interpret(program);
}

fn interpret(program: &str) {
    let mut tape: Vec<u32> = vec![0; 100];
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
                if let Some(c) = char::from_u32(tape[ptr]) {
                    print!("{}", c);
                } else {
                    panic!("Not a valid character");
                }
            },
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