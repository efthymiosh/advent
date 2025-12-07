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

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<u32>> = util::parse::with_nom(&path, parse_lists)?;
    let mut sum = 0;
    for ve in v {
        let max1_pos = ve[..ve.len() - 1].iter().enumerate().fold(0, |s, (idx, e)|{
            if *e > ve[s] {
                idx
            } else {
                s
            }
        });
        let max2 = ve[max1_pos + 1..].iter().max().ok_or("No max")?;
        let num = ve[max1_pos] * 10 + max2;
        println!("Jolt: {num}");
        sum += num;
    }
    println!("Sum of jolts: {sum}");
    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
