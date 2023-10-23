pub mod csv;

use std::error::Error;
use slug::slugify;

pub fn lowercase_text(text: String) -> Result<String, Box<dyn Error>> {
    Result::Ok(text.to_lowercase())
}

pub fn uppercase_text(text: String) -> Result<String, Box<dyn Error>> {
    Result::Ok(text.to_uppercase())
}

pub fn remove_whitespaces(text: String) -> Result<String, Box<dyn Error>> {
    Result::Ok(text.chars().filter(|c| !c.is_whitespace()).collect())
}

pub fn slugify_text(text: String) -> Result<String, Box<dyn Error>> {
    Result::Ok(slugify(text))
}