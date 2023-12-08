use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    let mut walk = String::new();
    let _ = reader.read_line(&mut walk)?;
    walk.pop();

    let dir: Vec<_> = walk.chars().collect();
    println!("{:?}", dir);

    // skip blank line
    let _ = reader.read_line(&mut String::new())?;

    let mut mapping = String::new();
    while reader.read_line(&mut mapping)? > 1 {
        let nodes: Vec<_> = mapping.split("=").map(|s| s.trim()).collect();
        left.insert(nodes[0].to_string(), nodes[1][1..4].to_string());
        right.insert(nodes[0].to_string(), nodes[1][6..9].to_string());

        mapping = String::new();
    }

    println!("{:?}", left);
    println!("{:?}", right);
    let mut current = "AAA".to_string();

    while current != "ZZZ" {
        let idx = sum % dir.len();
        if dir[idx] == 'L' {
            current = left.get(&current).unwrap().to_string();
        } else {
            current = right.get(&current).unwrap().to_string();
        }
        sum += 1;
    }

    println!("{}", sum);

    Ok(())
}
