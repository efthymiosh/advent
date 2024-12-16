use std::collections::{BinaryHeap, HashSet};

use nom::character::complete::none_of;
use nom::multi::{many1, separated_list0};
use nom::{bytes::complete::tag, IResult};

use util;

util::main![pt1, pt2];

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (remainder, v) = separated_list0(tag("\n"), many1(none_of("\n")))(input)?;
    Ok((remainder, v))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: u32,
    pos: (i64, i64),
    d: (i64, i64),
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn shortest_path(grid: &mut Vec<Vec<char>>, start: (usize, usize), end: char) -> Option<u32> {
    let size = grid.len();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {
        pos: (start.0 as i64, start.1 as i64),
        d: (0, 1),
        cost: 0,
    });

    while let Some(state) = heap.pop() {
        let p = state.pos;
        if p.0 < 0 || p.1 < 0 || p.0 >= size as i64 || p.1 >= size as i64 {
            continue;
        }
        match grid[p.0 as usize][p.1 as usize] {
            c if c == end => return Some(state.cost),
            '#' => continue,
            _ => {}
        };
        for (cost, pos, d) in [
            (state.cost + 1, (p.0 + state.d.0, state.pos.1 + state.d.1), state.d),
            (state.cost + 1000, p, (state.d.1, -state.d.0)),
            (state.cost + 1000, p, (-state.d.1, state.d.0)),
        ] {
            if visited.contains(&(pos, d)) {
                continue;
            }
            visited.insert((pos, d));
            heap.push(State { cost, pos, d });
        }
    }
    None
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut grid: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let start = util::grid::find(&grid, &'S').ok_or("Unable to find initial start position")?;
    let cost = shortest_path(&mut grid, start, 'E').ok_or("Maze has no solution!")?;

    println!("Least cost path: {}", cost);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut grid: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let start = util::grid::find(&grid, &'S').ok_or("Unable to find initial start position")?;
    let cost = shortest_path(&mut grid, start, 'E').ok_or("Maze has no solution!")?;

    println!("Least cost path: {}", cost);
    Ok(())
}
