use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap, VecDeque};
use std::cmp::{max, min};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;
    let mut state : HashMap<i32, i32> = HashMap::new();

    let file = File::open("input/input41.txt")?;
    let reader = BufReader::new(file);

    let mut iter = 1;

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

                let mut matches = 0;
                for num in rightnums {
                    if winnums.contains(num) {
                        matches += 1;
                    }
                }

                if !state.contains_key(&iter) {
                    state.insert(iter, 1);
                }

                for i in iter+1..iter+matches+1 {
                    if !state.contains_key(&i) {
                        state.insert(i, 1);
                    }
                    state.insert(i, state.get(&i).unwrap() + state.get(&iter).unwrap());
                }
            },
            Err(err) => {},
        }
        iter += 1;
    }

    for i in state.values() {
        sum += i;
    }
    println!("{}", sum);

    Ok(())
}
