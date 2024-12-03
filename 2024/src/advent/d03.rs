use crate::advent::util::parse;
use nom::sequence::tuple;
use nom::{
    bytes::complete::tag,
    character::complete::u32,
    IResult,
};

fn parse_instructions(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let mut v: Vec<(u32, u32)> = vec![];
    let mut remainder = input;
    while remainder.len() > 0 {
        if let Ok((rem, (_, a, _, b, _))) =
            tuple((tag("mul("), u32::<&str, nom::error::Error<&str>>, tag(","), u32, tag(")")))(remainder)
        {
            v.push((a, b));
            remainder = rem;
            continue;
        }
        remainder = &remainder[1..];
    }
    Ok((remainder, v))
}

fn parse_instructions_pt2(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let mut v: Vec<(u32, u32)> = vec![];
    let mut remainder = input;
    let mut accept: bool = true;
    while remainder.len() > 0 {
        if let Ok((rem, (_, a, _, b, _))) =
            tuple((tag("mul("), u32::<&str, nom::error::Error<&str>>, tag(","), u32, tag(")")))(remainder)
        {
            if accept {
                v.push((a, b));
            }
            remainder = rem;
        } else if let Ok((rem, _)) = tag::<&str, &str, nom::error::Error<&str>>("do()")(remainder) {
            accept = true;
            remainder = rem;
        } else if let Ok((rem, _)) = tag::<&str, &str, nom::error::Error<&str>>("don't()")(remainder) {
            accept = false;
            remainder = rem;
        } else {
            remainder = &remainder[1..];
        }
    }
    Ok((remainder, v))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(u32, u32)> = parse::with_nom(&path, parse_instructions)?;

    let sum_mul = v.iter().fold(0, |acc, &(a, b)| acc + a * b);
    println!("Sum of multiplications: {}", sum_mul);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(u32, u32)> = parse::with_nom(&path, parse_instructions_pt2)?;

    let sum_mul = v.iter().fold(0, |acc, &(a, b)| acc + a * b);
    println!("Sum of multiplications: {}", sum_mul);
    Ok(())
}
