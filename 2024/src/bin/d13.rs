use nom::character::complete::i64;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::{bytes::complete::tag, IResult};
use util;

util::main![pt1, pt2];

#[derive(Debug)]
struct ClawMachine {
    prize: (i64, i64),
    a: (i64, i64),
    b: (i64, i64),
}

const COST_A: i64 = 3;
const COST_B: i64 = 1;

impl ClawMachine {
    fn min_spend_win_pt2(&mut self) -> Option<i64> {
        self.prize.0 += 10000000000000;
        self.prize.1 += 10000000000000;
        self.min_spend_win()
    }

    fn min_spend_win(&self) -> Option<i64> {
        let a = self.a;
        let b = self.b;
        let prize = self.prize;
        /*
         * ap = presses of A
         * bp = presses of B
         *
         * a.0 * ap + b.0 * bp = prize.0
         * a.1 * ap + b.1 * bp = prize.1
         * // multiply first b.1, second by b.0
         * b.1 * a.0 * ap + b.0 * b.1 * bp = b.1 * prize.0
         * b.0 * a.1 * ap + b.0 * b.1 * bp = b.0 * prize.1
         * // solve for b.0 * b.1 * bp
         * b.0 * b.1 * bp = b.1 * prize.0 - b.1 * a.0 * ap
         * b.0 * b.1 * bp = b.0 * prize.1 - b.0 * a.1 * ap
         * =>
         * b.1 * prize.0 - b.1 * a.0 * ap = b.0 * prize.1 - b.0 * a.1 * ap
         * =>
         */
        let (ap, 0) = util::math::div_rem(b.0 * prize.1 - b.1 * prize.0, b.0 * a.1 - b.1 * a.0)
        else {
            return None;
        };

        let (bp, 0) = util::math::div_rem(prize.0 - a.0 * ap, b.0) else {
            return None;
        };

        if (bp, 0) == util::math::div_rem(prize.1 - a.1 * ap, b.1) {
            return Some(ap * COST_A + bp * COST_B);
        } else {
            return None;
        }
    }
}

fn parse_machines(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    let (remainder, v): (&str, Vec<((i64, i64), (i64, i64), (i64, i64))>) = separated_list0(
        tag("\n\n"),
        tuple((
            pair(
                preceded(tag("Button A: X+"), i64),
                delimited(tag(", Y+"), i64, tag("\n")),
            ),
            pair(
                preceded(tag("Button B: X+"), i64),
                delimited(tag(", Y+"), i64, tag("\n")),
            ),
            pair(preceded(tag("Prize: X="), i64), preceded(tag(", Y="), i64)),
        )),
    )(input)?;
    Ok((
        remainder,
        v.iter()
            .map(|(a, b, p)| ClawMachine {
                prize: *p,
                a: *a,
                b: *b,
            })
            .collect(),
    ))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<ClawMachine> = util::parse::with_nom(&path, parse_machines)?;

    let sum = v.iter().filter_map(|cm| cm.min_spend_win()).sum::<i64>();

    println!("minimum spend: {}", sum);

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut v: Vec<ClawMachine> = util::parse::with_nom(&path, parse_machines)?;

    let sum = v
        .iter_mut()
        .filter_map(|cm| cm.min_spend_win_pt2())
        .sum::<i64>();

    println!("minimum spend: {}", sum);

    Ok(())
}
