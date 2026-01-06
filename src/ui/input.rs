use crate::errors::AppError;
use std::io::{self, Read, Write};

pub fn read_line() -> Result<String, AppError> {
    print!("-> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub fn read_choice() -> Result<String, AppError> {
    read_line()
}
