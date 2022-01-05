use std::{convert::TryFrom, error::Error, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    UP,
    DOWN,
    FORWARD,
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "up" => Ok(Operation::UP),
            "down" => Ok(Operation::DOWN),
            "forward" => Ok(Operation::FORWARD),
            _ => Err("Bad input for Operation".into()),
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Operation::from_str(value)
    }
}
