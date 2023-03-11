use std::{error::Error, fs::read_to_string, collections::VecDeque};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./2022/day_6/input")?;
    let chars: VecDeque<char> = input.chars().collect();
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
