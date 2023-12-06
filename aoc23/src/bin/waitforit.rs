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

    let mut lo = 0;
    let mut hi = time;
    let mut mid = lo + (hi - lo) / 2;

    while lo < hi {
        if !race(mid - 1, time, dist) && race(mid, time, dist) {
            break;
        } else if race(mid - 1, time, dist) {
            hi = mid - 1;
        } else if !race(mid, time, dist) {
            lo = mid + 1;
        }
        mid = lo + (hi - lo) / 2;
    }
    let start = mid;

    lo = 0;
    hi = time;
    mid = lo + (hi - lo) / 2;

    while lo < hi {
        if race(mid, time, dist) && !race(mid + 1, time, dist) {
            break;
        } else if race(mid, time, dist) {
            lo = mid + 1;
        } else if !race(mid, time, dist) {
            hi = mid;
        }
        mid = lo + (hi - lo) / 2;
    }

    println!("{}", mid - start + 1);
    Ok(())
}

fn race(speed: i64, time: i64, dist: i64) -> bool {
    (time - speed) * speed > dist
}
