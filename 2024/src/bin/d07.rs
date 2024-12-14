use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list0,
    sequence::separated_pair, IResult,
};
use util;

util::main![pt1, pt2];

fn parse_equations(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    let (remainder, v) = separated_list0(
        tag("\n"),
        separated_pair(u64, tag(": "), separated_list0(tag(" "), u64)),
    )(input)?;
    Ok((remainder, v))
}

fn matchop(sum: u64, partial: u64, ve: &[u64]) -> bool {
    if partial > sum {
        return false;
    }
    if ve.len() == 0 {
        return sum == partial;
    }
    return matchop(sum, partial + ve[0], &ve[1..]) || matchop(sum, partial * ve[0], &ve[1..]);
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(u64, Vec<u64>)> = util::parse::with_nom(&path, parse_equations)?;

    let sum = v
        .iter()
        .filter_map(|(s, ve)| {
            if matchop(*s, ve[0], &ve[1..]) {
                Some(s)
            } else {
                None
            }
        })
        .sum::<u64>();

    println!("Sum: {}", sum);

    Ok(())
}

fn matchop2(sum: u64, partial: u64, ve: &[u64]) -> bool {
    if partial > sum {
        return false;
    }
    if ve.len() == 0 {
        return sum == partial;
    }
    return matchop2(sum, partial + ve[0], &ve[1..])
        || matchop2(sum, partial * ve[0], &ve[1..])
        || matchop2(
            sum,
            u64::from_str_radix(&(partial.to_string() + &ve[0].to_string()), 10).unwrap(),
            &ve[1..],
        );
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(u64, Vec<u64>)> = util::parse::with_nom(&path, parse_equations)?;

    let sum = v
        .iter()
        .filter_map(|(s, ve)| {
            if matchop2(*s, ve[0], &ve[1..]) {
                Some(s)
            } else {
                None
            }
        })
        .sum::<u64>();

    println!("Sum: {}", sum);

    Ok(())
}
