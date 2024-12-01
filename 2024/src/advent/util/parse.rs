use std::io;

#[allow(dead_code)]
pub(crate) fn with_nom<F, T>(input: &str, parser: F) -> Result<T, Box<dyn std::error::Error>>
where
    F: Fn(&str) -> nom::IResult<&str, T>,
{
    let (remainder, v) = parser(input).map_err(|e| {
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
