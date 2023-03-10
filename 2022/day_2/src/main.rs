use std::{fs::read_to_string, error::Error, cmp::Ordering};

#[derive(PartialEq)]
enum RPS {
    ROCK,
    PAPER,
    SCISSORS
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x = match self {
            RPS::ROCK => match other {
                RPS::ROCK => Ordering::Equal,
                RPS::PAPER => Ordering::Less,
                RPS::SCISSORS => Ordering::Greater
            },
            RPS::PAPER => match other {
                RPS::ROCK => Ordering::Greater,
                RPS::PAPER => Ordering::Equal,
                RPS::SCISSORS => Ordering::Less
            },
            RPS::SCISSORS => match other {
                RPS::ROCK => Ordering::Less,
                RPS::PAPER => Ordering::Greater,
                RPS::SCISSORS => Ordering::Equal
            }

        };
        Some(x)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./2022/day_2/input")?;
    let mut lines = input.lines();
    let mut points: usize = 0;
    while let Some(line) = lines.next() {
        let strat: Vec<&str> = line.split(" ").collect();

        let opponent = match strat[0] {
            "A" => RPS::ROCK,
            "B" => RPS::PAPER,
            "C" => RPS::SCISSORS,
            i => return Err(format!("Invalid input: {i}").into())

        };

        match strat[1] {
            "X" => {
                match opponent {
                    RPS::ROCK => {
                        points += 3;
                        RPS::SCISSORS
                    },
                    RPS::PAPER => {
                        points += 1;
                        RPS::ROCK
                    },
                    RPS::SCISSORS => {
                        points += 2;
                        RPS::PAPER
                    }
                }
            },
            "Y" => {
                match opponent {
                    RPS::ROCK => {
                        points += 4;
                        RPS::ROCK
                    },
                    RPS::PAPER => {
                        points += 5;
                        RPS::PAPER
                    },
                    RPS::SCISSORS => {
                        points += 6;
                        RPS::SCISSORS
                    }
                }
            },
            "Z" => {
                match opponent {
                    RPS::ROCK => {
                        points += 8;
                        RPS::PAPER
                    },
                    RPS::PAPER => {
                        points += 9;
                        RPS::SCISSORS
                    },
                    RPS::SCISSORS => {
                        points += 7;
                        RPS::ROCK
                    }
                }
            },
            i => return Err(format!("Invalid input: {i}").into())
        };
    }

    Ok(println!("My points: {points}"))
}
