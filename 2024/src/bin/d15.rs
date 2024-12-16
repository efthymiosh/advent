use itertools::Itertools;
use nom::character::complete::none_of;
use nom::multi::{many1, separated_list0};
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, IResult};
use util;

util::main![pt1, pt2];

fn parse_grid(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<(isize, isize)>)> {
    let (remainder, (grid, moves)) = separated_pair(
        separated_list0(tag("\n"), many1(none_of("\n"))),
        tag("\n\n"),
        separated_list0(tag("\n"), many1(none_of("\n"))),
    )(input)?;
    let moves = moves
        .iter()
        .flat_map(|vc| {
            vc.iter().map(|c| match *c {
                '^' => (-1, 0),
                '>' => (0, 1),
                '<' => (0, -1),
                'v' => (1, 0),
                _ => unreachable!(),
            })
        })
        .collect();

    Ok((remainder, (grid, moves)))
}

fn move_object(
    grid: &mut Vec<Vec<char>>,
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> Option<(usize, usize)> {
    // unchecked, walls all around.
    let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

    match grid[x][y] {
        '.' => Some((nx, ny)), // we are trying to move air. Air always "moves".
        '#' => None,           // we are trying to move a piece of wall. It doesn't.
        // try to move the next thing. if it moves, move
        obj @ ('O' | '@') => match move_object(grid, (nx, ny), (dx, dy)) {
            Some((_, _)) => {
                grid[x][y] = '.';
                grid[nx][ny] = obj;
                Some((nx, ny))
            }
            None => None,
        },
        _ => unreachable!(),
    }
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (mut grid, moves): (Vec<Vec<char>>, Vec<(isize, isize)>) =
        util::parse::with_nom(&path, parse_grid)?;

    let (mut rx, mut ry) = grid
        .iter()
        .enumerate()
        .find_map(|(x, ve)| ve.iter().position(|&n| n == '@').map(|y| (x, y)))
        .ok_or("Unable to find initial robot position")?;

    for m in moves {
        match move_object(&mut grid, (rx, ry), m) {
            Some((x, y)) => (rx, ry) = (x, y),
            None => {}
        }
        //util::grid::print(&grid, 1, '.');
        //util::debug::wait_ms(100);
    }

    let sum = grid
        .iter()
        .enumerate()
        .map(|(x, ve)| {
            ve.iter()
                .enumerate()
                .filter_map(|(y, c)| if *c == 'O' { Some(100 * x + y) } else { None })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("Sum of scoring: {}", sum);

    Ok(())
}

fn check_move_object(
    grid: &mut Vec<Vec<char>>,
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> Option<Vec<((usize, usize), (usize, usize))>> {
    // unchecked, walls all around.
    let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

    match grid[x][y] {
        '.' => Some(vec![]), // we are trying to move air. Air always "moves".
        '#' => None,         // we are trying to move a piece of wall. It doesn't.
        // gather all object moves required
        obj @ ('[' | ']') if dx != 0 => {
            let Some(mut v1) = check_move_object(grid, (nx, ny), (dx, dy)) else {
                return None;
            };
            let r = if obj == '[' { 1 } else { -1 };
            let ry = (ny as isize + r) as usize;
            let Some(mut v2) = check_move_object(grid, (nx, ry), (dx, dy)) else {
                return None;
            };
            v1.append(&mut v2);
            let mut v: Vec<_> = v1.into_iter().unique().collect();
            v.push(((x, y), (nx, ny)));
            v.push(((x, ((y as isize) + r) as usize), (nx, ry)));
            return Some(v);
        }
        '@' | '[' | ']' => {
            if let Some(mut v) = check_move_object(grid, (nx, ny), (dx, dy)) {
                v.push(((x, y), (nx, ny)));
                Some(v)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (grid, moves): (Vec<Vec<char>>, Vec<(isize, isize)>) =
        util::parse::with_nom(&path, parse_grid)?;

    // restructure for pt2
    let mut grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|ve| {
            ve.into_iter()
                .flat_map(|e| match e {
                    '@' => ['@', '.'],
                    'O' => ['[', ']'],
                    '#' => ['#', '#'],
                    '.' => ['.', '.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let (mut rx, mut ry) = grid
        .iter()
        .enumerate()
        .find_map(|(x, ve)| ve.iter().position(|&n| n == '@').map(|y| (x, y)))
        .ok_or("Unable to find initial robot position")?;

    for m in moves {
        let Some(v) = check_move_object(&mut grid, (rx, ry), m) else {
            continue;
        };
        for (old, new) in v.iter().take(v.len() - 1) {
            grid[new.0][new.1] = grid[old.0][old.1];
            grid[old.0][old.1] = '.';
        }
        // the last move is the robot:
        let (old, new) = v[v.len() - 1];
        grid[old.0][old.1] = '.';
        grid[new.0][new.1] = '@';
        (rx, ry) = new;
        //util::grid::print(&grid, 2, '.');
        //util::debug::wait_ms(100);
    }

    let sum = grid
        .iter()
        .enumerate()
        .map(|(x, ve)| {
            ve.iter()
                .enumerate()
                .filter_map(|(y, c)| if *c == '[' { Some(100 * x + y) } else { None })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("Sum of scoring: {}", sum);

    Ok(())
}
