use crate::advent::util;
use nom::character::complete::u64;
use nom::multi::separated_list0;
use nom::{bytes::complete::tag, IResult};

fn parse_stones(input: &str) -> IResult<&str, Vec<u64>> {
    let (remainder, v) = separated_list0(tag(" "), u64)(input)?;
    Ok((remainder, v))
}

fn blink(mut v: Vec<u64>, blinks: usize) {
    for _i in 0..blinks {
        v = v
            .iter()
            .flat_map(|&stone| {
                if stone == 0 {
                    return vec![1];
                }
                let s = stone.to_string();
                if s.len() % 2 == 0 {
                    let (ls, rs) = s.split_at(s.len() / 2);
                    vec![
                        u64::from_str_radix(ls, 10).unwrap(),
                        u64::from_str_radix(rs, 10).unwrap(),
                    ]
                } else {
                    vec![stone * 2024]
                }
            })
            .collect();
        println!("Performed {} iterations", _i);
    }
    println!("Amount of stones: {}", v.len());
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<u64> = util::parse::with_nom(&path, parse_stones)?;

    blink(v, 25);

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<u64> = util::parse::with_nom(&path, parse_stones)?;

    blink(v, 75);

    Ok(())
}
