use std::fs;

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(f) => {
            let mut sum = 0;
            for l in f.split('\n') {
                println!("\nLine: {l}");
                let n = get_num(l.to_string());
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