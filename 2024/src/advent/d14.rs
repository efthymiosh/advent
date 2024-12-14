use std::collections::HashSet;

use crate::advent::util;
use nom::character::complete::i64;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::{bytes::complete::tag, IResult};

#[derive(Debug)]
struct Robot {
    init: (i64, i64),
    v: (i64, i64),
}
const GRID_X: i64 = 101;
const GRID_Y: i64 = 103;

enum Quadrant {
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
    FOURTH = 4,
}

impl Robot {
    fn pos_after_seconds(&self, sec: i64) -> (i64, i64) {
        (
            (((self.init.0 + self.v.0 * sec) % GRID_X) + GRID_X) % GRID_X,
            (((self.init.1 + self.v.1 * sec) % GRID_Y) + GRID_Y) % GRID_Y,
        )
    }

    fn quadrant_after_seconds(&self, sec: i64) -> Option<Quadrant> {
        match self.pos_after_seconds(sec) {
            (x, y) if x < GRID_X / 2 && y < GRID_Y / 2 => Some(Quadrant::FIRST),
            (x, y) if x > GRID_X / 2 && y < GRID_Y / 2 => Some(Quadrant::SECOND),
            (x, y) if x < GRID_X / 2 && y > GRID_Y / 2 => Some(Quadrant::THIRD),
            (x, y) if x > GRID_X / 2 && y > GRID_Y / 2 => Some(Quadrant::FOURTH),
            _ => None,
        }
    }
}

fn parse_machines(input: &str) -> IResult<&str, Vec<Robot>> {
    let (remainder, v): (&str, Vec<((i64, i64), (i64, i64))>) = separated_list0(
        tag("\n"),
        separated_pair(
            preceded(tag("p="), separated_pair(i64, tag(","), i64)),
            tag(" "),
            preceded(tag("v="), separated_pair(i64, tag(","), i64)),
        ),
    )(input)?;
    Ok((
        remainder,
        v.iter()
            .map(|(init, v)| Robot { init: *init, v: *v })
            .collect(),
    ))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Robot> = util::parse::with_nom(&path, parse_machines)?;

    let c = v
        .iter()
        .filter_map(|robot| robot.quadrant_after_seconds(100))
        .fold((0, 0, 0, 0), |(a, b, c, d), q| match q {
            Quadrant::FIRST => (a + 1, b, c, d),
            Quadrant::SECOND => (a, b + 1, c, d),
            Quadrant::THIRD => (a, b, c + 1, d),
            Quadrant::FOURTH => (a, b, c, d + 1),
        });

    println!("Safety Factor: {}", c.0 * c.1 * c.2 * c.3);

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<Robot> = util::parse::with_nom(&path, parse_machines)?;

    let mut sec = 0;
    // our heuristic: at least half the bots must be mirrored on the X axis
    loop {
        sec += 1;
        // count amount of robots mirroring each other on the X axis
        let hspos: HashSet<(i64, i64)> =
            v.iter().map(|robot| robot.pos_after_seconds(sec)).collect();
        let mirror_count = hspos
            .iter()
            .filter_map(|(x, y)| hspos.get(&((GRID_X - *x), *y)))
            .count();

        // skip if less than half the bots are mirrored
        if mirror_count < v.len() / 2 {
            continue;
        }

        for y in 0..GRID_Y {
            for x in 0..GRID_X {
                if hspos.contains(&(x as i64, y as i64)) {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("After {} seconds", sec);
        break;
    }
    Ok(())
}
