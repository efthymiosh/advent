use advent2024::advent::util;
use nom::{bytes::complete::tag, character::complete::u32, multi::separated_list0, IResult};

advent2024::main![pt1,pt2];

fn parse_reports(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (remainder, v) = separated_list0(tag("\n"), separated_list0(tag(" "), u32))(input)?;
    Ok((remainder, v))
}

fn is_valid_report(ve: &Vec<u32>) -> bool {
    let increasing = ve.iter().is_sorted_by(|&a, &b| a < b && b - a <= 3);
    let decreasing = ve.iter().is_sorted_by(|&a, &b| a > b && a - b <= 3);
    increasing || decreasing
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<u32>> = util::parse::with_nom(&path, parse_reports)?;

    let amt_safe = v.iter().filter(|ve| is_valid_report(ve)).count();
    println!("Safe reports: {}", amt_safe);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<u32>> = util::parse::with_nom(&path, parse_reports)?;

    let amt_safe = v
        .iter()
        .filter(|&ve| {
            if is_valid_report(ve) {
                return true;
            }
            for i in 0..ve.len() {
                let mut vr = ve.clone();
                vr.remove(i);
                if is_valid_report(&vr) {
                    return true;
                }
            }
            return false;
        })
        .count();
    println!("Safe reports: {}", amt_safe);
    Ok(())
}
