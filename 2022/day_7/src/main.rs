#![feature(if_let_guard)]
use std::{error::Error, collections::HashMap};

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let mut cur_dir: Vec<&str> = Vec::new();
    let mut map: HashMap<Vec<&str>, usize> = HashMap::new();
    for line in INPUT.lines() {
        match line {
            line if line.starts_with("$ cd") => {
                let chunks: Vec<&str> = line.split_whitespace().collect();
                match chunks[2] {
                    ".." => { cur_dir.pop(); },
                    dir => cur_dir.push(dir)
                }
            },
            line if let Ok(size) = line.split_whitespace().next().unwrap().parse::<usize>() => {
                for i in 1..cur_dir.len() + 1 {
                    if let Some(v) = map.get_mut(&cur_dir[0..i]) {
                        *v += size;

                    } else {
                        map.insert(cur_dir[0..i].to_vec(), size);
                    }
                }
            },
            _ => {}
        }
    }

    let total_1: usize = map.values().filter(|&size| *size <= 100000).sum();

    let root_size: usize = *map.get(&vec!["/"]).unwrap();
    let total_2: &usize = map.values().filter(|&&size| size >= root_size - 40000000).min().unwrap();

    Ok(println!("Part one: {total_1}\nPart two: {total_2}"))
}
