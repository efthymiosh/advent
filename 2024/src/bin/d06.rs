use util;
use nom::{
    bytes::complete::tag,
    character::complete::none_of,
    multi::{many1, separated_list0},
    IResult,
};

advent2024::main![pt1,pt2];

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (remainder, v): (&str, Vec<Vec<char>>) =
        separated_list0(tag("\n"), many1(none_of("\n")))(input)?;
    Ok((remainder, v))
}

fn hit_obstacle(x: isize, y: isize, dx: isize, dy: isize) -> (isize, isize, isize, isize) {
    // reset pos, guard has not passed from here
    let rx = x - dx;
    let ry = y - dy;
    // turn 90 degrees to the right
    // seq: (-1, 0) -> (0, 1) -> (1, 0) -> (0, -1)
    let (rdx, rdy) = match (dx, dy) {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => unreachable!("Delta found to be ({}, {})", dx, dy),
    };
    (rx, ry, rdx, rdy)
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;
    let (x, y) = v
        .iter()
        .enumerate()
        .find_map(|(x, ve)| ve.iter().position(|&n| n == '^').map(|y| (x, y)))
        .ok_or("Unable to find initial guard position")?;

    // account for the guard initial pos as a passed tile
    v[x][y] = 'x';
    let mut sum = 1;

    let (mut x, mut y) = (x as isize, y as isize);
    let (mut dx, mut dy): (isize, isize) = (-1, 0);

    while x + dx < v.len() as isize && y + dy < v[0].len() as isize && x + dx >= 0 && y + dy >= 0 {
        x += dx;
        y += dy;
        match v[x as usize][y as usize] {
            'x' => {}
            '#' => (x, y, dx, dy) = hit_obstacle(x, y, dx, dy),
            _ => {
                v[x as usize][y as usize] = 'x';
                sum += 1;
            }
        }
    }

    println!("Guard covers: {} tiles before exiting", sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;
    let (initx, inity) = v
        .iter()
        .enumerate()
        .find_map(|(x, ve)| ve.iter().position(|&n| n == '^').map(|y| (x, y)))
        .ok_or("Unable to find initial guard position")?;

    let (mut x, mut y) = (initx as isize, inity as isize);
    let (mut dx, mut dy): (isize, isize) = (-1, 0);

    // generate obstacles
    let mut obstacles = Vec::new();
    while x + dx < v.len() as isize && y + dy < v[0].len() as isize && x + dx >= 0 && y + dy >= 0 {
        x += dx;
        y += dy;
        match v[x as usize][y as usize] {
            'x' => {}
            '#' => (x, y, dx, dy) = hit_obstacle(x, y, dx, dy),
            _ => {
                v[x as usize][y as usize] = 'x';
                obstacles.push((x, y));
            }
        }
    }

    println!("obstacles to test: {}", obstacles.len());

    let mut sum = 0;
    // test obstacles
    for (xo, yo) in &obstacles {
        let (mut x, mut y) = (initx as isize, inity as isize);
        let (mut dx, mut dy): (isize, isize) = (-1, 0);
        let mut loop_pos = vec![];
        let mut loop_check = false;
        while x + dx < v.len() as isize
            && y + dy < v[0].len() as isize
            && x + dx >= 0
            && y + dy >= 0
        {
            x += dx;
            y += dy;

            if loop_check {
                if loop_pos.contains(&(x, y, dx, dy)) {
                    sum += 1;
                    break;
                }
                loop_pos.push((x, y, dx, dy));
            }

            if (x, y) == (*xo, *yo) {
                (x, y, dx, dy) = hit_obstacle(x, y, dx, dy);
                loop_check = true;
                continue;
            }
            match v[x as usize][y as usize] {
                '#' => (x, y, dx, dy) = hit_obstacle(x, y, dx, dy),
                '*' => v[x as usize][y as usize] = '+',
                _ => v[x as usize][y as usize] = '*',
            }
        }
    }

    println!("Guard loops in {} obstacles", sum);
    Ok(())
}
