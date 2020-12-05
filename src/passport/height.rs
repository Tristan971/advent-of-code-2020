use regex::Regex;

use crate::passport::height::LengthUnit::{Centimeters, Inches};

lazy_static! {
    static ref HGT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
}

pub enum LengthUnit {
    Inches,
    Centimeters,
}

pub struct Height {
    len: i32,
    unit: LengthUnit,
}

pub fn to_height(spec: &str) -> Height {
    let captures = HGT.captures(spec).unwrap();
    return Height {
        len: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        unit: match captures.get(2).unwrap().as_str() {
            "in" => Some(Inches),
            "cm" => Some(Centimeters),
            _ => None,
        }
        .unwrap(),
    };
}
