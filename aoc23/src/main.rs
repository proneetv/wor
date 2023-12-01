use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input/input11.txt")?;
    let reader = BufReader::new(file);

    let mut sum : u64 = 0;
    let mut curr_num;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                curr_num = "".to_string();
                for c in line.chars() {
                    if c.is_digit(10) {
                        curr_num.push(c);
                        break;
                    }
                }
                for c in line.chars().rev() {
                    if c.is_digit(10) {
                        curr_num.push(c);
                        break;
                    }
                }
            },
            Err(err) => return Err(err),
        }
        println!("{}", curr_num.parse::<u64>().unwrap());
        sum += curr_num.parse::<u64>().unwrap();
    }

    println!("{}", sum);

    Ok(())
}
