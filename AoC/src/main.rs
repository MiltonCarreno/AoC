use std::fs::File;
use std::fs;
use std::collections::HashMap;

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(f) => {
            let lines: Vec<&str> = f.split('\n')
                .filter(|x| !x.is_empty()).collect();
        
            let mut num_ways: usize = 1;
            let mut time: String = "".to_string();
            let mut distance: String = "".to_string();
            for (idx, &line) in lines.iter().enumerate() {
                // println!("\nLine #{idx}: {line}");
                match line.to_string().starts_with("Time:") {
                    true => {
                        time = line
                            .split(":").nth(1).unwrap().split(" ")
                            .filter(|x| !x.is_empty())
                            .fold("".to_string(), |acc, x| acc + x);
        
                    },
                    false => {                        
                        distance = line
                            .split(":").nth(1).unwrap().split(" ")
                            .filter(|x| !x.is_empty())
                            .fold("".to_string(), |acc, x| acc + x);
                    
        
                    },
                }
            }
            // println!("times: {:#?}", time);
            // println!("distance: {:#?}", distance);
            calc_ways2(time, distance);
        }
        Err(e) => println!("Error opening file: {e}")
    }
    
}

// Day 6 Part 2
fn calc_ways2(time: String, distance: String) {
    let t = time.parse::<usize>().unwrap();
    let d = distance.parse::<usize>().unwrap();
    let mut ways: usize = 0;

    println!("times: {:#?}", t);
    println!("distance: {:#?}", d);

    for i in 1..t {
        if i * (t - i) > d {
            ways += 1;
        }
    }
    println!("Ways: {:#?}", ways);
}

// Day 6 Part 1
fn calc_ways(times: Vec<usize>, distance: Vec<usize>) {
    let ways: Vec<usize> = times.iter().enumerate().map(
        |(idx, &time)| {
            let mut ways: usize = 0;
            for i in 1..time {
                if i * (time - i) > distance[idx] {
                    ways += 1;
                }
            }
            ways
        }
    ).collect();

    println!("Ways: {:#?}", ways);

    let total_ways = ways.iter().fold(1, |acc, x| acc * x);
    println!("Total ways: {:#?}", total_ways);
}

// Day 5 Part 2
fn get_seeds2(line: String, seeds: &mut Vec<(usize,usize)>) {
    let seed_line: Vec<&str> = line.split(":").collect();
    let seed_nums: Vec<&str> = seed_line[1].trim().split(" ").collect();
    let mut idx = 0;
    while idx < seed_nums.len() {
        let min = seed_nums[idx].parse::<usize>().unwrap();
        let max = min + seed_nums[idx+1].parse::<usize>().unwrap() - 1;
        seeds.push((min, max));
        idx += 2;
    }
}

fn convert_num(
    min: usize, max: usize, rate: usize, oper: Oper
) -> (usize, usize) {
    return match oper {
        Oper::Add => {
            (min.checked_add(rate).unwrap(),
            max.checked_add(rate).unwrap())
        },
        Oper::Sub => {
            (min.checked_sub(rate).unwrap(),
            max.checked_sub(rate).unwrap())
        },
    };
}

fn rec_convert_ranges(
    map: &HashMap<(usize, usize), (usize, Oper)>, s_min: usize, s_max: usize
) -> Vec<(usize, usize)> {
    let mut new_ranges: Vec<(usize, usize)> = vec![];

    for (&(min, max), &(rate, oper)) in map {
        let s_min_lower = min > s_min;
        let s_max_higher = max < s_max;
        let s_min_in_range = min <= s_min && s_min <= max;
        let s_max_in_range = min <= s_max && s_max <= max;
        if s_min_in_range && s_max_in_range { // within range
            // println!("IN RANGE -- Min: {min}, Max: {max}, SMIN: {s_min}, SMAX: {s_max}");
            // in range
            new_ranges.push(convert_num(s_min, s_max, rate, oper));
            return  new_ranges;
        } else if s_min_in_range && s_max_higher { // top portion extra
            // println!("EXTRA TOP -- Min: {min}, Max: {max}, SMIN: {s_min}, SMAX: {s_max}");
            // bottom in range
            new_ranges.push(convert_num(s_min, max, rate, oper));
            // max + 1, s_max
            new_ranges.append(
                &mut rec_convert_ranges(map, max + 1, s_max)
            );
            return  new_ranges;
        } else if s_min_lower && s_max_in_range { // bottom portion extra
            // println!("EXTRA BOTTOM -- Min: {min}, Max: {max}, SMIN: {s_min}, SMAX: {s_max}");
            // top in range
            new_ranges.push(convert_num(min, s_max, rate, oper));
            // s_min, min - 1
            new_ranges.append(
                &mut rec_convert_ranges(map, s_min, min - 1)
            );
            return  new_ranges;
        } else if s_min_lower && s_max_higher { // bottom and top extra portions
            // println!("EXTRA TOP&BOTTOM -- Min: {min}, Max: {max}, SMIN: {s_min}, SMAX: {s_max}");
            // middle within range
            new_ranges.push(convert_num(min, max, rate, oper));
            // max + 1, s_max
            new_ranges.append(
                &mut rec_convert_ranges(map, max + 1, s_max)
            );
            // s_min, min - 1
            new_ranges.append(
                &mut rec_convert_ranges(map, s_min, min - 1)
            );
            return  new_ranges;
        } else {     // outside range (i.e. greater or lesser)
            // println!("NO MATCH");

        }
                
            
    }

    if new_ranges.len() == 0 {
        new_ranges.push((s_min, s_max));
    }

    return new_ranges;
}

fn calc_lowest_dest2(
    seeds: Vec<(usize, usize)>, order: Vec<String>,
    map: HashMap<String, HashMap<(usize, usize), (usize, Oper)>>
) {
    let mut locations: Vec<(usize, usize)> = vec![];

    for (seed_min, seed_max) in seeds {
        let mut ranges: Vec<(usize, usize)> = vec![(seed_min, seed_max)];
        for map_name in &order {
            let mut converted_ranges = vec![];
            for (s_min, s_max) in ranges {
                converted_ranges.append(
                    &mut rec_convert_ranges(
                    map.get(map_name).unwrap(),
                     s_min, s_max
                ));
            }
            ranges = converted_ranges;
        }
        locations.append(&mut ranges);
    }
    // println!("Converted: {:#?}", locations);
    let lowest = locations.iter().map(
        |(o,d)| {
            *o
    }).min().unwrap();
    println!("Lowest Dest: {}", lowest);
}

// Day 5 Part 1
fn get_seeds(line: String, seeds: &mut Vec<usize>) {
    let seed_line: Vec<&str> = line.split(":").collect();
    for seed in seed_line[1].trim().split(" ") {
        seeds.push(seed.parse::<usize>().unwrap());
    }
}

#[derive(Debug, Clone, Copy)]
enum Oper {
    Sub,
    Add,
}

fn add_map_values(
    map_name: String, values_line: String, 
    map: &mut HashMap<String, HashMap<(usize, usize), (usize, Oper)>>
) {
    let mut convertion_map: HashMap<(usize, usize), (usize, Oper)> = HashMap::new();
    let values: Vec<&str> = values_line.trim().split(" ").collect();
    let dest = values[0].to_string().parse::<usize>().unwrap();
    let orig = values[1].to_string().parse::<usize>().unwrap();
    let range = values[2].to_string().parse::<usize>().unwrap() - 1;

    let (min_orig, max_orig) = (orig, orig + range);
    let convert_rate: (usize, Oper) = match orig > dest {
        true => {
            (orig.checked_sub(dest).unwrap(), Oper::Sub)
        },
        false => {
            (dest.checked_sub(orig).unwrap(), Oper::Add)
        },
    };

    convertion_map.insert((min_orig, max_orig), convert_rate);
    
    match map.contains_key(&map_name) {
        true => {
            for (k,v) in convertion_map {
                map.get_mut(&map_name).unwrap().insert(k, v);
            }
        },
        false => {
            map.insert(map_name, convertion_map);
        }
    }
}

fn calc_lowest_dest(
    seeds: Vec<usize>, order: Vec<String>, 
    map: &mut HashMap<String, HashMap<(usize, usize), (usize, Oper)>>
) {
    let mut locations: Vec<(usize, usize)> = vec![];
    for seed in seeds {
        let mut converted_num = seed;
        for current_map in &order {
            let mut already_converted = false;
            for ((min, max), (rate, oper)) in map.get(current_map).unwrap() {
                let should_convert = !already_converted &&
                    converted_num >= *min && converted_num <= *max;
                if should_convert {
                    match oper {
                        Oper::Add => {
                            already_converted = true;
                            converted_num = converted_num.checked_add(*rate)
                                .unwrap()
                        },
                        Oper::Sub => {
                            already_converted = true;
                            converted_num = converted_num.checked_sub(*rate)
                            .unwrap()
                        },
                    }
                }
            }
        }
        locations.push((seed, converted_num));
    }
    println!("Converted: {:#?}", locations);
    let lowest = locations.iter().map(
        |(o,d)| {
            *d
    }).min().unwrap();
    println!("Lowest Dest: {}", lowest);
}

// Day 4 Part 1
fn calc_scatch_card(
    line: String, idx: &usize, cards: &mut HashMap<usize, usize>
) {
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
    
    println!("Matches: {}", matches);
    
    let copies = cards.get(idx).unwrap().to_owned();

    println!("Copies: {}", copies);
    if matches > 0 {
        for i in 1..=matches {
            match cards.get_mut(&(idx+i)) {
                Some(num) => {*num += (1*copies)},
                None => {cards.insert(idx+i, 1*copies);},
            }
        }
    }
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