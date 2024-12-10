use crate::advent::util::parse;
use itertools::Itertools;
use nom::character::complete::digit0;
use nom::multi::separated_list0;
use nom::{bytes::complete::tag, IResult};

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (remainder, v) = separated_list0(tag("\n"), digit0)(input)?;
    Ok((
        remainder,
        v.iter()
            .map(|e| e.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect(),
    ))
}

fn search(grid: &Vec<Vec<u32>>, x: usize, y: usize, item: u32) -> Option<Vec<(usize, usize)>> {
    if grid[x][y] != item {
        return None;
    }
    if item == 9 {
        return Some(vec![(x, y)]);
    }
    let mut ret = vec![];
    if x > 0 {
        if let Some(mut v) = search(&grid, x - 1, y, item + 1) {
            ret.append(&mut v);
        }
    }
    if x < grid.len() - 1 {
        if let Some(mut v) = search(&grid, x + 1, y, item + 1) {
            ret.append(&mut v);
        }
    }
    if y > 0 {
        if let Some(mut v) = search(&grid, x, y - 1, item + 1) {
            ret.append(&mut v);
        }
    }
    if y < grid[0].len() - 1 {
        if let Some(mut v) = search(&grid, x, y + 1, item + 1) {
            ret.append(&mut v);
        }
    }
    return Some(ret);
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<u32>> = parse::with_nom(&path, parse_grid)?;

    let sum = v
        .iter()
        .enumerate()
        .map(|(x, ve)| {
            ve.iter()
                .enumerate()
                .filter_map(|(y, &e)| if e == 0 { search(&v, x, y, 0) } else { None })
                .map(|e| e.iter().unique().count())
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Sum of trailhead scores: {}", sum);

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let _v: Vec<Vec<u32>> = parse::with_nom(&path, parse_grid)?;
    Ok(())
}
