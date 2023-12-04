use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::cmp::{max, min};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input41.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let twoparts: Vec<_> = line.split("|").map(|s| s.trim()).collect();
                let casewin: Vec<_> = twoparts[0].split(":").map(|s| s.trim()).collect();
                let leftnums: Vec<_> = casewin[1].split(" ").map(|s| s.trim()).collect();
                let rightnums: Vec<_> = twoparts[1].split(" ").map(|s| s.trim()).collect();

                let mut winnums = HashSet::new();
                for num in leftnums {
                    if num.len() != 0 {
                        winnums.insert(num);
                    }
                }
                println!("{:?}", winnums);

                let mut curr_sum = 0;
                let mut first = false;
                for num in rightnums {
                    if winnums.contains(num) {
                        if first {
                            curr_sum = curr_sum * 2;
                        } else {
                            curr_sum = 1;
                            first = true;
                        }
                    }
                }

                sum += curr_sum;
            },
            Err(err) => {},
        }
    }

    println!("{}", sum);

    Ok(())
}
