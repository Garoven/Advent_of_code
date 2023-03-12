use std::{error::Error, cmp::Ordering};

const INPUT: &str = include_str!("../input");

#[derive(PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x = match self {
            Rps::Rock => match other {
                Rps::Rock => Ordering::Equal,
                Rps::Paper => Ordering::Less,
                Rps::Scissors => Ordering::Greater
            },
            Rps::Paper => match other {
                Rps::Rock => Ordering::Greater,
                Rps::Paper => Ordering::Equal,
                Rps::Scissors => Ordering::Less
            },
            Rps::Scissors => match other {
                Rps::Rock => Ordering::Less,
                Rps::Paper => Ordering::Greater,
                Rps::Scissors => Ordering::Equal
            }

        };
        Some(x)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = INPUT.lines();
    let mut points: usize = 0;
    for line in lines {
        let strat: Vec<&str> = line.split(' ').collect();

        let opponent = match strat[0] {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            i => return Err(format!("Invalid input: {i}").into())

        };

        match strat[1] {
            "X" => {
                match opponent {
                    Rps::Rock => {
                        points += 3;
                        Rps::Scissors
                    },
                    Rps::Paper => {
                        points += 1;
                        Rps::Rock
                    },
                    Rps::Scissors => {
                        points += 2;
                        Rps::Paper
                    }
                }
            },
            "Y" => {
                match opponent {
                    Rps::Rock => {
                        points += 4;
                        Rps::Rock
                    },
                    Rps::Paper => {
                        points += 5;
                        Rps::Paper
                    },
                    Rps::Scissors => {
                        points += 6;
                        Rps::Scissors
                    }
                }
            },
            "Z" => {
                match opponent {
                    Rps::Rock => {
                        points += 8;
                        Rps::Paper
                    },
                    Rps::Paper => {
                        points += 9;
                        Rps::Scissors
                    },
                    Rps::Scissors => {
                        points += 7;
                        Rps::Rock
                    }
                }
            },
            i => return Err(format!("Invalid input: {i}").into())
        };
    }

    Ok(println!("My points: {points}"))
}
