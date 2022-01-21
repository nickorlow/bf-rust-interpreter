use std::{fs, usize};
use std::num::Wrapping;
use std::env;
use std::io::Read;
use rand::Rng;
use getch::Getch;
use std::io::stdin;

// The amount of cells available to programs
const SPACE: usize = 30_000;

fn main() {
    let file_name: String = env::args().nth(1).unwrap();
    println!("Loading Brainfuck file: {}\n\n", file_name);
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let mut space : [Wrapping<u8>; SPACE] = [Wrapping(0); SPACE];
    let mut cur_point: usize = 0;
    let mut cur_location: usize = 0;
    let chars: Vec<_> = contents.chars().collect();

    while cur_location < contents.len() {
        let c: char = chars[cur_location];
        match c {
            '>'=> cur_point += 1,
            '<'=> cur_point -= 1,
            '+'=> space[cur_point] += Wrapping(1),
            '-'=> space[cur_point] -= Wrapping(1),
            '.'=> print!("{}", space[cur_point].0 as char),
            ','=> {
                let mut s=String::new();
                stdin().read_line(&mut s);
                space[cur_point] = Wrapping(s.chars().nth(0).unwrap() as u8);
            },
            '[' | ']' => {
                if space[cur_point] == Wrapping(0) || c == ']' {
                    let modifier: i32 = if c == '[' {1} else {-1};
                    let modifier_char: char = if c == ']' {'['} else {']'};
                    let mut ct: i32 = 0;

                    cur_location = add(cur_location, modifier);
                    while ct != 0 || chars[cur_location] != modifier_char {
                        if chars[cur_location] == '[' {
                            ct += 1;
                        } else if chars[cur_location] == ']' {
                            ct -= 1;
                        }
                        cur_location = add(cur_location, modifier);
                    }
                    if c == ']' {cur_location -= 1};
                }
            },
            '*' => {space[cur_point] = rand::thread_rng().gen()},
            '#' => print!("{}", space[cur_point].0),
            _ => { }
        }
        cur_location += 1;
    }
    println!("\n\n");
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}