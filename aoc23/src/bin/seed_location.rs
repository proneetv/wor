use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input/input51.txt")?;
    let mut reader = BufReader::new(file);

    // seeds: 79 14 55 13
    // seed-to-soil map:
    // soil-to-fertilizer map:
    // fertilizer-to-water map:
    // water-to-light map:
    // light-to-temperature map:
    // temperature-to-humidity map:
    // humidity-to-location map:

    let mut seedstr = String::new();
    let _ = reader.read_line(&mut seedstr)?;

    let seednumstr = &seedstr[7..];
    let seedrange: Vec<i64> = seednumstr
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut i = 0;
    while i < seedrange.len() {
        seeds.push((seedrange[i], seedrange[i + 1]));
        i += 2;
    }

    let mut listomaps = Vec::new();

    // lines to be ignored
    let _ = reader.read_line(&mut String::new())?;
    let _ = reader.read_line(&mut String::new())?;

    let mut count = 0;
    while count < 7 {
        let mut mapstrs = Vec::new();
        let mut token = String::new();
        while reader.read_line(&mut token)? > 1 {
            mapstrs.push(token.clone());
            token = String::new();
        }
        let _ = reader.read_line(&mut token); // prep for next iteration
        let rangemap = parsemap(&mapstrs);
        listomaps.push(rangemap.clone());

        count += 1;
    }

    for mapper in &listomaps {
        let mut new_ranges = Vec::new();
        for this in mapper {
            let mut build_range = Vec::new();
            for seed in seeds {
                if to(seed) < this.0 || seed.0 > to_m(*this) {
                    // non overlapping
                    build_range.push(seed.clone());
                } else {
                    // right portion overlapping
                    if seed.0 < this.0 {
                        build_range.push((seed.0, this.0 - seed.0).clone());
                    }

                    // left portion overlapping
                    if to(seed) > to_m(*this) {
                        build_range.push((to_m(*this) - 1, to(seed) - to_m(*this)).clone());
                    }

                    let c = this.0 - this.1;
                    let left = max(seed.0, this.0) - c;
                    let right = min(to(seed), to_m(*this)) - c;
                    new_ranges.push((left, right - left).clone());
                }
            }
            seeds = build_range.clone();
        }
        new_ranges.append(&mut seeds);
        seeds = new_ranges.clone();
    }

    println!("{:?}", seeds.into_iter().map(|(from, _)| from).min());

    Ok(())
}

fn to(val: (i64, i64)) -> i64 {
    val.0 + val.1
}

fn to_m(val: (i64, i64, i64)) -> i64 {
    val.0 + val.2
}

fn parsemap(ranges: &Vec<String>) -> Vec<(i64, i64, i64)> {
    let mut map = Vec::new();
    for s in ranges {
        let dstsrc: Vec<_> = s
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| s.len() != 0)
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        map.push((dstsrc[1], dstsrc[0], dstsrc[2]));
    }
    map
}
