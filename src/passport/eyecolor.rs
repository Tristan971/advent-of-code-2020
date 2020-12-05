use crate::passport::eyecolor::EyeColor::{Amber, Blue, Brown, Gray, Green, Hazelnut, Other};

pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazelnut,
    Other,
}

impl EyeColor {
    pub fn from_code(code: &str) -> Option<EyeColor> {
        match code {
            "amb" => Some(Amber),
            "blu" => Some(Blue),
            "brn" => Some(Brown),
            "gry" => Some(Gray),
            "grn" => Some(Green),
            "hzl" => Some(Hazelnut),
            "oth" => Some(Other),
            _ => None,
        }
    }
}
