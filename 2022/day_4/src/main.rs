use std::{fs::read_to_string, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./2022/day_4/input")?;
    let mut lines = input.lines();
    let mut count: usize = 0;
    while let Some(line) = lines.next() {
        let (e1, e2) = line.split_once(',').ok_or("Invalid pair")?;
        let (e1_1, e1_2) = e1.split_once('-').ok_or("Invalid range")?;
        let (e2_1, e2_2) = e2.split_once('-').ok_or("Invalid range")?;
        let r1 = e1_1.parse::<usize>()?..e1_2.parse::<usize>()?;
        let r2 = e2_1.parse::<usize>()?..e2_2.parse::<usize>()?;

        //if r1.start <= r2.start && r1.end >= r2.end || r1.start >= r2.start && r1.end <= r2.end {
        //    count += 1;
        //}
        
        if r2.contains(&r1.start) || r2.contains(&r1.end) || r1.contains(&r2.start) || r1.contains(&r2.end) ||
            r1.start <= r2.start && r1.end >= r2.end || r1.start >= r2.start && r1.end <= r2.end 
        {
            count += 1;
        } 
    }

    Ok(println!("Pairs count: {count}"))
}
