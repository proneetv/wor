use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap, VecDeque};
use std::cmp::{max, min};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;
    let mut state : HashMap<i32, Vec<i32>> = HashMap::new();

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
                    state.insert(iter, Vec::new());
                }

                for i in iter+1..iter+matches+1 {
                    state.entry(iter).or_default().push(i);
                }
            },
            Err(err) => {},
        }
        iter += 1;
    }

    let mut visited = VecDeque::new();
    for i in 1..iter {
        visited.push_back(i);
    }

    while visited.len() != 0 {
        sum += 1;
        let item = visited.pop_front().unwrap();
        let vals = state.get(&item).unwrap();
        for i in vals {
            visited.push_back(*i);
        }
    }

    println!("{}", sum);

    Ok(())
}
