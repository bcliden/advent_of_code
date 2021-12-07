use std::{convert::TryFrom, error::Error, str::FromStr};
use crate::operation::Operation;

/*
    TODO: This totally should have just been an enum
        with values inside it.

    Command:
        Forward(u32)
        Up(u32)
        Down(u32)

    But i guess it's good that I made my own using a tuple and an enum?
*/

#[derive(Debug)]
pub struct Command(pub Operation, pub u32);

impl TryFrom<&str> for Command {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Command::from_str(input)
    }
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // this all seems very clumsy
        let split_string = input.split(' ').collect::<Vec<_>>();
        match split_string.as_slice() {
            [op, n] if Operation::from_str(op).is_ok() && n.parse::<u32>().is_ok() => {
                Ok(Command(Operation::from_str(op)?, n.parse()?))
            }
            _ => Err("Malformed command".into()),
        }
    }
}
