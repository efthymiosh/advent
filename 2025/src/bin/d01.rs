use util;

util::main![pt1, pt2];

use nom::{
    bytes::complete::tag,
    character::complete::{i32, one_of},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

fn parse_lists(input: &str) -> IResult<&str, Vec<(char, i32)>> {
    let (remainder, v) = separated_list0(tag("\n"), tuple((one_of("LR"), i32)))(input)?;
    Ok((remainder, v))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(char, i32)> = util::parse::with_nom(&path, parse_lists)?;

    let diff_sum: i32 = v
        .iter()
        .scan((50, 0), |(state, _), (dir, clicks)| {
            let x = match *dir {
                'L' => (100 + *state - clicks) % 100,
                'R' => (100 + *state + clicks) % 100,
                _ => unreachable!(),
            };
            *state = x;
            Some(if x == 0 { 1 } else { 0 })
        })
        .sum();

    println!("Amount of times dial at zero: {}", diff_sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(char, i32)> = util::parse::with_nom(&path, parse_lists)?;

    let diff_sum: i32 = v
        .iter()
        .scan((50, 0), |(state, _), (dir, clicks)| {
            let mut count = clicks / 100;
            let clicks = clicks % 100;
            let x = match *dir {
                'L' => (100 + *state - clicks) % 100,
                'R' => (100 + *state + clicks) % 100,
                _ => unreachable!(),
            };
            // If we were on a zero before turning the dials we don't need to do anything:
            // We've already counted the zero on the previous turn
            if *state != 0 {
                // If we landed on a zero, or passed through the zero, count it
                if x == 0 || (*dir == 'L' && x >= *state) || (*dir == 'R' && x <= *state) {
                    count += 1;
                }
            }
            *state = x;
            Some(count)
        })
        .sum();

    println!("Amount of times dial passed through zero: {}", diff_sum);
    Ok(())
}
