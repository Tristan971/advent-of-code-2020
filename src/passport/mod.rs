mod eyecolor;
mod height;

pub mod passport {
    use std::collections::HashMap;

    use crate::passport::eyecolor::EyeColor;
    use crate::passport::height::{to_height, Height};

    pub struct Passport {
        pub birth_year: i32,
        pub issue_year: i32,
        pub expir_year: i32,
        pub height: Height,
        pub hair_color: String,
        pub eye_color: EyeColor,
        pub passport_id: String,
        pub country_id: Option<String>,
    }

    pub fn parse_passport(input: &str) -> Passport {
        let field_map = input
            .split_whitespace()
            .map(|fieldspec| fieldspec.split(":").collect::<Vec<&str>>())
            .filter(|parts| parts.len() == 2)
            .fold(HashMap::new(), |mut fields, parts| {
                let key = parts[0];
                let value = parts[1];

                fields.insert(key, value);
                return fields;
            });

        return Passport {
            birth_year: field_map.get("byr").unwrap().parse::<i32>().unwrap(),
            issue_year: field_map.get("iyr").unwrap().parse::<i32>().unwrap(),
            expir_year: field_map.get("eyr").unwrap().parse::<i32>().unwrap(),
            height: field_map.get("hgt").map(|&field| to_height(field)).unwrap(),
            hair_color: field_map
                .get("hcl")
                .map(|&field| String::from(field))
                .unwrap(),
            eye_color: field_map
                .get("ecl")
                .map(|&field| EyeColor::from_code(field).unwrap())
                .unwrap(),
            passport_id: field_map
                .get("pid")
                .map(|&field| String::from(field))
                .unwrap(),
            country_id: field_map.get("cid").map(|&field| String::from(field)),
        };
    }
}
