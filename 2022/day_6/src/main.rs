use std::{error::Error, collections::VecDeque};

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let chars: VecDeque<char> = INPUT.chars().collect();
    let mut i = 0usize;

    // Part one
    let result_1 = loop {
        let mut slice: Vec<&char> = chars.range(i..i + 4).collect();
        slice.sort();
        slice.dedup_by(|a, b| a == b);
        match slice.len() {
            4 => {
                let x = i + 4;
                i = 0;
                break x
            },
            _ => i += 1,
        }
    };

    // Part two
    let result_2 =  loop {
        let mut slice: Vec<&char> = chars.range(i..i + 14).collect();
        slice.sort();
        slice.dedup_by(|a, b| a == b);
        match slice.len() {
            14 => {
                break i + 14
            },
            _ => i += 1,
        }
    };

    Ok(println!("Part one: {}\nPart two: {}", result_1, result_2))
}
