use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list0,
    sequence::separated_pair, IResult,
};
use util;

util::main![pt1, pt2];

fn parse_inventory(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (remainder, (iv, fi)) = separated_pair(
        separated_list0(tag("\n"), separated_pair(u64, tag("-"), u64)),
        tag("\n\n"),
        separated_list0(tag("\n"), u64),
    )(input)?;
    let inventory = iv.iter().map(|(a, b)| *a..=*b).collect();
    Ok((remainder, (inventory, fi)))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (inventory, fruit_ids): (Vec<RangeInclusive<u64>>, Vec<u64>) =
        util::parse::with_nom(&path, parse_inventory)?;
    let mut fresh = 0;
    for id in fruit_ids {
        for r in &inventory {
            if r.contains(&id) {
                println!("{id} is fresh, because it's contained in {r:?}");
                fresh += 1;
                break;
            }
        }
    }
    println!("There are {fresh} fresh fruit in total.");
    Ok(())
}

fn merge_sorted_ranges(
    r1: &RangeInclusive<u64>,
    r2: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    if r1.end() >= r2.start() {
        if r1.end() > r2.end() {
            return Some(*r1.start()..=*r1.end());
        }
        else {
            return Some(*r1.start()..=*r2.end());
        }
    }
    None
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (mut inventory, _fruit_ids): (Vec<RangeInclusive<u64>>, Vec<u64>) =
        util::parse::with_nom(&path, parse_inventory)?;

    inventory.sort_by(|r1, r2| r1.start().cmp(r2.start()));

    let mut iter = inventory.iter().peekable();
    let mut v: Vec<RangeInclusive<u64>> = Vec::new();
    let mut kept: Option<RangeInclusive<u64>> = None;
    while let Some(r) = iter.next() {
        let Some(rkept) = &kept else {
            kept = Some(r.clone());
            continue;
        };
        if let Some(merged) = merge_sorted_ranges(&rkept, &r) {
            kept = Some(merged);
            continue;
        }
        v.push(rkept.clone());
        kept = Some(r.clone());
    }
    if let Some(rkept) = &kept {
        v.push(rkept.clone());
    }

    let mut sum = 0;
    for r in v {
        println!("{}-{}", r.start(), r.end());
        sum += r.count();
    }

    println!("Count of all elements: {sum}");
    Ok(())
}
