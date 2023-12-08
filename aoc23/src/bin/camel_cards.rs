use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input/input71.txt")?;
    let reader = BufReader::new(file);

    let mut games = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let tokens: Vec<_> = line.split(" ").collect();
                games.push((tokens[0].to_string(), tokens[1].to_string()));
            }
            Err(_) => {}
        }
    }

    let mut hashedcards = games
        .clone()
        .into_iter()
        .map(|(cards, bid)| {
            (
                cards
                    .chars()
                    .map(|c| match c {
                        'A' => 12,
                        'K' => 11,
                        'Q' => 10,
                        'J' => 0,
                        'T' => 9,
                        '9' => 8,
                        '8' => 7,
                        '7' => 6,
                        '6' => 5,
                        '5' => 4,
                        '4' => 3,
                        '3' => 2,
                        '2' => 1,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<usize>>(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .map(|(cards, bid)| -> (usize, Vec<usize>, usize) {
            let mut count: [usize; 13] = [0; 13];
            cards.iter().for_each(|card| count[*card] += 1);

            let jackcount = count[0];

            // reset jack count for replacing
            count[0] = 0;

            let count = count.into_iter().filter(|f| f > &1).collect::<Vec<usize>>();

            let strength = match (count.iter().sum::<usize>() + jackcount, count.len()) {
                (5, 1) => 7,
                (5, 0) => 7,
                (4, 0) => 7,
                (4, 1) => 6,
                (3, 0) => 6,
                (5, 2) => 5,
                (3, 1) => 4,
                (2, 0) => 4,
                (4, 2) => 3,
                (2, 1) => 2,
                (1, 0) => 2,
                _ => 1,
            };

            (strength, cards, bid)
        })
        .collect::<Vec<(usize, Vec<usize>, usize)>>();

    hashedcards.sort_unstable();
    println!("{:?}", hashedcards);

    let mut sum = 0;
    for (i, card) in hashedcards.iter().enumerate() {
        sum += (i + 1) as u32 * card.2 as u32;
    }

    println!("{}", sum);

    Ok(())
}
