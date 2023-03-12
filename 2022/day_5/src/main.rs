#![feature(iter_next_chunk)]
use std::error::Error;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let mut magazine: Vec<Vec<char>> = vec![vec![];9];
    let mut lines = INPUT.lines();
    let pattern = lines.next_chunk::<8>().unwrap().into_iter().rev();
    for line in pattern {
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

    let lines = lines.skip(2);
    for line in lines {
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
