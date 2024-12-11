use crate::advent::util;
use nom::character::complete::u64;
use nom::multi::separated_list0;
use nom::{bytes::complete::tag, IResult};

fn parse_stones(input: &str) -> IResult<&str, Vec<u64>> {
    let (remainder, v) = separated_list0(tag(" "), u64)(input)?;
    Ok((
        remainder,
        v
    ))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<u64> = util::parse::with_nom(&path, parse_stones)?;

    for _i in 0..25 {
        v = v.iter().flat_map(|&stone|{
            let s = stone.to_string();
            if stone == 0 {
                vec![1]
            }
            else if s.len() % 2 ==0 {
                let (ls, rs) = s.split_at(s.len() / 2);
                vec![u64::from_str_radix(ls, 10).unwrap(), u64::from_str_radix(rs, 10).unwrap()]
            } else {
                vec![stone * 2024]
            }
        }).collect();
        println!("After {} iterations: {:?}", _i, v);
    }
    println!("Amount of stones: {}", v.len());

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<u64> = util::parse::with_nom(&path, parse_stones)?;

    println!("Amount of stones: {}", v.len());

    Ok(())
}
