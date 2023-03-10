use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./2022/day_1/input")?;
    let mut lines = input.lines().into_iter();
    let mut elfs: Vec<usize> = Vec::new();
    let mut elf: usize = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            elfs.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<usize>()?;
        }
    }
    elfs.push(elf);
    elfs.sort();

    let top_3 = elfs.split_off(elfs.len()-3);

    Ok(println!("Most 3 calories carried: {:?}. All: {}", top_3, top_3.iter().sum::<usize>()))
}
