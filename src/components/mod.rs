use std::{error, fmt};

pub mod animal;
pub mod sounds;

#[derive(Debug)]
pub struct ParseComponentError(String);

impl fmt::Display for ParseComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse {} as a component", self.0)
    }
}

impl error::Error for ParseComponentError {}
