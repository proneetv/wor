use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input31.txt")?;
    let reader = BufReader::new(file);
    let mat: Vec<Vec<char>> = reader.lines().map(|s| s.expect("REASON").chars().collect()).collect();

    let row = mat.len();
    let col = mat[0].len();

    let mut r = 0;
    let mut i;

    loop {
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

                let state = check_surrounding(start, end, r, &mat);

                if state {
                    sum += curr_num;
                } // if part number
            } // frame current number

            i = i+1;
            if i >= col {
                break;
            }
        }
        r = r + 1;
        if r >= row {
            break;
        }
    }

    println!("{}", sum);

    Ok(())
}

// left-1,lvl; right+1,lvl
// left-1,lvl-1 -> right+1,lvl-1
// left-1,lvl+1 -> right+1,lvl+1
fn check_surrounding(left: usize, right: usize, lvl: usize, mat: &Vec<Vec<char>>) -> bool {

    let left_col = if left == 0 {
        0
    } else {
        left - 1
    };
    let right_col = if right == 139 {
        139
    } else {
        right + 1
    };

    let top_lvl = if lvl == 0 {
        0
    } else {
        lvl - 1
    };
    let bot_lvl = if lvl == 139 {
        139
    } else {
        lvl + 1
    };

    let mut i = top_lvl;
    let mut j;

    loop {
        j = left_col;
        loop {

            assert!(i < 140 && j < 140, "{} {} {} {} {} {}", i, j, left_col, right_col, top_lvl, bot_lvl);
            if !mat[i][j].is_numeric() && mat[i][j] != '.' {
                return true;
            }

            j = j + 1;
            if j > right_col {
                break;
            }
        }

        i = i + 1;
        if i > bot_lvl {
            break;
        }
    }

    false
}
