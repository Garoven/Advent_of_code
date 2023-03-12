use std::error::Error;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let lines = INPUT.lines();
    let mut count_1: usize = 0;
    let mut count_2: usize = 0;
    for line in lines {
        let (e1, e2) = line.split_once(',').ok_or("Invalid pair")?;
        let (e1_1, e1_2) = e1.split_once('-').ok_or("Invalid range")?;
        let (e2_1, e2_2) = e2.split_once('-').ok_or("Invalid range")?;
        let r1 = e1_1.parse::<usize>()?..e1_2.parse::<usize>()?;
        let r2 = e2_1.parse::<usize>()?..e2_2.parse::<usize>()?;

        // Part one
        if r1.start <= r2.start && r1.end >= r2.end || r1.start >= r2.start && r1.end <= r2.end {
            count_1 += 1;
        }
        
        // Part two
        if r2.contains(&r1.start) || r2.contains(&r1.end) || r1.contains(&r2.start) || r1.contains(&r2.end) ||
            r1.start <= r2.start && r1.end >= r2.end || r1.start >= r2.start && r1.end <= r2.end 
        {
            count_2 += 1;
        } 
    }

    Ok(println!("Part one: {count_1}\nPart two: {count_2}"))
}
