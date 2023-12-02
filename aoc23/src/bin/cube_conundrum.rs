use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input21.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Game <num>: XX red, YY green, ZZ blue; XX red, YY green, ...
                let colonsplit: Vec<_> = line.split(":").collect();

                let mut game_r = 0;
                let mut game_g = 0;
                let mut game_b = 0;

                let games = colonsplit[1].split(";");
                for game in games {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;

                    let rgb: Vec<_> = game.split(",").map(|s| s.trim()).collect();
                    for i in rgb {
                        if let Some((num, color)) = i.split_once(" ") {
                            match color {
                                "red" => r = num.parse::<i32>().unwrap(),
                                "green" => g = num.parse::<i32>().unwrap(),
                                "blue" => b = num.parse::<i32>().unwrap(),
                                _ => {},
                            }
                        } // iteration rgb values are set
                        game_r = cmp::max(game_r, r);
                        game_g = cmp::max(game_g, g);
                        game_b = cmp::max(game_b, b);
                    }
                }
                let power = game_r * game_g * game_b;
                sum += power;
            },
            Err(err) => return Err(err),
        }
    }

    println!("{}", sum);

    Ok(())
}
