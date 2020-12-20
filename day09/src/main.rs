use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("please specify a file");
    }

    let file = File::open(args.get(1).unwrap()).unwrap();

    let preamble_size: usize = args.get(2).unwrap().parse().unwrap();

    let values: Vec<i64> = BufReader::new(file)
        .lines()
        .filter_map(|r| match r {
            Ok(line) => Some(line.parse().unwrap()),
            _ => None,
        })
        .collect();

    let invalid = find_invalid_value(preamble_size, values.clone());
    if invalid <= 0 {
        panic!("Could not find invalid value!");
    } else {
        println!("Found invalid value: {}", invalid);
    }

    let range = find_invalid_range(&values, invalid);
    match range {
        Some((start, end)) => {
            let s = &values[start..end];
            let min = s.iter().min().unwrap();
            let max = s.iter().max().unwrap();
            println!("Part 2: Found {} + {} => {}", min, max, min + max);
        }
        None => panic!("Failed to find range for invalid values!"),
    };
}

fn find_invalid_range(values: &Vec<i64>, invalid: i64) -> Option<(usize, usize)> {
    for start in 0..values.len() - 2 {
        let mut length = 0;
        let mut total = 0;
        while start + length < values.len() {
            total = total + &values[start + length];
            length += 1;
            if length >= 2 {
                if total == invalid {
                    println!(
                        "Found run of invalids from {} with length {}",
                        start, length
                    );
                    return Some((start, start + length));
                }
            }
        }
    }
    return None;
}

fn find_invalid_value(preamble_size: usize, values: Vec<i64>) -> i64 {
    let mut buffer: Vec<i64> = Vec::new();

    for value in values {
        if buffer.len() >= preamble_size {
            let mut found = false;
            for i in 0..preamble_size {
                for j in (i + 1)..preamble_size {
                    if buffer[i] + buffer[j] == value {
                        found = true;
                    }
                }
            }
            if !found {
                return value;
            }
        }

        buffer.push(value);
        while buffer.len() > preamble_size {
            buffer.remove(0);
        }
    }

    return 0;
}
