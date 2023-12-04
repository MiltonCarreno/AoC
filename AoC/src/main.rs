use core::num;
use std::fs::File;
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut file = File::create("output.txt").unwrap();
    match fs::read_to_string("input.txt") {
        Ok(f) => {
            let mut sum = 0;
            for (idx, line) in f.split('\n').enumerate() {
                println!("\nLine #{idx}: {line}");
                let card_points = 
                    calc_scatch_card(line.to_string());
                sum += card_points;
            }
            // calc value
            println!("\nTotal lines sum: {sum}")
        }
        Err(e) => println!("Error opening file: {e}")
    }
    
}

// Day 4 Part 1
fn calc_scatch_card(line: String) -> usize {
    let nums: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
    let winning_nums: Vec<&str> = nums[0].trim().split(" ").filter(|x| {
        !x.is_empty()
    }).collect();
    let card_nums: Vec<&str> = nums[1].trim().split(" ").filter(|x| {
        !x.is_empty()
    }).collect();

    println!("Winning Nums: {:#?}", winning_nums);
    println!("Card Nums: {:#?}", card_nums);
    let matches: usize = card_nums.iter().map(|x| {
        match winning_nums.contains(x) {
            true => 1,
            false => 0,
        }
    }).sum();
    let base: usize = 2;
    let points = match matches {
        0 => 0,
        1 => 1,
        _ => base.pow(matches as u32 -1),
    };
    println!("Matches: {}", matches);
    println!("Card Points: {}", points);
    return points;
}

// Day 3 Part 2
fn get_row_info2(line: String, row_num: usize
) -> (Vec<char>, Vec<(usize, usize)>, HashMap<String, Vec<Vec<(usize, usize)>>>) {
    let row: Vec<char> = line.chars().collect();
    let mut symbols: Vec<(usize, usize)> = vec![];
    let mut nums: HashMap<String, Vec<Vec<(usize, usize)>>> = HashMap::new();
    let mut c_idx: usize = 0;
    while c_idx < row.len() {
        if row[c_idx].is_numeric() {
            let mut num: String = row[c_idx].to_string();
            let mut digits: Vec<(usize, usize)> = vec![];
            digits.push((row_num, c_idx));

            let mut right = c_idx;
            let mut found_right = false;

            while !found_right {
                if right+1 < row.len() && row[right+1].is_numeric() {
                    num += &row[right+1].to_string();
                    right += 1;
                    digits.push((row_num, right));
                } else {
                    found_right = true;
                }
            }
            if nums.contains_key(&num) {
                let mut val = nums.get(&num).unwrap().to_vec();
                val.push(digits);
                nums.insert(num, val);
            } else {
                let mut val: Vec<Vec<(usize, usize)>> = vec![];
                val.push(digits);
                nums.insert(num, val);
            }
            c_idx = right + 1;
        } else if row[c_idx] == '*' {
            symbols.push((row_num, c_idx));
            c_idx += 1;
        } else {
            c_idx += 1;
        }
    }

    println!("{:#?}", nums);

    return (row, symbols, nums);
}

fn calc_gears(
    nums: HashMap<String, Vec<Vec<(usize, usize)>>>, 
    symbols: Vec<(usize, usize)>
) -> usize {
    let mut sum: usize = 0;
    for (row, col) in symbols {
        let mut adj_nums = vec![];
        for (num, coords) in &nums {
            for val in coords {
                let mut found_symbol =
                    val.contains(&(row, col)) ||
                    val.contains(&(row+1, col)) ||
                    val.contains(&(row+1, col+1)) ||
                    val.contains(&(row, col+1));

                if row > 0 {
                    found_symbol = found_symbol || 
                        val.contains(&(row-1, col)) ||
                        val.contains(&(row-1, col+1));
                }
                if col > 0 {
                    found_symbol = found_symbol ||
                        val.contains(&(row, col-1)) ||
                        val.contains(&(row+1, col-1));
                }
                if row > 0 && col > 0 {
                    found_symbol = found_symbol ||
                        val.contains(&(row-1, col-1));
                }
    
                match found_symbol {
                    true => adj_nums.push(num.clone()),
                    false => (),
                }
            }
        }
        if adj_nums.len() == 2 {
            sum += 
                adj_nums[0].parse::<usize>().unwrap() *
                adj_nums[1].parse::<usize>().unwrap();
        }
    }

    return sum;
}

// Day 3 Part 1
fn get_row_info(line: String, row_num: usize
) -> (Vec<char>, Vec<(usize, usize)>, HashMap<String, Vec<Vec<(usize, usize)>>>) {
    let row: Vec<char> = line.chars().collect();
    let mut symbols: Vec<(usize, usize)> = vec![];
    let mut nums: HashMap<String, Vec<Vec<(usize, usize)>>> = HashMap::new();
    let mut c_idx: usize = 0;
    while c_idx < row.len() {
        if row[c_idx].is_numeric() {
            let mut num: String = row[c_idx].to_string();
            let mut digits: Vec<(usize, usize)> = vec![];
            digits.push((row_num, c_idx));

            let mut right = c_idx;
            let mut found_right = false;

            while !found_right {
                if right+1 < row.len() && row[right+1].is_numeric() {
                    num += &row[right+1].to_string();
                    right += 1;
                    digits.push((row_num, right));
                } else {
                    found_right = true;
                }
            }
            if nums.contains_key(&num) {
                let mut val = nums.get(&num).unwrap().to_vec();
                val.push(digits);
                nums.insert(num, val);
            } else {
                let mut val: Vec<Vec<(usize, usize)>> = vec![];
                val.push(digits);
                nums.insert(num, val);
            }
            c_idx = right + 1;
        } else if row[c_idx] != '.' {
            symbols.push((row_num, c_idx));
            c_idx += 1;
        } else {
            c_idx += 1;
        }
    }

    println!("{:#?}", nums);

    return (row, symbols, nums);
}

fn merge_maps(
    nums: &mut HashMap<String, Vec<Vec<(usize, usize)>>>,
    row: HashMap<String, Vec<Vec<(usize, usize)>>>
) {
    for (num, coords) in row {
        if nums.contains_key(&num) {
            let mut val = nums.get(&num).unwrap().to_vec();
            for v in coords {
                val.push(v);
            }
            nums.insert(num, val);
        } else {
            nums.insert(num, coords);
        }
    }
}

fn calc_parts(
    nums: HashMap<String, Vec<Vec<(usize, usize)>>>, 
    symbols: Vec<(usize, usize)>
) -> usize {
    let mut sum: usize = 0;
    for (num, coords) in nums {
        for val in coords {
            let ans: usize = val.iter().map(|(x,y)| {
                let row = x.to_owned();
                let col = y.to_owned();
                let mut found_symbol =
                    symbols.contains(&(row+1, col)) ||
                    symbols.contains(&(row+1, col+1)) ||
                    symbols.contains(&(row, col+1));

                if row > 0 {
                    found_symbol = found_symbol || 
                        symbols.contains(&(row-1, col)) ||
                        symbols.contains(&(row-1, col+1));
                }
                if col > 0 {
                    found_symbol = found_symbol ||
                        symbols.contains(&(row, col-1)) ||
                        symbols.contains(&(row+1, col-1));
                }
                if row > 0 && col > 0 {
                    found_symbol = found_symbol ||
                        symbols.contains(&(row-1, col-1));
                }
    
                match found_symbol {
                    true => 1,
                    false => 0,
                }
            }).sum();
            
            if ans > 0 {
                sum += num.parse::<usize>().unwrap();
            }
        }
    }

    return sum;
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