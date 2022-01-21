use std::fs;
use std::num::Wrapping;
use std::env;

fn main() {
    let file_name: String = env::args().nth(1).unwrap();
    println!("Loading Brainfuck file: {}\n\n", file_name);
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let mut space : [Wrapping<u8>; 30000] = [Wrapping(0); 30000];
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
            ','=> {},
            '['=> if space[cur_point] == Wrapping(0) {
                cur_location += 1;
                let mut ct: i32 = 0;
                while ct != 0 || chars[cur_location] != ']' {
                    if chars[cur_location] == '[' {
                        ct += 1;
                    } else if chars[cur_location] == ']' {
                        ct -= 1;
                    }
                    cur_location += 1;
                }
            },
            ']'=> {
                cur_location -= 1;
                let mut ct: i32 = 0;
                while ct != 0 || chars[cur_location] != '[' {
                    if chars[cur_location] == '[' {
                        ct += 1;
                    } else if chars[cur_location] == ']' {
                        ct -= 1;
                    }
                    cur_location -= 1;
                }
                cur_location -=1;
            },
            _ => { }
        }
        cur_location += 1;
    }
    println!("\n\n");
}
