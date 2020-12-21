use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq)]
enum Cell {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            'L' => Cell::EmptySeat,
            '#' => Cell::OccupiedSeat,
            '.' => Cell::Floor,
            _ => panic!("Invalid char!"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::EmptySeat => 'L',
            Cell::OccupiedSeat => '#',
            Cell::Floor => '.',
        }
    }
}

struct Seating {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Seating {
    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn load_from_file(file: File) -> Seating {
        let mut cells: Vec<Cell> = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for r in BufReader::new(file).lines() {
            if let Ok(line) = r {
                height += 1;
                width = 0;
                for c in line.chars() {
                    width += 1;
                    cells.push(Cell::from_char(c));
                }
            }
        }

        Seating {
            cells,
            width,
            height,
        }
    }

    fn update(&mut self, updater: fn (x: usize, y: usize) -> Cell) -> bool {
        let mut updated = false;
        let mut updated_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                updated_cells.push({
                    let current_cell = &self.cells[self.index(x, y)];
                    let updated_cell = updater(x, y);
                    if *current_cell != updated_cell {
                        updated = true
                    }

                    updated_cell
                })
            }
        }
        self.cells = updated_cells;
        updated
    }

    fn update_neighbour_count(&self, x: usize, y: usize) -> Cell {
        match self.cells[self.index(x, y)] {
            Cell::OccupiedSeat => {
                if self.neighbour_count(x, y) >= 4 {
                    Cell::EmptySeat
                } else {
                    Cell::OccupiedSeat
                }
            }
            Cell::EmptySeat => {
                if self.neighbour_count(x, y) == 0 {
                    Cell::OccupiedSeat
                } else {
                    Cell::EmptySeat
                }
            }
            Cell::Floor => Cell::Floor,
        }
    }

    fn neighbour_count(&self, x: usize, y: usize) -> usize {
        let mut total = 0;

        for delta_col in [-1, 0, 1].iter().cloned() {
            for delta_row in [-1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let neighbour_row = y as i32 + delta_row;
                let neighbour_col = x as i32 + delta_col;

                let invalid_row = neighbour_row < 0
                    || neighbour_row >= self.height as i32
                    || neighbour_col < 0
                    || neighbour_col >= self.width as i32;

                if !invalid_row
                    && self.cells[self.index(neighbour_col as usize, neighbour_row as usize)]
                        == Cell::OccupiedSeat
                {
                    // if x == 1 && y == 1 {
                    //     println!("{}, {}", neighbour_col, neighbour_row);
                    // }
                    total += 1;
                }
            }
        }
        total
    }

    fn update_visible_neighbours(&self, x: usize, y: usize) -> Cell {
        self.cells[self.index(x, y)].clone()
    }

    fn display(&self) {
        let mut ss = Vec::new();
        for y in 0..self.height {
            let mut s = String::new();
            for x in 0..self.width {
                s.push(self.cells[self.index(x, y)].to_char());
            }
            ss.push(s);
        }

        for s in ss.iter() {
            println!("{}", s);
        }
    }

    fn display_neighbour_counts(&self) {
        let mut ss = Vec::new();
        for y in 0..self.height {
            let mut s = String::new();
            for x in 0..self.width {
                let idx = self.index(x, y);
                if self.cells[idx] == Cell::Floor {
                    s.push(' ');
                } else {
                    s.push(('0' as u8 + (self.neighbour_count(x, y) as u8)) as char);
                }
            }
            ss.push(s);
        }

        for s in ss.iter() {
            println!("{}", s);
        }
    }

    fn occupied_count(&self) -> usize {
        self.cells
            .iter()
            .cloned()
            .filter(|s| *s == Cell::OccupiedSeat)
            .count()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to load!")
    }

    let file = File::open(args.get(1).unwrap()).unwrap();

    let mut seating = Seating::load_from_file(file);

    let mut count = 0;

    while seating.update(|x, y| seating.update_neighbour_count(x, y)) {
        count += 1;
    }

    println!("Completed after {} steps", count);
    println!("{} occupied seats", seating.occupied_count());
}

#[cfg(test)]
mod tests {
    use crate::Seating;
    use std::fs::File;

    #[test]
    fn check_sample_stability() {
        let f = File::open("sample.txt").unwrap();
        let mut seating = Seating::load_from_file(f);

        println!("{} x {}", seating.width, seating.height);

        let mut updated = true;
        let mut count = 0;

        while updated {
            println!("Step {}", count);
            seating.display();
            println!();
            // seating.display_neighbour_counts();
            updated = seating.update(|x, y| seating.update_neighbour_count(x, y)) && count < 100;
            count += 1;
            // println!();
        }

        count -= 1;

        assert_eq!(5, count);
        assert_eq!(37, seating.occupied_count());
    }
}
