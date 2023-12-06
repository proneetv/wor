use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input/input61.txt")?;
    let mut reader = BufReader::new(file);

    let mut timestr = String::new();
    let _ = reader.read_line(&mut timestr);

    let mut diststr = String::new();
    let _ = reader.read_line(&mut diststr);

    let dists = &diststr[11..];
    let times = &timestr[11..];

    let time = times
        .split(" ")
        .map(|y| y.trim())
        .filter(|x| x.len() != 0)
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let dist = dists
        .split(" ")
        .map(|y| y.trim())
        .filter(|x| x.len() != 0)
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let wins: usize = (0..time)
        .filter(|speed| (time - speed) * speed > dist)
        .count();

    println!("{}", wins);
    Ok(())
}
