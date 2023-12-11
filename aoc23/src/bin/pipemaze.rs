use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input/input10.txt")?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut prev = (-1, -1);

    let mut i = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let row: Vec<_> = line.chars().collect();

                let mut j = 0;
                for c in row.iter() {
                    if *c == 'S' {
                        prev = (i, j);
                        break;
                    }
                    j += 1;
                }
                map.push(row);
                i += 1;
            }
            Err(_) => panic!("invalid input"),
        }
    }

    println!("start: {:?}", prev);
    println!("{}x{}", map.len(), map[0].len());

    // make a step
    // row,col+1: -,J,7 => right
    // row,col-1: -,L,F => left
    // row-1,col: |,7,F => up
    // row+1,col: |,L,J => down

    let mut curr = match (
        giv(&map, up(prev)),
        giv(&map, down(prev)),
        giv(&map, left(prev)),
        giv(&map, right(prev)),
    ) {
        (Some('|' | '7' | 'F'), _, _, _) => up(prev),
        (_, Some('|' | 'J' | 'L'), _, _) => down(prev),
        (_, _, Some('-' | 'F' | 'L'), _) => left(prev),
        (_, _, _, Some('-' | 'J' | '7')) => right(prev),
        _ => panic!("no direction chars found"),
    };

    let mut count: i64 = 1;

    loop {
        count += 1;

        println!("{:?} {:?}", curr, giv(&map, curr));

        let next = match giv(&map, curr) {
            Some('|') => {
                if prev == up(curr) {
                    down(curr)
                } else {
                    up(curr)
                }
            }
            Some('-') => {
                if prev == left(curr) {
                    right(curr)
                } else {
                    left(curr)
                }
            }
            Some('L') => {
                if prev == up(curr) {
                    right(curr)
                } else {
                    up(curr)
                }
            }
            Some('J') => {
                if prev == up(curr) {
                    left(curr)
                } else {
                    up(curr)
                }
            }
            Some('7') => {
                if prev == down(curr) {
                    left(curr)
                } else {
                    down(curr)
                }
            }
            Some('F') => {
                if prev == down(curr) {
                    right(curr)
                } else {
                    down(curr)
                }
            }
            Some('S') => break,
            Some('.') => panic!("ground"),
            _ => panic!("invalid char"),
        };

        prev = curr;
        curr = next;
    }

    println!("{:?}", count / 2);

    Ok(())
}

fn up(this: (i64, i64)) -> (i64, i64) {
    (this.0 - 1, this.1)
}

fn down(this: (i64, i64)) -> (i64, i64) {
    (this.0 + 1, this.1)
}

fn left(this: (i64, i64)) -> (i64, i64) {
    (this.0, this.1 - 1)
}

fn right(this: (i64, i64)) -> (i64, i64) {
    (this.0, this.1 + 1)
}

fn giv(map: &Vec<Vec<char>>, curr: (i64, i64)) -> Option<char> {
    let rows = map.len();
    let cols = map[0].len();

    if curr.0 >= 0
        && curr.0 < rows.try_into().unwrap()
        && curr.1 >= 0
        && curr.1 < cols.try_into().unwrap()
    {
        return Some(map[curr.0 as usize][curr.1 as usize]);
    }

    None
}
