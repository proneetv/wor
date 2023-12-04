use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let sum = 0;

    let file = File::open("input/input41.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(_line) => {},
            Err(_err) => {},
        }
    }

    println!("{}", sum);

    Ok(())
}
