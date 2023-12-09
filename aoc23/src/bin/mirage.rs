use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input9.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let nums = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
                sum += next_term(&nums);
            }
            Err(_) => {}
        }
    }

    println!("{:?}", sum);

    Ok(())
}

fn next_term(nums: &Vec<i64>) -> i64 {
    let mut diff = Vec::new();
    for i in 0..nums.len() - 1 {
        diff.push(&nums[i + 1] - &nums[i]);
    }

    if diff.iter().all(|n| *n == 0) {
        return *nums.last().unwrap();
    } else {
        return *nums.last().unwrap() + next_term(&diff);
    }
}
