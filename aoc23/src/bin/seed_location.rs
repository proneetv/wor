#![feature(btree_cursors)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Bound;

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
    let seeds: Vec<i64> = seednumstr
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
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

    let mut seedloc: Vec<i64> = Vec::new();
    for seed in seeds {
        let mut start = seed;
        for map in &listomaps {
            match srctodst(start, map) {
                Some((src, dst, cnt)) => {
                    if start >= src && start < src + cnt {
                        start = dst + (start - src);
                    }
                }
                None => {}
            }
        }
        seedloc.push(start);
    }

    println!("{:?}", seedloc.iter().min());

    Ok(())
}

fn srctodst(key: i64, mapper: &BTreeMap<i64, (i64, i64)>) -> Option<(i64, i64, i64)> {
    let cursor = mapper.upper_bound(Bound::Included(&key));
    if cursor.key() != None {
        let val = cursor.value()?.clone();
        Some((cursor.key()?.clone(), val.0, val.1))
    } else {
        None
    }
}

fn parsemap(ranges: &Vec<String>) -> BTreeMap<i64, (i64, i64)> {
    let mut map = BTreeMap::new();
    for s in ranges {
        let dstsrc: Vec<_> = s
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| s.len() != 0)
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        map.insert(dstsrc[1], (dstsrc[0], dstsrc[2]));
    }
    map
}
