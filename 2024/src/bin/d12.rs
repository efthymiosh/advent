use nom::character::complete::none_of;
use nom::multi::{many1, separated_list0};
use nom::{bytes::complete::tag, IResult};

use util;

advent2024::main![pt1,pt2];

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (remainder, v) = separated_list0(tag("\n"), many1(none_of("\n")))(input)?;
    Ok((remainder, v))
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum D {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn search(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    x: isize,
    y: isize,
    item: char,
    d: D,
) -> (Option<Vec<(usize, usize)>>, Option<Vec<(isize, isize, D)>>) {
    if x < 0 || y < 0 || x as usize >= grid.len() || y as usize >= grid.len() {
        return (None, Some(vec![(x, y, d)]));
    }
    let (xu, yu) = (x as usize, y as usize);
    if grid[xu][yu] != item {
        return (None, Some(vec![(x, y, d)]));
    }
    if visited[xu][yu] {
        return (None, Some(vec![]));
    }
    visited[xu][yu] = true;

    let mut ret = vec![(xu, yu)];
    let mut fences = vec![];
    for (nx, ny, d) in &[
        (x - 1, y, D::UP),
        (x + 1, y, D::DOWN),
        (x, y - 1, D::LEFT),
        (x, y + 1, D::RIGHT),
    ] {
        match search(grid, visited, *nx, *ny, item, *d) {
            (None, Some(mut vf)) => {
                fences.append(&mut vf);
            }
            (Some(mut v), Some(mut vf)) => {
                ret.append(&mut v);
                fences.append(&mut vf);
            }
            _ => unreachable!(),
        }
    }
    return (Some(ret), Some(fences));
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; v[0].len()]; v.len()];

    let mut sum = 0;
    for x in 0..v.len() {
        for y in 0..v[0].len() {
            let (Some(ve), Some(vf)) =
                search(&v, &mut visited, x as isize, y as isize, v[x][y], D::UP)
            else {
                continue;
            };
            sum += ve.len() * vf.len();
        }
    }

    println!("Total price is: {}", sum);

    Ok(())
}

fn reduce(mut vf: Vec<(isize, isize, D)>) -> usize {
    let mut ret = 0;
    // sorting the vec so that we only need to reduce in one direction
    // by popping the last element and checking for any previous
    vf.sort();
    while let Some((a, b, d)) = vf.pop() {
        if vf.binary_search(&(a, b - 1, d)).is_ok() || vf.binary_search(&(a - 1, b, d)).is_ok() {
            continue;
        }
        ret += 1;
    }

    ret
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; v[0].len()]; v.len()];

    let mut sum = 0;
    for x in 0..v.len() {
        for y in 0..v[0].len() {
            let (Some(ve), Some(vf)) =
                search(&v, &mut visited, x as isize, y as isize, v[x][y], D::UP)
            else {
                continue;
            };
            let r = reduce(vf);
            sum += ve.len() * r;
        }
    }

    println!("Total price is: {}", sum);

    Ok(())
}
