use std::fs::File;
use std::fs;

fn main() {
    let mut file = File::create("output.txt").unwrap();
    match fs::read_to_string("input.txt") {
        Ok(f) => {
            let mut sum = 0;
            for (i, l) in f.split('\n').enumerate() {
                println!("\nLine #{i}: {l}");
                let n = check_cubes_2(l.to_string());
                println!("Num in line: {n}");
                sum += n;
            }
            println!("\nTotal lines sum: {sum}")
        }
        Err(e) => println!("Error opening file: {e}")
    }
    
}

// Day 2 Part 2
fn check_cubes_2(line: String) -> i32 {
    let line_info: Vec<&str> = line.split(":").collect();
    let samples = line_info[1].to_string();
    
    let mut red = 1;
    let mut green = 1;
    let mut blue = 1;
    for sample in samples.split(";") {
        let _cubes: Vec<_> = sample.split(",").map(
            |x| {
                let cube: Vec<&str> = x.trim().split(" ").collect();
                let color = cube[1];
                let num = cube[0].to_string().parse::<i32>().unwrap();
                match color {
                    "red" if num > red => red = num,
                    "green" if num > green => green = num,
                    "blue" if num > blue => blue = num,
                    _ => (),
                }
            }
        ).collect();
    }

    return red * green * blue;
}

// Day 2 Part 1
static RED: i32 = 12;
static GREEN: i32 = 13;
static BLUE: i32 = 14;

fn check_cubes(line: String) -> i32 {
    let line_info: Vec<&str> = line.split(":").collect();
    let game_num = line_info[0].to_string();
    let samples = line_info[1].to_string();
    
    let mut sum: i32 = 0;
    for sample in samples.split(";") {
        let cubes: i32 = sample.split(",").map(
            |x| {
                let cube: Vec<&str> = x.trim().split(" ").collect();
                let color = cube[1];
                let num = cube[0].to_string().parse::<i32>().unwrap();
                match color {
                    "red" if num > RED => 1,
                    "green" if num > GREEN => 1,
                    "blue" if num > BLUE => 1,
                    _ => 0,
                }
            }
        ).sum();
        sum += cubes;
    }

    if sum == 0 {
        let n: Vec<&str>  = game_num.split(" ").collect();
        return n[1].parse::<i32>().unwrap();
    }
    return 0;
}

// Day 1 Part 1

fn get_num(line: String) -> i32 {
    let nums: Vec<char> = line.chars()
        .filter(|x| x.is_numeric()).collect();

    let num = nums[0].to_string() + &nums[nums.len() - 1].to_string();
    return num.parse::<i32>().unwrap();
}

// Day 1 Part 2

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