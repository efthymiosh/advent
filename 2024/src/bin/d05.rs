use util;
use nom::character::complete::u32;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, IResult};

advent2024::main![pt1,pt2];

fn parse_inputs(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (remainder, pair) = separated_pair(
        separated_list0(tag("\n"), separated_pair(u32, tag("|"), u32)),
        tag("\n\n"),
        separated_list0(tag("\n"), separated_list0(tag(","), u32)),
    )(input)?;
    Ok((remainder, pair))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (vrules, vlist) = util::parse::with_nom(&path, parse_inputs)?;

    let sum: u32 = vlist
        .iter()
        .map(|vec| {
            for (preceed, succeed) in &vrules {
                let Some(pos) = vec.iter().position(|n| n == preceed) else {
                    continue;
                };
                let None = vec.iter().take(pos).position(|n| n == succeed) else {
                    return 0;
                };
            }
            return vec[vec.len() / 2];
        })
        .sum();

    println!("sum: {}", sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (vrules, mut vlist) = util::parse::with_nom(&path, parse_inputs)?;

    let sum: u32 = vlist
        .iter_mut()
        .map(|vec| {
            let mut corrected = false;
            let mut repeat = true;
            while repeat {
                repeat = false;
                for (preceed, succeed) in &vrules {
                    let Some(pos) = vec.iter().position(|n| n == preceed) else {
                        continue;
                    };
                    if let Some(posv) = vec.iter().take(pos).position(|n| n == succeed) {
                        vec.swap(pos, posv);
                        repeat = true;
                        corrected = true;
                        break;
                    };
                }
            }
            if corrected {
                return vec[vec.len() / 2];
            }
            else {
                return 0;
            }
        })
        .sum();

    println!("sum: {}", sum);
    Ok(())
}
