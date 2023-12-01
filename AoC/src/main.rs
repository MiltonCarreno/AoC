use std::fs::File;
use std::fs;
use std::io::Write;

fn main() {
    let mut file = File::create("output.txt").unwrap();
    match fs::read_to_string("input.txt") {
        Ok(f) => {
            let mut sum = 0;
            for (i, l) in f.split('\n').enumerate() {
                println!("\nLine #{i}: {l}");
                let n = get_num_2(l.to_string());
                println!("Num in line: {n}");
                sum += n;
            }
            println!("\nTotal lines sum: {sum}")
        }
        Err(e) => println!("Error opening file: {e}")
    }
    
}

fn get_num(line: String) -> i32 {
    let nums: Vec<char> = line.chars()
        .filter(|x| x.is_numeric()).collect();

    let num = nums[0].to_string() + &nums[nums.len() - 1].to_string();
    return num.parse::<i32>().unwrap();
}

fn get_num_2(line: String) -> i32 {
    let mut nums = vec![];
    let chars: Vec<char> = line.as_bytes()
        .iter().map(|x| *x as char).collect();

    for (idx, ch) in chars.iter().enumerate() {
        match ch {
            _ if ch.is_numeric() => nums.push(ch),
            'o' if idx+3 <= chars.len() => {
                match chars[idx..idx+3] {
                    ['o', 'n', 'e'] => nums.push(&'1'),
                    _ => (),
                }
            },
            't' if idx+3 <= chars.len() && chars[idx+1] == 'w' => {
                match chars[idx..idx+3] {
                    ['t', 'w', 'o'] => nums.push(&'2'),
                    _ => (),
                }
            },
            't' if idx+5 <= chars.len() && chars[idx+1] == 'h' => {
                match chars[idx..idx+5] {
                    ['t', 'h', 'r', 'e', 'e'] => nums.push(&'3'),
                    _ => (),
                }
            },
            'f' | 'n' if idx+4 <= chars.len() => {
                match chars[idx..idx+4] {
                    ['f', 'o', 'u', 'r'] => nums.push(&'4'),
                    ['f', 'i', 'v', 'e'] => nums.push(&'5'),
                    ['n', 'i', 'n', 'e'] => nums.push(&'9'),
                    _ => (),
                }
            },
            's' if idx+3 <= chars.len() && chars[idx+1] == 'i' => {
                match chars[idx..idx+3] {
                    ['s', 'i', 'x'] => nums.push(&'6'),
                    _ => (),
                }
            },
            's' if idx+5 <= chars.len() && chars[idx+1] == 'e' => {
                match chars[idx..idx+5] {
                    ['s', 'e', 'v', 'e', 'n'] => nums.push(&'7'),
                    _ => (),
                }
            },
            'e' if idx+5 <= chars.len() => {
                match chars[idx..idx+5] {
                    ['e', 'i', 'g', 'h', 't'] => nums.push(&'8'),
                    _ => (),
                }
            },
            _ => ()
        }
    }

    let num = nums[0].to_string() + &nums[nums.len() - 1].to_string();
    return num.parse::<i32>().unwrap();
}