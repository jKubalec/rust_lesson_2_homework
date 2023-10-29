#[derive(Clone)]
pub enum TransformMode {
    CsvParse,
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    RealTime,
    Stop,   //  specific command for stopping the transmission
}

impl TransformMode {
    pub fn from_str(input: &str) -> Option<TransformMode> {
        match input {
            "csv" => Some(TransformMode::CsvParse),
            "lowercase" => Some(TransformMode::Lowercase),
            "uppercase" => Some(TransformMode::Uppercase),
            "no-spaces" => Some(TransformMode::NoSpaces),
            "slugify" => Some(TransformMode::Slugify),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TransformMode::CsvParse => "CsvParse",
            TransformMode::Lowercase => "Lowercase",
            TransformMode::Uppercase => "Uppercase",
            TransformMode::NoSpaces => "NoSpaces",
            TransformMode::Slugify => "Slugify",
            TransformMode::RealTime => "RealTime",
            TransformMode::Stop => "Stop",
        }
    }
}

impl PartialEq for TransformMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TransformMode::CsvParse, TransformMode::CsvParse) => true,
            (TransformMode::Lowercase, TransformMode::Lowercase) => true,
            (TransformMode::Uppercase, TransformMode::Uppercase) => true,
            (TransformMode::NoSpaces, TransformMode::NoSpaces) => true,
            (TransformMode::Slugify, TransformMode::Slugify) => true,
            (TransformMode::RealTime, TransformMode::RealTime) => true,
            (TransformMode::Stop, TransformMode::Stop) => true,
            _ => false,
        }
    }
}