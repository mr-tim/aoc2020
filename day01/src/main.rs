use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify file to operate on");
    }
    let filename = args.get(1).unwrap();

    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();

    let mut values: Vec<i32> = Vec::new();
    for r in lines {
        if let Ok(line) = r {
            if let Ok(value) = line.parse() {
                values.push(value);
            }
        }
    }

    println!("Loaded {} expenses", values.len());

    println!("Part 1");
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            let a = values[i];
            let b = values[j];
            if a + b == 2020 {
                println!("{} + {} = 2020", a, b);
                println!("{} x {} = {}", a, b, a * b);
            }
        }
    }

    println!("Part 2");
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            for k in (j + 1)..values.len() {
                let a = values[i];
                let b = values[j];
                let c = values[k];

                if a + b + c == 2020 {
                    println!("{} + {} + {} = 2020", a, b, c);
                    println!("{} x {} x {} = {}", a, b, c, a * b * c);
                }
            }
        }
    }
}
