use std::env;
use std::fs::File;
use std::io::SeekFrom::Start;
use std::io::{BufRead, BufReader, Lines, Seek};

struct PasswordPolicy {
    param1: u8,
    param2: u8,
    character: char,
}

impl PasswordPolicy {
    fn is_valid(&self, value: &str) -> bool {
        let occurrences = value.chars().filter(|&c| self.character == c).count() as u8;
        occurrences >= self.param1 && occurrences <= self.param2
    }

    fn is_valid2(&self, value: &str) -> bool {
        let cs: Vec<char> = value.chars().collect();
        let mut occurences = 0;
        if cs[(self.param1 - 1) as usize] == self.character {
            occurences += 1;
        }
        if cs[(self.param2 - 1) as usize] == self.character {
            occurences += 1;
        }
        return occurences == 1;
    }

    fn parse(policy: &str) -> PasswordPolicy {
        let sp: Vec<&str> = policy.split(|c| c == '-' || c == ' ').collect();
        return PasswordPolicy {
            param1: sp[0].parse().unwrap(),
            param2: sp[1].parse().unwrap(),
            character: sp[2].chars().nth(0).unwrap(),
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify password file");
    }
    let filename = args.get(1).unwrap();

    let mut file = File::open(filename).unwrap();

    let valid_count1 = get_valid_count(BufReader::new(&file).lines(), PasswordPolicy::is_valid);
    println!("Part 1: {} passwords were valid", valid_count1);

    file.seek(Start(0)).unwrap();

    let valid_count2 = get_valid_count(BufReader::new(&file).lines(), PasswordPolicy::is_valid2);
    println!("Part 2: {} passwords were valid", valid_count2);
}

type PasswordChecker = fn(&PasswordPolicy, &str) -> bool;

fn get_valid_count(lines: Lines<BufReader<&File>>, checker: PasswordChecker) -> usize {
    let valid_count = lines
        .filter(|r| {
            if let Ok(line) = r {
                let parts: Vec<&str> = line.split(": ").collect();
                let policy = PasswordPolicy::parse(parts[0]);
                checker(&policy, parts[1])
            } else {
                false
            }
        })
        .count();
    valid_count
}

#[cfg(test)]
mod tests {
    use crate::PasswordPolicy;

    #[test]
    fn test_parse() {
        let p = PasswordPolicy::parse("1-3 a");
        assert_eq!(1, p.param1);
        assert_eq!(3, p.param2);
        assert_eq!('a', p.character);
    }

    #[test]
    fn test_is_valid() {
        let p = PasswordPolicy::parse("1-3 a");
        assert!(p.is_valid("alphabet"));
        assert!(!p.is_valid("baaaad"));
        assert!(!p.is_valid("dog"));
    }

    #[test]
    fn test_is_valid2() {
        let p = PasswordPolicy::parse("1-3 a");
        assert!(p.is_valid2("abcde"));
        assert!(!p.is_valid2("abade"));
        assert!(p.is_valid2("cbade"));
        assert!(!p.is_valid2("bbbbb"));
    }
}
