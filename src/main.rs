use std::io;
use std::env;
use std::error::Error;

mod transformations;
use transformations::csv::Csv;
use transformations::csv::CsvError;
use transformations::csv::CsvError::*;


const PARSE_CSV: &'static str = "CSV"; 

fn main() {
    let args: Vec<String> = env::args().collect();

    let transformation_input = args.get(1);

    let transformation_name = match transformation_input {
        Some(s) if s.as_str() == "csv" => PARSE_CSV,
        Some(s) => s,
        None => {
            eprintln!("No argument given. Exiting...");
            return;
        },
    };

    if transformation_name == PARSE_CSV {
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
                EmptyString => eprintln!("Empty string on input. Doing nothing."),
                WrongLine(wrong_line) => eprintln!("Line of wrong length on the input. Exiting:\n{}", wrong_line)
            }
        }

    } else {
        let transformation:  fn(String) -> Result<String, Box<dyn Error>> = 
            match transformation_name {
                "lowercase" => transformations::lowercase_text,
                "uppercase" => transformations::uppercase_text,
                "no-spaces" => transformations::remove_whitespaces,
                "slugify" => transformations::slugify_text,
                unkown_op => {
                    eprintln!("Unknown operation {} entered. Exiting...", unkown_op);
                    return;
                }
            };

        println!("Let's {} some input! Type it in:", transformation_name);

        let mut raw_string = String::new();

        match io::stdin().read_line(&mut raw_string) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read {}", e);
                return;
            }
        }

        let result = transformation(raw_string);
        
        match result {
            Ok(result_text) => {println!("Result:\n{:?}", result_text)}
            Err(e) => {eprintln!("Error processing input text: {}", e)}
        }
    }
}
