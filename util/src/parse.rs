use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn with_nom<F, T>(filepath: &str, parser: F) -> Result<T, Box<dyn std::error::Error>>
where
    F: Fn(&str) -> nom::IResult<&str, T>,
{
    let input: String = std::fs::read_to_string(filepath)?.trim().parse()?;
    let (remainder, v) = parser(&input).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Error while parsing input: {}", e.to_string()),
        )
    })?;
    if !remainder.is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("There's a remainder after parsing input: {}", remainder),
        )));
    }
    Ok(v)
}

pub fn in_lines(
    path: &str,
) -> Result<Box<dyn Iterator<Item = String>>, Box<dyn std::error::Error>> {
    let input = File::open(path)?;
    let reader = io::BufReader::new(input);

    let iter = reader
        .lines()
        .filter_map(|l| Some(l.ok()?.trim_end().to_owned()));
    Ok(Box::new(iter))
}
