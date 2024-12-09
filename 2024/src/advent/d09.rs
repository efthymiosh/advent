use crate::advent::util;
use nom::{character::complete::anychar, multi::many1, IResult};

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
        if r > i {
            v.swap(i, r);
            r -= 1;
        } else {
            break;
        }
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

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Option<u64>> = util::parse::with_nom(&path, parse)?;

    println!("fileblocks: {:?}", v);
    Ok(())
}
