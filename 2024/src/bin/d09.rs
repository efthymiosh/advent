use advent2024::advent::util;
use nom::{character::complete::anychar, multi::many1, IResult};

advent2024::main![pt1,pt2];

fn parse(input: &str) -> IResult<&str, Vec<Option<u64>>> {
    let (remainder, s) = many1(anychar)(input)?;
    // explode the state into the fileblocks
    let mut id = 0;
    let mut id_flag = true;
    Ok((
        remainder,
        s.iter()
            .flat_map(|&c| {
                let mut digit = c.to_digit(10).unwrap() as u64;
                let mut v = vec![];
                while digit > 0 {
                    digit -= 1;
                    v.push(if id_flag { Some(id) } else { None });
                }
                if id_flag {
                    id += 1;
                }
                id_flag = !id_flag;
                v
            })
            .collect(),
    ))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<Option<u64>> = util::parse::with_nom(&path, parse)?;

    let mut i = 0;
    let mut r = v.len() - 1;
    loop {
        while let Some(&x) = v.get(i) {
            if x == None {
                break;
            }
            i += 1;
        }
        // Sitting at an empty spot. Move an item from the end
        if r <= i {
            break;
        }
        v.swap(i, r);
        r -= 1;
    }
    let sum = v.iter().enumerate().map(|(i, o)| {
        let Some(id) = o else {
            return 0;
        };
        i as u64 * id
    }).sum::<u64>();

    println!("fileblocks: {:?}\nChecksum: {}", v, sum);
    Ok(())
}

const ZID: u64 = u64::MAX;

fn parse_pt2(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (remainder, s) = many1(anychar)(input)?;
    // explode the state into the fileblocks
    let mut id = 0;
    let mut id_flag = true;
    Ok((
        remainder,
        s.iter()
            .map(|&c| {
                let digit = c.to_digit(10).unwrap() as u64;
                let ret = if id_flag {
                    id += 1;
                    (id - 1, digit)
                } else {
                    (ZID, digit)
                };
                id_flag = !id_flag;
                ret
            })
            .collect(),
    ))
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<(u64, u64)> = util::parse::with_nom(&path, parse_pt2)?;

    let mut i = 0;
    loop {
        while let Some(&x) = v.get(i) {
            if x.0 == ZID {
                break;
            }
            i += 1;
        }
        if i >= v.len() {
            break;
        }
        // pos (i) sitting at an empty spot. Pull something smaller.
        // Starting from the end, find the next non-zero position with a smaller amount.
        let mut temp_r = v.len() - 1;
        while temp_r > 0 && (v[temp_r].1 > v[i].1 || v[temp_r].0 == ZID) {
            temp_r -= 1;
        }
        if temp_r < i {
            i += 1;
            continue;
        }
        if v[temp_r].1 == v[i].1 {
            v.swap(i, temp_r);
        } else { // v[temp_r].1 < v[i].1
            v[i].1 -= v[temp_r].1;
            v.insert(i, v[temp_r]);
            v[temp_r + 1].0 = ZID;
        }
    }
    let mut pos = 0;
    let sum = v.iter().map(|(id, amt)| {
        if *id == ZID {
            pos += amt;
            return 0;
        }
        let mut ret = 0;
        for _ in 0..*amt {
            ret += id * pos;
            pos += 1;
        }
        ret
    }).sum::<u64>();

    println!("\nChecksum: {}", sum);
    Ok(())
}
