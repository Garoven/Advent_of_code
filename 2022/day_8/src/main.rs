use std::error::Error;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in INPUT.lines() {
        for (i, c) in line.chars().enumerate() {
            if let Some(v) = map.get_mut(i) {
                v.push(c.to_digit(10).unwrap());
            } else {
                map.push(vec![c.to_digit(10).unwrap()]);
            }
        }
    }
    let mut total_1: usize = 0;
    for (hi, h) in map.iter().enumerate() {
        for (vi, v) in h.iter().enumerate() {
            match hi {
                0 => total_1 += 1,
                hi if hi == map.len() - 1 => total_1 += 1,
                hi => {
                    match vi {
                        0 => total_1 += 1,
                        vi if vi == h.len() - 1 => total_1 += 1,
                        vi => {
                            if map[hi + 1..].iter().all(|h| h[vi] < *v ) ||
                                map[..hi].iter().all(|h| h[vi] < *v ) ||
                                map[hi][vi + 1..].iter().all(|ov| ov < v) ||
                                map[hi][..vi].iter().all(|ov| ov < v)
                            {
                                total_1 += 1;
                            }
                        },
                    }
                },
            }
        }
    }

    // 314820
    Ok(println!("Part one: {}\n", total_1))
}
