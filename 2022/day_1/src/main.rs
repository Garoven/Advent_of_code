use std::error::Error;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let lines = INPUT.lines();
    let mut elfs: Vec<usize> = Vec::new();
    let mut elf: usize = 0;
    for line in lines {
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
