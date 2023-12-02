use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let mut sum = 0;

    let file = File::open("input/input21.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Game <num>: XX red, YY green, ZZ blue; XX red, YY green, ...
                let mut gamestate = true;
                let colonsplit: Vec<_> = line.split(":").collect();
                let gameblock: Vec<_> = colonsplit[0].split(" ").collect();
                let gamenum = gameblock[1].parse::<i32>().unwrap();

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
                            if r <= 12 && g <= 13 && b <= 14 {
                                continue;
                            } else {
                                gamestate = false;
                                break;
                            }
                        }
                    }
                    if gamestate == false {
                        break;
                    }
                }
                if gamestate {
                    sum += gamenum;
                }
            },
            Err(err) => return Err(err),
        }
    }

    println!("{}", sum);

    Ok(())
}
