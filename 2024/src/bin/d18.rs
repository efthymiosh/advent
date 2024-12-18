use std::collections::{BinaryHeap, HashSet};

use nom::character::complete::u32;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, IResult};

use util;

util::main![pt1, pt2];

fn parse_positions(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (remainder, v) = separated_list0(tag("\n"), separated_pair(u32, tag(","), u32))(input)?;
    Ok((remainder, v))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: u32,
    pos: (i64, i64),
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

fn shortest_path(grid: &mut Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let size = grid.len();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {
        pos: (start.0 as i64, start.1 as i64),
        cost: 0,
    });

    while let Some(state) = heap.pop() {
        let p = state.pos;
        if p.0 < 0 || p.1 < 0 || p.0 >= size as i64 || p.1 >= size as i64 {
            continue;
        }
        if (p.0 as usize, p.1 as usize) == end {
            return Some(state.cost)
        }
        if grid[p.0 as usize][p.1 as usize] == '#' {
            continue;
        }
        for pos in [(p.0 - 1, p.1), (p.0 + 1, p.1), (p.0, p.1 - 1), (p.0, p.1 + 1)] {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            heap.push(State { cost: state.cost + 1, pos });
        }
    }
    None
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let positions = util::parse::with_nom(&path, parse_positions)?;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    for (x, y) in positions.into_iter().take(1024) {
        grid[x as usize][y as usize] = '#'
    }

    let start = (0, 0);
    let end = (grid.len() - 1, grid[0].len() - 1);
    let cost = shortest_path(&mut grid, start, end).ok_or("Maze has no solution!")?;

    println!("Least cost path: {}", cost);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let positions = util::parse::with_nom(&path, parse_positions)?;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    for (x, y) in positions.into_iter() {
        grid[x as usize][y as usize] = '#';
        let start = (0, 0);
        let end = (grid.len() - 1, grid[0].len() - 1);
        if let Some(_) = shortest_path(&mut grid, start, end) {
            continue;
        } else {
            println!("Coordinates of first byte that blocks solution: {},{}", x, y);
            break;
        }
    }


    Ok(())
}
