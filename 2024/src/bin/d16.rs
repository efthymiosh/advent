use nom::character::complete::none_of;
use nom::multi::{many1, separated_list0};
use nom::{bytes::complete::tag, IResult};

use util;
use util::algorithms::search;

util::main![pt1, pt2];

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (remainder, v) = separated_list0(tag("\n"), many1(none_of("\n")))(input)?;
    Ok((remainder, v))
}

fn advance(cost: u32, p: (usize, usize), d: (isize, isize)) -> Vec<(u32, (usize, usize), (isize, isize))> {
    let mut ret = vec![
        (cost + 1000, p, (d.1, -d.0)),
        (cost + 1000, p, (-d.1, d.0)),
    ];
    let (x, y) = (p.0 as isize + d.0, p.1 as isize + d.1);
    if x >= 0 && y >= 0 {
        ret.push((cost + 1, (x as usize, y as usize), d));
    }
    ret
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let grid: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let start = util::grid::find(&grid, &'S').ok_or("Unable to find initial start position")?;
    let end = util::grid::find(&grid, &'E').ok_or("Unable to find initial start position")?;
    let cost = search::dijkstra(&grid, &advance, start, end, '#').ok_or("Maze has no solution!")?;

    println!("Least cost path: {}", cost);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let grid: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let start = util::grid::find(&grid, &'S').ok_or("Unable to find initial start position")?;
    let end = util::grid::find(&grid, &'E').ok_or("Unable to find initial start position")?;
    let cost = search::dijkstra(&grid, &advance, start, end, '#').ok_or("Maze has no solution!")?;

    println!("Least cost path: {}", cost);
    Ok(())
}
