use std::error::Error;
use std::io::{self, ErrorKind};

use crate::model::transform_mode::TransformMode;
use crate::transformations::functions;

#[derive(Clone)]
pub struct Command {
    pub command: TransformMode,
    pub input: String,
}

impl Command {
    pub fn new(command: TransformMode, input: impl Into<String>) -> Command {
        Command {
            command,
            input: input.into(),
        }
    }

    pub const STOP: Command = Command {
        command: TransformMode::Stop,
        input: String::new(),
    };

    pub fn execute(&self) -> Result<String, Box<dyn Error>> {

        if matches!(self.command, TransformMode::Stop | TransformMode::CsvParse) {
            return Err(Box::new(io::Error::new(ErrorKind::Other, 
                         format!("Tried to execute{} command. Not executable. ", self.command.to_str()))));
        }

        let transformation: fn(String) -> Result<String, Box<dyn Error>> = 
        match self.command {
            TransformMode::Lowercase => functions::lowercase_text,
            TransformMode::Uppercase => functions::uppercase_text,
            TransformMode::NoSpaces => functions::remove_whitespaces,
            TransformMode::Slugify => functions::slugify_text,
            _ => unreachable!(),
        };

        transformation(self.input.clone())
    }

    pub fn is_stop(&self) -> bool {
        self.command == TransformMode::Stop
    }
}