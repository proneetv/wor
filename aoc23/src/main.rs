use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let nums: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string()
    ];
    let revnums: Vec<String> = nums.iter().map(|c| c.chars().rev().collect()).collect();

    let file = File::open("input/input11.txt")?;
    let reader = BufReader::new(file);

    let mut sum : u64 = 0;
    let mut curr_num;
    for line in reader.lines() {
        curr_num = 0;
        match line {
            Ok(line) => {
                curr_num += get_digit(&line, &nums);
                let revline : String = line.chars().rev().collect();
                curr_num = curr_num*10 + get_digit(&revline, &revnums);
            },
            Err(err) => return Err(err),
        }
        // println!("{}", curr_num);
        sum += curr_num;
    }

    println!("{}", sum);

    Ok(())
}

fn get_digit(line: &str, nums: &Vec<String>) -> u64 {
    let mut digit_pos = 0;
    let mut digit = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            digit = c as u64 - '0' as u64;
            break;
        }
        digit_pos += 1;
    }

    let mut token_num = 1;
    for token in nums {
        match line.find(token) {
            Some(pos) => {
                if pos < digit_pos {
                    digit = token_num;
                    digit_pos = pos;
                }
            },
            None => {},
        };
        token_num += 1;
    }

    return digit;
}
