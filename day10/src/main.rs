use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to use");
    }

    let file = File::open(args.get(1).unwrap()).unwrap();

    let output_joltages: Vec<i32> = BufReader::new(file).lines().filter_map(|r| {
        match r {
            Ok(line) => {
                Some(line.parse().unwrap())
            },
            _ => None,
        }
    }).collect();

    let (ones, twos, threes) = chain_adapters(&output_joltages);

    println!("Part 1: ones: {}, twos: {}, threes: {} => {}", ones, twos, threes, ones * threes);
}

fn chain_adapters(adapters: &Vec<i32>) -> (i32, i32, i32) {
    let target = adapters.iter().max().unwrap() + 3;

    let mut sorted_adapters = adapters.clone();
    sorted_adapters.sort();
    let mut a = sorted_adapters.iter();
    let mut current = 0;
    let mut counts = vec![0, 0, 0];

    while current < target-3 {
        let next = a.next();
        match next {
            Some(value) => {
                if *value <= (current + 3) {
                    let delta = value-current-1;
                    counts[delta as usize] += 1;
                    println!("{} (+{}) => {:?}", value, delta+1, counts);
                    current = *value;
                } else {
                    return (0, 0, 0);
                }
            },
            None => {
                return (0, 0, 0);
            }
        }
    }
    let end_delta = target-current-1;
    counts[end_delta as usize] += 1;
    (counts[0], counts[1], counts[2])
}

#[cfg(test)]
mod tests {
    use crate::chain_adapters;

    #[test]
    fn test_simple_setup() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let (ones, _twos, threes) = chain_adapters(&adapters);

        assert_eq!(7, ones);
        assert_eq!(5, threes);
    }

    #[test]
    fn test_longer_setup() {
        let adapters = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        let (ones, _twos, threes) = chain_adapters(&adapters);

        assert_eq!(22, ones);
        assert_eq!(10, threes);
    }
}