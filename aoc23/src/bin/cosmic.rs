use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input/input11.txt")?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                map.push(line.chars().collect());
            }
            Err(_) => {}
        }
    }

    let mut row = 0;
    while row < map.len() {
        if (0..map[0].len()).all(|c| map[row][c] == '.') {
            row += 1;
            let mut newrow = Vec::new();
            let mut i = 0;
            while i < map[0].len() {
                newrow.push('.');
                i += 1;
            }

            map.insert(row, newrow);
        }
        row += 1;
    }

    let mut col = 0;
    while col < map[0].len() {
        if (0..map.len()).all(|r| map[r][col] == '.') {
            col += 1;

            let mut i = 0;
            while i < map.len() {
                map[i].insert(col, '.');
                i += 1;
            }
        }
        col += 1;
    }

    let mut galaxy = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '#' {
                galaxy.push((row as i64, col as i64));
            }
        }
    }

    let mut sum = 0;
    for from in &galaxy {
        for to in &galaxy {
            print!("f{:?}", from);
            print!("t{:?}", to);
            println!();
            sum += (from.0 - to.0).abs() + (from.1 - to.1).abs();
        }
    }

    println!("{}", sum / 2);

    Ok(())
}
