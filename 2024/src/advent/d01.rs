use crate::advent::util::parse;
use nom::{
    bytes::complete::tag, character::complete::u32, multi::separated_list0,
    sequence::separated_pair, IResult,
};

fn parse_lists(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (remainder, v) = separated_list0(tag("\n"), separated_pair(u32, tag("   "), u32))(input)?;
    let (vl, vr): (Vec<u32>, Vec<u32>) = v.into_iter().unzip();
    Ok((remainder, (vl, vr)))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (mut vl, mut vr): (Vec<u32>, Vec<u32>) = parse::with_nom(&path, parse_lists)?;
    vl.sort();
    vr.sort();

    let diff_sum: u32 = vl.iter().zip(vr).map(|(a, b)| a.abs_diff(b)).sum();

    println!("Sum of diffs: {}", diff_sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (vl, vr): (Vec<u32>, Vec<u32>) = parse::with_nom(&path, parse_lists)?;

    let similarity_sum: u32 = vl
        .iter()
        .map(|l| vr.iter().filter(|&r| r == l).count() as u32 * l)
        .sum();

    println!("Similarity sum: {}", similarity_sum);
    Ok(())
}
