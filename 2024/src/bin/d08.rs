use std::collections::{HashMap, HashSet};

use advent2024::advent::util;
use nom::{
    bytes::complete::tag,
    character::complete::none_of,
    multi::{many1, separated_list0},
    IResult,
};

use itertools::Itertools;

advent2024::main![pt1,pt2];

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (remainder, v) = separated_list0(tag("\n"), many1(none_of("\n")))(input)?;
    Ok((remainder, v))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut grid: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (x, ve) in grid.iter_mut().enumerate() {
        for (y, e) in ve.iter_mut().enumerate() {
            if *e == '.' {
                continue;
            }
            if let Some(v) = antennas.get_mut(e) {
                v.push((x as isize, y as isize));
            } else {
                antennas.insert(*e, vec![(x as isize, y as isize)]);
            }
        }
    }

    let mut antinodes = HashSet::new();
    util::grid::print(&grid, 1, '.');
    for ve in antennas.values() {
        for pair in ve.iter().permutations(2) {
            let a = pair[0];
            let b = pair[1];
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            let check = [(a.0 + dx, a.1 + dy), (b.0 - dx, b.1 - dy)];
            for (x, y) in check {
                if x >= 0 && x < grid.len() as isize && y >= 0 && y < grid.len() as isize {
                    antinodes.insert((x, y));
                }
            }
        }
    }

    println!("Antinodes: {:?}\nAmount: {}", antinodes, antinodes.len());
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut grid: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (x, ve) in grid.iter_mut().enumerate() {
        for (y, e) in ve.iter_mut().enumerate() {
            if *e == '.' {
                continue;
            }
            if let Some(v) = antennas.get_mut(e) {
                v.push((x as isize, y as isize));
            } else {
                antennas.insert(*e, vec![(x as isize, y as isize)]);
            }
        }
    }

    let mut antinodes = HashSet::new();
    util::grid::print(&grid, 1, '.');
    for ve in antennas.values() {
        for pair in ve.iter().permutations(2) {
            let a = pair[0];
            let b = pair[1];
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            let mut x = a.0;
            let mut y = a.1;
            while x >= 0 && x < grid.len() as isize && y >= 0 && y < grid.len() as isize {
                antinodes.insert((x, y));
                x += dx;
                y += dy;
            }
            x = b.0;
            y = b.1;
            while x >= 0 && x < grid.len() as isize && y >= 0 && y < grid.len() as isize {
                antinodes.insert((x, y));
                x -= dx;
                y -= dy;
            }
        }
    }

    println!("Antinodes: {:?}\nAmount: {}", antinodes, antinodes.len());
    Ok(())
}
