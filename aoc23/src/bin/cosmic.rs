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

    let mut rowdist = Vec::new();
    let mut coldist = Vec::new();

    let mut r = 0;
    for row in 0..map.len() {
        rowdist.push(r as i64);
        if (0..map[0].len()).all(|c| map[row][c] == '.') {
            r += 1000000 - 1;
        }
        r += 1;
    }

    let mut c = 0;
    for col in 0..map[0].len() {
        coldist.push(c as i64);
        if (0..map.len()).all(|r| map[r][col] == '.') {
            c += 1000000 - 1;
        }
        c += 1;
    }

    let mut galaxy = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '#' {
                galaxy.push((row, col));
            }
        }
    }

    let mut sum = 0;
    for from in &galaxy {
        for to in &galaxy {
            sum +=
                (rowdist[from.0] - rowdist[to.0]).abs() + (coldist[from.1] - coldist[to.1]).abs();
        }
    }

    println!("{}", sum / 2);

    Ok(())
}
