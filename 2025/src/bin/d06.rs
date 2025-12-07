use nom::{
    IResult, bytes::complete::tag, character::complete::{one_of, u64}, multi::{many1, separated_list0}, sequence::tuple
};
use util;

util::main![pt1, pt2];

fn parse_math(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>, Vec<char>)> {
    let (remainder, (v1, _, v2, _, v3, _, v4, _, v5)) = tuple((
        separated_list0(
            many1(tag(" ")),
            u64
        ),
        tag("\n"),
        separated_list0(
            many1(tag(" ")),
            u64
        ),
        tag("\n"),
        separated_list0(
            many1(tag(" ")),
            u64
        ),
        tag("\n"),
        separated_list0(
            many1(tag(" ")),
            u64
        ),
        tag("\n"),
        separated_list0(
            many1(tag(" ")),
            one_of("+*")
        )
    ))(input)?;
    Ok((remainder, (
        v1,
        v2,
        v3,
        v4,
        v5
    )))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (v1, v2, v3, v4, v5)  = util::parse::with_nom(&path, parse_math)?;
    let mut sum = 0;
    for i in 0..v1.len() {
        let result = 
        match v5[i] {
            '*' => v1[i] * v2[i] * v3[i] * v4[i],
            '+' => v1[i] + v2[i] + v3[i] + v4[i],
            _ => unreachable!(),
        };
        sum += result;
    }
    println!("Result: {sum}");
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (_v1, _v2, _v3, _v4, _v5)  = util::parse::with_nom(&path, parse_math)?;
    Ok(())
}
