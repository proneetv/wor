use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input8.txt")?;
    let mut reader = BufReader::new(file);

    let mut left = HashMap::new();
    let mut right = HashMap::new();
    let mut opt = Vec::new();

    let mut walk = String::new();
    let _ = reader.read_line(&mut walk)?;
    walk.pop();
    let dir: Vec<_> = walk.chars().collect();
    let _ = reader.read_line(&mut String::new())?;

    let mut mapping = String::new();
    while reader.read_line(&mut mapping)? > 1 {
        let nodes: Vec<_> = mapping.split("=").map(|s| s.trim()).collect();
        left.insert(nodes[0].to_string(), nodes[1][1..4].to_string());
        right.insert(nodes[0].to_string(), nodes[1][6..9].to_string());

        let len = nodes[0].to_string().len();
        if &nodes[0][len - 1..] == "A" {
            opt.push(nodes[0].to_string());
        }

        mapping = String::new();
    }

    let mut ans = Vec::new();

    for op in 0..opt.len() {
        let mut current = opt[op].to_string();
        while !current.ends_with('Z') {
            let idx = sum % dir.len();
            if dir[idx] == 'L' {
                current = left.get(&current).unwrap().to_string();
            } else {
                current = right.get(&current).unwrap().to_string();
            }
            sum += 1;
        }
        ans.push(sum);
        sum = 0;
    }

    println!(
        "{}",
        ans.iter()
            .fold(1 as u64, |l, &r| lcm(l, r.try_into().unwrap()))
    );

    Ok(())
}
