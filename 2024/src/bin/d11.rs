use std::collections::HashMap;

use util;
use nom::character::complete::u64;
use nom::multi::separated_list0;
use nom::{bytes::complete::tag, IResult};

advent2024::main![pt1,pt2];

fn parse_stones(input: &str) -> IResult<&str, Vec<u64>> {
    let (remainder, v) = separated_list0(tag(" "), u64)(input)?;
    Ok((remainder, v))
}

fn memocount(counts: &mut HashMap<(u64, u64), u64>, stone: u64, moves: u64) -> u64 {
    if let Some(res) = counts.get(&(stone, moves)) {
        return *res;
    }
    if moves == 0 {
        return 1;
    }
    let s = stone.to_string();
    let res = if stone == 0 {
        memocount(counts, 1, moves - 1)
    } else if s.len() % 2 == 0 {
        let (ls, rs) = s.split_at(s.len() / 2);
        let l = u64::from_str_radix(ls, 10).unwrap();
        let r = u64::from_str_radix(rs, 10).unwrap();
        memocount(counts, l, moves - 1) + memocount(counts, r, moves - 1)
    } else {
        memocount(counts, stone * 2024, moves - 1)
    };
    counts.insert((stone, moves), res);
    return res;
}

fn blink(v: Vec<u64>, blinks: u64) {
    let mut counts: HashMap<(u64, u64), u64> = HashMap::new();

    let sum = v.iter().map(|stone| memocount(&mut counts, *stone, blinks)).sum::<u64>();
    println!("Final amount of stones: {}", sum);
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
