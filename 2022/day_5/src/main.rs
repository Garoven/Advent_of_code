#![feature(iter_next_chunk)]
use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./2022/day_5/input")?;
    let mut magazine: Vec<Vec<char>> = vec![vec![];9];
    let mut lines = input.lines();
    let mut pattern = lines.next_chunk::<8>().unwrap().into_iter().rev();
    while let Some(line) = pattern.next() {
        let line = format!("{line} ");
        let mut chars = line.chars();
        let mut i = 0;
        while let Ok(ch) = chars.next_chunk::<4>() {
            if ch[1].is_alphabetic() {
                magazine[i].push(ch[1]);
            }
            i += 1;
        }
    }

    let mut part_one = magazine.clone();
    let mut part_two = magazine.clone();

    let mut lines = lines.skip(2);
    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let count: usize = split[1].parse()?;
        let from: usize = split[3].parse()?;
        let to: usize = split[5].parse()?;

        // Part one
        let r = part_one[from-1].len() - count..part_one[from-1].len();
        let mut stack: Vec<char> = part_one[from-1].drain(r).rev().collect();
        part_one[to-1].append(&mut stack);

        // Part two
        let r = part_two[from-1].len() - count..part_two[from-1].len();
        let mut stack: Vec<char> = part_two[from-1].drain(r).collect();
        part_two[to-1].append(&mut stack);
    }

    let result_1: Vec<&char> = part_one.iter().map(|v| v.last().unwrap()).collect();
    let result_2: Vec<&char> = part_two.iter().map(|v| v.last().unwrap()).collect();

    Ok(println!("Part one: {:?}\nPart two: {:?}", result_1, result_2))
}
