use nom::character::complete::alpha1;
use nom::multi::separated_list0;
use nom::{bytes::complete::tag, IResult};
use util;

util::main![pt1, pt2];

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (remainder, v) = separated_list0(tag("\n"), alpha1)(input)?;
    Ok((remainder, v.iter().map(|e| e.chars().collect()).collect()))
}

fn search(grid: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize, rem: &[char]) -> u32 {
    if rem.len() == 0 {
        return 1;
    }
    let xr = (x as isize) + dx;
    let yr = (y as isize) + dy;
    if xr < 0 || xr >= grid.len() as isize {
        return 0;
    }
    if yr < 0 || yr >= grid[0].len() as isize {
        return 0;
    }
    if grid[xr as usize][yr as usize] != rem[0] {
        return 0;
    }
    return search(&grid, xr as usize, yr as usize, dx, dy, &rem[1..]);
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let occurences: u32 = v.iter().enumerate().map(|(x, ve)| {
        ve.iter().enumerate().filter(|(_, &e)| e == 'X').map(|(y, _)|{
            let mut int_sum = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    int_sum += search(&v, x, y, dx, dy, &['M','A','S']);
                }
            }
            int_sum
        }).sum::<u32>()
    }).sum();

    println!("XMAS Occurences: {}", occurences);

    Ok(())
}


pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Vec<char>> = util::parse::with_nom(&path, parse_grid)?;

    let occurences: u32 = v.iter().enumerate().map(|(x, ve)| {
        ve.iter().enumerate().filter(|(_, &e)| e == 'A').filter_map(|(y, _)|{
            let Some(xl) = x.checked_sub(1) else {
                return None;
            };
            let Some(yl) = y.checked_sub(1) else {
                return None;
            };
            let Some(xr) = x.checked_add(1) else {
                return None;
            };
            let Some(yr) = y.checked_add(1) else {
                return None;
            };
            if xr >= v.len() || yr >= v[xr].len() {
                return None;
            }
            let p1 = match (&v[xl][yl], &v[xr][yr]) {
                ('S', 'M') | ('M', 'S') => true,
                _ => false
            };
            let p2 = match (&v[xl][yr], &v[xr][yl]) {
                ('S', 'M') | ('M', 'S') => true,
                _ => false
            };

            if p1 && p2 {
             return Some(1)
            }
            None
        }).sum::<u32>()
    }).sum();

    println!("XMAS Occurences: {}", occurences);

    Ok(())
}
