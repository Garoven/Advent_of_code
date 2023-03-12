#![feature(iter_next_chunk)]
use std::error::Error;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let prio: Vec<(usize, char)> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate().collect();
    let mut lines = INPUT.lines();
    let mut points: usize = 0;
    while let Ok(line) = lines.next_chunk::<3>() {
        let letter: Option<char> = line[0].chars().find(|c| line[1].contains(*c) && line[2].contains(*c));
        prio.iter().find(|(v, c)| {
            if *c == letter.unwrap() {
                points += v+1;
                true
            } else {
                false
            }
        });
    }

    Ok(println!("Total points: {points}"))
}
