use std::collections::HashMap;

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

fn assign_costs(
    grid: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    costs: &mut Vec<Vec<Option<u32>>>,
    (x, y): (isize, isize),
    end: char,
    dist: u32,
) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    costs[x as usize][y as usize] = if let Some(m) = costs[x as usize][y as usize] {
        Some((m).min(dist))
    } else {
        Some(dist)
    };
    path.push((x as usize, y as usize));
    let neighbours = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    for &(dx, dy) in neighbours {
        if x + dx < 0 || x + dx >= grid.len() as isize {
            continue;
        }
        if y + dy < 0 || y + dy >= grid.len() as isize {
            continue;
        }
        let nx = (x + dx) as usize;
        let ny = (y + dy) as usize;
        if grid[nx][ny] == '#' || visited[nx][ny] {
            continue;
        }
        visited[nx][ny] = true;
        path.append(&mut assign_costs(
            grid,
            visited,
            costs,
            (x + dx, y + dy),
            end,
            dist + 1,
        ));
        println!("{} {}: path {:?}", x, y, path);
        visited[nx][ny] = false;
    }
    path
}

fn advance(
    cost: u32,
    p: (usize, usize),
    d: (isize, isize),
) -> Vec<(u32, (usize, usize), (isize, isize))> {
    let (x, y) = p;
    vec![
        (cost, (x - 1, y), d),
        (cost, (x + 1, y), d),
        (cost, (x, y - 1), d),
        (cost, (x, y + 1), d),
    ]
}

fn solve(grid: &mut Vec<Vec<char>>, pt2: bool) -> Result<u32, Box<dyn std::error::Error>> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut costs = vec![vec![None; grid[0].len()]; grid.len()];

    let (sx, sy) = util::grid::find(&grid, &'S').ok_or("Unable to find initial start position")?;
    let (ex, ey) = util::grid::find(&grid, &'E').ok_or("Unable to find initial start position")?;
    let start = (sx as isize, sy as isize);
    let path = assign_costs(grid, &mut visited, &mut costs, start, 'E', 0);

    let unwrapped_costs: Vec<Vec<u32>> = costs
        .iter()
        .map(|vc| {
            vc.iter()
                .map(|o| if let Some(x) = o { *x } else { u32::MAX })
                .collect::<Vec<u32>>()
        })
        .collect();

    util::grid::print(&unwrapped_costs, 3, u32::MAX);

    let mut hm = HashMap::new();
    if !pt2 {
        // traverse the path from end to start
        let (mut x, mut y) = (ex as isize, ey as isize);
        while (x, y) != (sx as isize, sy as isize) {
            // check if a barricade would lower solution by > 100
            let neighbours = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
            for &(dx, dy) in neighbours {
                if x + 2 * dx < 0 || x + 2 * dx >= grid.len() as isize {
                    continue;
                }
                if y + 2 * dy < 0 || y + 2 * dy >= grid.len() as isize {
                    continue;
                }
                if grid[(x + dx) as usize][(y + dy) as usize] != '#' {
                    continue;
                }
                let Some(cost) = costs[x as usize][y as usize] else {
                    continue;
                };
                let Some(ncost) = costs[(x + 2 * dx) as usize][(y + 2 * dy) as usize] else {
                    continue;
                };
                let savings = cost as isize - ncost as isize - 2;
                if savings >= 100 {
                    println!("Shortcut at {},{} saves {}", x + dx, y + dy, savings);
                    if let Some(x) = hm.get(&savings) {
                        hm.insert(savings, x + 1);
                    } else {
                        hm.insert(savings, 1);
                    }
                }
            }
            for &(dx, dy) in neighbours {
                if x + dx < 0 || x + dx >= grid.len() as isize {
                    continue;
                }
                if y + dy < 0 || y + dy >= grid.len() as isize {
                    continue;
                }
                let Some(cost) = costs[x as usize][y as usize] else {
                    continue;
                };
                let Some(ncost) = costs[(x + dx) as usize][(y + dy) as usize] else {
                    continue;
                };
                if cost as isize - ncost as isize == 1 {
                    (x, y) = (x + dx, y + dy);
                    break;
                }
            }
        }
    } else {
        println!("Race path: {:?}\n len: {}", path, path.len());
        for (idx, i) in path.iter().enumerate() {
            for j in path.iter().skip(idx + 1) {
                let Some(cost) = search::dijkstra_cutoff(&grid, &advance, *i, *j, '.', Some(20))
                else {
                    continue;
                };
                let savings = costs[j.0][j.1].unwrap() as isize
                    - costs[i.0][i.1].unwrap() as isize
                    - cost as isize;
                if savings >= 50 {
                    if let Some(x) = hm.get(&savings) {
                        hm.insert(savings, x + 1);
                    } else {
                        hm.insert(savings, 1);
                    }
                }
            }
        }
    }
    let mut sum = 0;
    let mut v: Vec<_> = hm.iter().collect();
    v.sort();
    for (k, v) in v {
        println!("There's {} cheat(s) that save {} picoseconds", v, k);
        sum += v;
    }
    Ok(sum)
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = util::parse::with_nom(&path, parse_grid)?;
    println!("Savings: {:?}", solve(&mut grid, false)?);

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = util::parse::with_nom(&path, parse_grid)?;
    println!("Savings: {:?}", solve(&mut grid, true)?);
    Ok(())
}
