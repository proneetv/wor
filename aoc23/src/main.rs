use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let sum = 0;

    let file = File::open("input/input41.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => { println!("{}", line); },
            Err(_) => {},
        }
    }

    println!("{}", sum);

    Ok(())
}
