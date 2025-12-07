use util;

util::main![pt1, pt2];

use nom::{bytes::complete::tag, character::complete::digit0, multi::separated_list0, IResult};

fn parse_lists(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (remainder, v) = separated_list0(tag("\n"), digit0)(input)?;
    Ok((
        remainder,
        v.iter()
            .map(|e| e.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect(),
    ))
}

fn battery_jolt(input: Vec<u32>, _amount: usize) -> Result<u32, Box<dyn std::error::Error>> {
    // input: the battery vector to examine
    // amount: the amount of batteries
    let max1_pos = input[..input.len() - 1].iter().enumerate().fold(0, |s, (idx, e)|{
        if *e > input[s] {
            idx
        } else {
            s
        }
    });
    let max2 = input[max1_pos + 1..].iter().max().ok_or("No max")?;
    let num = input[max1_pos] * 10 + max2;
    println!("Jolt: {num}");
    return Ok(num)

}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<u32>> = util::parse::with_nom(&path, parse_lists)?;
    let mut sum = 0;
    for ve in v {
        sum += battery_jolt(ve, 2)?;
    }
    println!("Sum of jolts: {sum}");
    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
