use std::{fmt, collections::HashMap, ops::Deref};

pub enum CsvError {
    EmptyString,
    WrongLine(String)
}
pub struct Csv {
    header: Vec<String>,
    content: Vec<Vec<String>>,
}

impl Csv {
    pub fn parse_text(csv_text: String) -> Result<Self, CsvError> {
        let mut lines = csv_text.lines();
        let header_line = match lines.next() {
            Some(line) => line.to_string(),
            None => {
                return Err(CsvError::EmptyString);
            }
        };

        let header: Vec<String> = header_line.split(",").map(String::from).collect();
        let header_length = header.len();

        let content_res: Result<Vec<Vec<String>>, CsvError> = lines.map(|line| {
            let line_content: Vec<String> = line.split(",").map(String::from).collect();
            if line_content.len() == header_length {
                Ok(line_content)
            } else {
                Err(CsvError::WrongLine(line.to_string()))
            }
        }).collect();

        match content_res {
            Ok(content) => Ok(Csv {
                                header: header,
                                content: content,
                            }),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header_indexed: Vec<_> = self.header.iter().enumerate()
                                                                     .map(|(index, col_name)| (index, col_name.deref()))
                                                                    .collect();
        let header_lenghts: HashMap<usize, usize> = self.header.iter().enumerate().map(|(i, col_name)| (i, col_name.len() + 6)).collect();
        let line_length = header_lenghts.iter().map(|x| x.1).fold(0, |total, current| total + current + 1) + 1;
        writeln!(f,"{}", "-".repeat(line_length))?;
        write!(f, "|")?;
        for header in header_indexed {
            write!(f, "   {}   |", header.1)?;
        }
        writeln!(f,"")?;
        writeln!(f,"{}", "=".repeat(line_length))?;
        for line in &self.content {
            write!(f, "|")?;
            for (index, value) in line.iter().enumerate() {
                write!(f, "{}|", pad_string(value, *header_lenghts.get(&index).or(Some(&0)).unwrap()))?;
            }
            writeln!(f,"")?;
        }
        writeln!(f,"{}", "-".repeat(line_length))?;
        Ok(())
    }
}

fn pad_string(s: &str, length: usize) -> String {
    if s.len() < length {
        format!(" {}{}",s.to_string(), " ".repeat(length - s.len() - 1))
    } else {
        format!("{}...", &s[..length - 3])
    }
}