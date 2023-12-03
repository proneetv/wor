use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::{max, min};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input31.txt")?;
    let reader = BufReader::new(file);
    let mat: Vec<Vec<char>> = reader.lines().map(|s| s.expect("REASON").chars().collect()).collect();

    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();
    let row = mat.len();
    let col = mat[0].len();

    let mut i;
    for r in 0..row {
        i = 0;
        loop {
            let mut curr_num;
            let c = mat[r][i];
            let start = i;
            let mut end;

            if c.is_numeric() {
                curr_num = c as u32 - '0' as u32;
                end = i;

                while i+1 < col && mat[r][i+1].is_numeric() {
                    i = i+1;
                    curr_num = curr_num * 10 + (mat[r][i] as u32 - '0' as u32);
                    end = i;
                }

                let state = check_surrounding(start, end, r, &mat, row, col);
                if state.0 {
                    let key = format!("{},{}", state.1, state.2);

                    if gears.contains_key(&key) {
                        let mut vals = gears.get(&key).unwrap().clone();
                        vals.push(curr_num);
                        gears.insert(key, (&vals).to_vec());
                    } else {
                        gears.insert(key, vec![curr_num]);
                    }
                }
            }

            // need to learn how to beautify this
            i = i+1;
            if i >= col {
                break;
            }
        }
    }

    for gs in gears.values() {
        if gs.len() == 2 {
            sum += gs[0] * gs[1];
        }
        if gs.len() > 2 {
            println!("{:?}", gs);
        } // no test case with more than 2 part nums around a gear
    }

    println!("{}", sum);

    Ok(())
}

// left-1,lvl; right+1,lvl
// left-1,lvl-1 -> right+1,lvl-1
// left-1,lvl+1 -> right+1,lvl+1
fn check_surrounding(left: usize, right: usize, lvl: usize,
                     mat: &Vec<Vec<char>>, row: usize, col: usize) -> (bool, usize, usize) {

    let left_col = max(0, left as isize - 1) as usize;
    let right_col = min(col - 1, right + 1) as usize;
    let top_lvl = max(0, lvl as isize - 1) as usize;
    let bot_lvl = min(row - 1, lvl + 1);

    for i in top_lvl..bot_lvl+1 {
        for j in left_col..right_col+1 {
            if mat[i][j] == '*' {
                return (true, i, j);
            }
        }
    }

    (false, 0, 0)
}
