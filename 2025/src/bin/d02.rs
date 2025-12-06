use util;

util::main![pt1, pt2];

use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list0,
    sequence::separated_pair, IResult,
};

fn parse_lists(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (remainder, v) = separated_list0(tag(","), separated_pair(u64, tag("-"), u64))(input)?;
    Ok((remainder, v))
}

fn amt_digits(a: u64) -> u32 {
    // Find amount of digits
    let mut t = a;
    let mut digits = 0;
    while t != 0 {
        digits += 1;
        t /= 10;
    }
    digits
}

fn repeating_digits(a: u64, parts_amt: u32) -> bool {
    let tdigits = amt_digits(a);
    if tdigits % parts_amt != 0 {
        return false;
    }
    let part_size: u64 = (10 as u64).pow(tdigits / parts_amt);
    let modulo = a % part_size;
    let mut tst = modulo;
    for _ in 1..parts_amt {
        tst *= part_size;
        tst += modulo;
    }
    return tst == a;
}

fn repeating(a: u64) -> bool {
    let digs = amt_digits(a);
    for i in 2..=digs {
        if repeating_digits(a, i) {
            return true;
        }
    }
    return false;
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(u64, u64)> = util::parse::with_nom(&path, parse_lists)?;
    let mut invalid_sum = 0;
    for (s, e) in v {
        for i in s..=e {
            if repeating_digits(i, 2) {
                invalid_sum += i;
            }
        }
    }
    println!("Invalid Sum: {}", invalid_sum);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<(u64, u64)> = util::parse::with_nom(&path, parse_lists)?;
    let mut invalid_sum = 0;
    for (s, e) in v {
        for i in s..=e {
            if repeating(i) {
                invalid_sum += i;
            }
        }
    }
    println!("Invalid Sum: {}", invalid_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeating_digits() {
        // Repeating
        assert!(repeating_digits(123123, 2));
        assert!(repeating_digits(121212, 3));
        assert!(repeating_digits(111111, 3));
        assert!(repeating_digits(111111, 2));
        assert!(repeating_digits(123123, 2));

        // Not repeating
        assert!(!repeating_digits(123123, 3));
        assert!(!repeating_digits(123123, 4));
        assert!(!repeating_digits(111111, 4));
        assert!(!repeating_digits(1111111, 4));
    }

    #[test]
    fn test_repeating() {
        assert!(repeating(123123));
        assert!(repeating(112112));
        assert!(repeating(111));
        assert!(repeating(10101010));
        assert!(repeating(1112211122));
        assert!(!repeating(112));
        assert!(!repeating(131));
        assert!(!repeating(1112213122));
    }
}
