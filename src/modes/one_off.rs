use std::io;

use crate::model::command::Command;
use crate::model::transform_mode::TransformMode;

use crate::transformations::csv::Csv;
use crate::transformations::csv::CsvError;
use crate::transformations::csv::CsvError::*;

fn print_csv() -> () {
    println!("Let's read some CSV! Type it in, at the end just write empty line");
        let mut input = String::new();

        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let trimmed = line.trim();
            if trimmed.is_empty() {
                break;
            }
            input.push_str(trimmed);
            input.push('\n');
        }

        let my_csv: Result<Csv, CsvError> = Csv::parse_text(input);

        match my_csv {
            Ok(csv) => println!("{}", csv),
            Err(e) => match e {
                EmptyString => 
                    eprintln!("Empty string on input. Doing nothing."),
                WrongLine(wrong_line) =>
                    eprintln!("Line of wrong length on the input. Exiting:\n{}", wrong_line),
            }
        }
}

pub fn one_off_processing(mode : TransformMode) -> () {
    match mode {
        TransformMode::CsvParse => print_csv(),
        _ => {
            println!("Let's {} some input! Type it in:", mode.to_str());
            let mut raw_string = String::new();

            match io::stdin().read_line(&mut raw_string) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to read {}", e);
                    return;
                }
            };
            
            let transformCommand = Command{command: mode, input: raw_string};
            let result = transformCommand.execute();
            // transformation(raw_string);
            
            match result {
                Ok(result_text) => {println!("Result:\n{:?}", result_text)}
                Err(e) => {eprintln!("Error processing input text: {}", e)}
            }
        }
    };
}
