use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to use");
    }

    let file = File::open(args.get(1).unwrap()).unwrap();

    let output_joltages: Vec<i32> = BufReader::new(file)
        .lines()
        .filter_map(|r| match r {
            Ok(line) => Some(line.parse().unwrap()),
            _ => None,
        })
        .collect();

    let (ones, twos, threes) = chain_adapters(&output_joltages);

    println!(
        "Part 1: ones: {}, twos: {}, threes: {} => {}",
        ones,
        twos,
        threes,
        ones * threes
    );

    println!(
        "Part 2: {} arrangements",
        count_arrangements(&output_joltages)
    );
}

fn chain_adapters(adapters: &Vec<i32>) -> (i32, i32, i32) {
    let target = adapters.iter().max().unwrap() + 3;

    let mut sorted_adapters = adapters.clone();
    sorted_adapters.sort();
    let mut a = sorted_adapters.iter();
    let mut current = 0;
    let mut counts = vec![0, 0, 0];

    while current < target - 3 {
        let next = a.next();
        match next {
            Some(value) => {
                if *value <= (current + 3) {
                    let delta = value - current - 1;
                    counts[delta as usize] += 1;
                    println!("{} (+{}) => {:?}", value, delta + 1, counts);
                    current = *value;
                } else {
                    return (0, 0, 0);
                }
            }
            None => {
                return (0, 0, 0);
            }
        }
    }
    let end_delta = target - current - 1;
    counts[end_delta as usize] += 1;
    (counts[0], counts[1], counts[2])
}

fn count_arrangements(adapters: &Vec<i32>) -> i64 {
    let target = (adapters.iter().max().unwrap() + 3) as usize;
    let mut sorted_adapters = adapters.clone();
    sorted_adapters.push(0);
    sorted_adapters.push(target as i32);
    sorted_adapters.sort();

    let mut counts = vec![0; target + 1];
    counts[0] = 1;

    for i in sorted_adapters.iter() {
        if *i == 0 {
            continue;
        }
        counts[*i as usize] = count_or_zero(&counts, i - 1)
            + count_or_zero(&counts, i - 2)
            + count_or_zero(&counts, i - 3);
    }

    // recursive attempt
    //visit_arrangements(&sorted_adapters, target, 0, 0)

    // counting approach
    // let mut total = 1;
    // let mut idx = sorted_adapters.len()-1;
    // let mut current = target;
    // while idx > 0 {
    //     current -= 3;
    //     println!("Counting options down to {}", current);
    //     let mut count = 0;
    //     while sorted_adapters[idx] >= current && idx > 0{
    //         count += 1;
    //         idx -= 1;
    //     }
    //     println!("Found {}", count);
    //     match count {
    //         1 => total *= 1,
    //         2 => total *= 2,
    //         3 => total *= 4,
    //         _ => panic!(":("),
    //     };
    //     println!("total: {}", total);
    //
    //     current = sorted_adapters[idx+1];
    // }

    counts[target as usize]
}

fn count_or_zero(counts: &Vec<i64>, idx: i32) -> i64 {
    if idx < 0 {
        0
    } else {
        *counts.get(idx as usize).unwrap()
    }
}

// fn visit_arrangements(sorted_adapters: &Vec<i32>, target: i32, last: i32, idx: usize) -> i32 {
//     // println!("target: {}, path: {:?}, idx: {}, adapters: {:?}", target, path, idx, sorted_adapters);
//
//     if last >= target-3 {
//         1
//     } else {
//         let mut current_idx = idx;
//         let mut total = 0;
//
//         while current_idx < sorted_adapters.len()
//             && sorted_adapters[current_idx] <= last + 3 {
//
//             total += visit_arrangements(
//                 sorted_adapters,
//                 target,
//                 sorted_adapters[current_idx],
//                 current_idx+1,
//             );
//             current_idx += 1
//         }
//         total
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{chain_adapters, count_arrangements};

    #[test]
    fn test_simple_setup() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let (ones, _twos, threes) = chain_adapters(&adapters);

        assert_eq!(7, ones);
        assert_eq!(5, threes);
    }

    #[test]
    fn test_longer_setup() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let (ones, _twos, threes) = chain_adapters(&adapters);

        assert_eq!(22, ones);
        assert_eq!(10, threes);
    }

    #[test]
    fn count_setups() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let count = count_arrangements(&adapters);
        assert_eq!(8, count);
    }

    #[test]
    fn count_setups2() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let count = count_arrangements(&adapters);
        assert_eq!(19208, count);
    }
}
