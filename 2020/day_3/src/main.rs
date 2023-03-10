#![feature(iter_next_chunk)]
use std::{error::Error, fs::read_to_string };

fn main() -> Result<(), Box<dyn Error>> {
    let prio: Vec<(usize, char)> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate().collect();
    let input = read_to_string("./2020/day_3/input")?;
    let mut lines = input.lines();
    let mut points: usize = 0;
    while let Ok(line) = lines.next_chunk::<3>() {
        let letter: Option<char> = line[0].chars().find_map(|c| {
            if line[1].contains(c) && line[2].contains(c) {
                Some(c)
            } else {
                None
            }
        });
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
