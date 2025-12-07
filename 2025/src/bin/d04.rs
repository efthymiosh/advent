use util;

util::main![pt1, pt2];

use nom::{
    bytes::complete::tag,
    character::complete::none_of,
    multi::{many1, separated_list0},
    IResult,
};

const TOILET_ROLL: char = '@';
const EMPTY_SPOT: char = '.';

fn parse_lists(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (remainder, v) = separated_list0(tag("\n"), many1(none_of("\n")))(input)?;
    Ok((
        remainder,
        v.iter()
            .map(|e| {
                e.iter()
                    .map(|c| match c {
                        '@' => TOILET_ROLL,
                        '.' => EMPTY_SPOT,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    ))
}

fn remove_rolls(v: &[Vec<char>]) -> Option<Vec<(usize, usize)>> {
    let mut vret: Vec<(usize, usize)> = Vec::new();
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            if v[i][j] != TOILET_ROLL {
                continue;
            }
            let neighbours: &[(isize, isize)] = &[
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),   (1, 0), (1, 1),
            ];
            let mut nsum = 0;
            for (ni, nj) in neighbours {
                let tx = (i as isize + ni) as usize;
                let ty = (j as isize + nj) as usize;
                if let Some(x) = v.get(tx) {
                    if let Some(e) = x.get(ty) {
                        match *e {
                            TOILET_ROLL => nsum += 1,
                            EMPTY_SPOT => {},
                            _ => unreachable!(),
                        }
                    }
                }
            }
            if nsum < 4 {
                vret.push((i, j));
            }
        }
    }
    if vret.len() != 0 {
        Some(vret)
    } else {
        None
    }
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_lists)?;
    util::grid::print_vec(&v, 3);
    if let Some(vr) = remove_rolls(&v) {
        println!("Accessible toilet rolls: {}", vr.len());
    }
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_lists)?;
    util::grid::print_vec(&v, 3);
    let mut accessible = 0;
    while let Some(vr) = remove_rolls(&v) {
        accessible += vr.len();
        for (i, j) in vr {
            v[i][j] = '.';
        }
        util::grid::print_vec(&v, 3);
    }
    println!("Accessible toilet rolls: {accessible}");
    Ok(())
}
