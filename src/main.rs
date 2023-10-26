use std::io;
use std::env;
use slug::slugify;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1].is_empty() {
        println!("No argument given. Exiting...");
        return;
    }

    let transformation = &args[1];
    println!("Let's {} some input!", transformation);

    let mut raw_string = String::new();

    match io::stdin().read_line(&mut raw_string) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to read {}", e);
            return;
        }
    }

    let result = match transformation.as_str() {
        "lowercase" => raw_string.to_lowercase(),
        "uppercase" => raw_string.to_uppercase(),
        "no-spaces" => raw_string.replace(' ', "").replace('\t',""),
        "slugify"   => slugify(raw_string),
        _ => {
            "Unknown operation. Original string: ".to_owned() + &raw_string
        }
    };

    println!("Result:\n{}", result);
}
