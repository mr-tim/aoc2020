use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Seat {
    row: u16,
    column: u16,
}

impl Seat {
    fn seat_id(&self) -> u16 {
        self.row * 8 + self.column
    }

    fn parse(seat_specifier: &str) -> Seat {
        let mut row_min = 0;
        let mut row_max = 128;
        let mut col_min = 0;
        let mut col_max = 8;

        let chars: Vec<char> = seat_specifier.chars().collect();

        for idx in 0..7 {
            let mid_point = row_min + (row_max - row_min) / 2;

            match chars[idx] {
                'F' => row_max = mid_point,
                'B' => row_min = mid_point,
                _ => panic!("invalid seat spec!"),
            }
            // println!("row: {}-{}", row_min, row_max);
        }

        for idx in 7..10 {
            let mid_point = col_min + (col_max - col_min) / 2;
            match chars[idx] {
                'L' => col_max = mid_point,
                'R' => col_min = mid_point,
                _ => panic!("invalid seat spec!"),
            }
            // println!("col: {}-{}", col_min, col_max);
        }

        Seat {
            row: row_min,
            column: col_min,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Specify file");
    }
    let file = File::open(args.get(1).unwrap()).unwrap();
    let mut max_id = 0;
    let mut seen_seats = vec![false; 8 * 128];

    for r in BufReader::new(file).lines() {
        if let Ok(line) = r {
            let seat = Seat::parse(line.as_str());
            if seat.seat_id() > max_id {
                max_id = seat.seat_id();
            }
            seen_seats[seat.seat_id() as usize] = true;
        }
    }

    println!("Max seat id: {}", max_id);

    for i in 8..(8 * 127) {
        if !seen_seats[i] && seen_seats[i - 1] && seen_seats[i + 1] {
            println!("Missing seat id is: {}", i);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Seat;

    #[test]
    fn test_parsing() {
        let seat1 = Seat::parse("BFFFBBFRRR");
        assert_eq!(70, seat1.row);
        assert_eq!(7, seat1.column);
        assert_eq!(567, seat1.seat_id());

        let seat2 = Seat::parse("FFFBBBFRRR");
        assert_eq!(14, seat2.row);
        assert_eq!(7, seat2.column);
        assert_eq!(119, seat2.seat_id());

        let seat3 = Seat::parse("BBFFBBFRLL");
        assert_eq!(102, seat3.row);
        assert_eq!(4, seat3.column);
        assert_eq!(820, seat3.seat_id());
    }
}
