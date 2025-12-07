use util;

util::main![pt1, pt2];

use nom::{bytes::complete::tag, character::complete::digit0, multi::separated_list0, IResult};

fn parse_lists(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let (remainder, v) = separated_list0(tag("\n"), digit0)(input)?;
    Ok((
        remainder,
        v.iter()
            .map(|e| {
                e.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|e| e as u64)
                    .collect()
            })
            .collect(),
    ))
}

fn battery_jolt(input: Vec<u64>, amount: usize) -> Result<u64, Box<dyn std::error::Error>> {
    // input: the battery vector to examine
    // amount: the amount of batteries
    let mut num = 0;
    let mut start_pos = 0;
    for i in 1..=amount {
        let slice = &input[start_pos..input.len() - amount + i];
        let max_pos = slice
            .iter()
            .enumerate()
            .fold(0, |s, (idx, e)| if *e > slice[s] { idx } else { s });
        num *= 10;
        num += slice[max_pos];
        start_pos += max_pos + 1;
    }
    println!("Vector: {input:?}\nJolt: {num}");
    return Ok(num);
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<u64>> = util::parse::with_nom(&path, parse_lists)?;
    let mut sum = 0;
    for ve in v {
        sum += battery_jolt(ve, 2)?;
    }
    println!("Sum of jolts: {sum}");
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<u64>> = util::parse::with_nom(&path, parse_lists)?;
    let mut sum = 0;
    for ve in v {
        sum += battery_jolt(ve, 12)?;
    }
    println!("Sum of jolts: {sum}");
    Ok(())
}
