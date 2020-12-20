#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum FieldType {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl FieldType {
    fn validate(&self, value: &str) -> bool {
        lazy_static! {
            static ref HAIR_RE: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
            static ref PID_RE: Regex = Regex::new("[0-9]{9}").unwrap();
        }
        match self {
            FieldType::Byr => FieldType::int_in_range(value, 1920, 2002),
            FieldType::Iyr => FieldType::int_in_range(value, 2010, 2020),
            FieldType::Eyr => FieldType::int_in_range(value, 2020, 2030),
            FieldType::Hgt => {
                let (height_value, height_unit) = value.split_at(value.len() - 2);
                match height_unit {
                    "cm" => FieldType::int_in_range(height_value, 150, 193),
                    "in" => FieldType::int_in_range(height_value, 59, 76),
                    _ => false,
                }
            }
            FieldType::Hcl => HAIR_RE.is_match(value),
            FieldType::Ecl => match value {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            },
            FieldType::Pid => PID_RE.is_match(value),
            FieldType::Cid => true,
        }
    }

    fn int_in_range(value: &str, min: i32, max: i32) -> bool {
        match value.parse::<i32>() {
            Ok(int_value) => int_value >= min && int_value <= max,
            Err(_) => false,
        }
    }
}

impl FromStr for FieldType {
    type Err = ();

    fn from_str(input: &str) -> Result<FieldType, Self::Err> {
        match input {
            "byr" => Ok(FieldType::Byr),
            "iyr" => Ok(FieldType::Iyr),
            "eyr" => Ok(FieldType::Eyr),
            "hgt" => Ok(FieldType::Hgt),
            "hcl" => Ok(FieldType::Hcl),
            "ecl" => Ok(FieldType::Ecl),
            "pid" => Ok(FieldType::Pid),
            "cid" => Ok(FieldType::Cid),
            _ => Err(()),
        }
    }
}

struct Passport {
    fields: HashMap<FieldType, String>,
}

impl Passport {
    fn add_field(&mut self, field_type: FieldType, value: String) {
        self.fields.insert(field_type, value);
    }

    fn is_valid(&self) -> bool {
        let required_fields_specified = self.fields.len() == 8
            || (self.fields.len() == 7 && !self.fields.contains_key(&FieldType::Cid));

        let mut fields_valid = true;

        for (field_type, value) in self.fields.iter() {
            println!("Validating value {} for field {:?}", value, field_type);
            fields_valid &= field_type.validate(value.as_str());
        }

        required_fields_specified && fields_valid
    }

    fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify password file");
    }
    let filename = args.get(1).unwrap();

    let file = File::open(filename).unwrap();

    let mut valid_count = 0;
    let mut current_passport = Passport::new();

    for r in BufReader::new(file).lines() {
        if let Ok(line) = r {
            if line.len() == 0 {
                if current_passport.is_valid() {
                    valid_count += 1;
                }
                current_passport = Passport::new();
            } else {
                let kvs: Vec<&str> = line.split(" ").collect();
                for kv in kvs {
                    let kv_split: Vec<&str> = kv.split(":").collect();
                    let field_type: FieldType = kv_split[0].parse().unwrap();
                    let value = kv_split[1];
                    current_passport.add_field(field_type, String::from(value));
                }
            }
        } else {
            println!("Not ok line! {:?}", r);
        }
    }

    // part 2 doesn't want this?
    // if current_passport.is_valid() {
    //     valid_count += 1
    // }

    println!("Found {} valid passports", valid_count);
}

#[cfg(test)]
mod tests {
    use crate::{FieldType, Passport};

    #[test]
    fn test_is_valid() {
        // all 8
        let mut all8_passport = Passport::new();
        all8_passport.add_field(FieldType::Byr, "2002".to_string());
        all8_passport.add_field(FieldType::Iyr, "2012".to_string());
        all8_passport.add_field(FieldType::Eyr, "2020".to_string());
        all8_passport.add_field(FieldType::Hgt, "165cm".to_string());
        all8_passport.add_field(FieldType::Hcl, "#123abc".to_string());
        all8_passport.add_field(FieldType::Ecl, "brn".to_string());
        all8_passport.add_field(FieldType::Pid, "000000001".to_string());
        all8_passport.add_field(FieldType::Cid, "somewhere".to_string());
        assert!(all8_passport.is_valid());

        // all reqd fields (missing cid)
        let mut all_reqd_passport = Passport::new();
        all_reqd_passport.add_field(FieldType::Byr, "2002".to_string());
        all_reqd_passport.add_field(FieldType::Iyr, "2012".to_string());
        all_reqd_passport.add_field(FieldType::Eyr, "2020".to_string());
        all_reqd_passport.add_field(FieldType::Hgt, "165cm".to_string());
        all_reqd_passport.add_field(FieldType::Hcl, "#123abc".to_string());
        all_reqd_passport.add_field(FieldType::Ecl, "brn".to_string());
        all_reqd_passport.add_field(FieldType::Pid, "000000001".to_string());
        assert!(all_reqd_passport.is_valid());
    }

    #[test]
    fn test_validation() {
        assert!(FieldType::Byr.validate("2002"));
        assert!(!FieldType::Byr.validate("2003"));
        assert!(FieldType::Hgt.validate("60in"));
        assert!(FieldType::Hgt.validate("190cm"));
        assert!(!FieldType::Hgt.validate("190in"));
        assert!(!FieldType::Hgt.validate("190"));
        assert!(FieldType::Hcl.validate("#123abc"));
        assert!(!FieldType::Hcl.validate("#123abz"));
        assert!(!FieldType::Hcl.validate("123abc"));
        assert!(FieldType::Ecl.validate("brn"));
        assert!(!FieldType::Ecl.validate("wat"));
        assert!(FieldType::Pid.validate("000000001"));
        assert!(FieldType::Pid.validate("0123456789"));
    }
}
