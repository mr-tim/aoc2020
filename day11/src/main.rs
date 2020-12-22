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

#[derive(Clone)]
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

    fn update(&mut self, updater: fn(&Seating, usize, usize) -> Cell) -> bool {
        let mut updated = false;
        let mut updated_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                updated_cells.push({
                    let current_cell = &self.cells[self.index(x, y)];
                    let updated_cell = updater(self, x, y);
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

    fn neighbour_count(&self, x: usize, y: usize) -> usize {
        let mut total = 0;

        for delta_col in [-1, 0, 1].iter().cloned() {
            for delta_row in [-1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let neighbour_row = y as i32 + delta_row;
                let neighbour_col = x as i32 + delta_col;

                if !self.invalid_cell(neighbour_col, neighbour_row)
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

    fn visible_neighbours(&self, x: usize, y: usize) -> usize {
        let mut total = 0;

        for dx in [-1, 0, 1].iter().cloned() {
            for dy in [-1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let mut step = 1;
                'search: loop {
                    let new_x = x as i32 + (step * dx);
                    let new_y = y as i32 + (step * dy);
                    if self.invalid_cell(new_x, new_y) {
                        break 'search;
                    } else {
                        let cell = &self.cells[self.index(new_x as usize, new_y as usize)];
                        match *cell {
                            Cell::OccupiedSeat => {
                                total += 1;
                                break 'search;
                            }
                            Cell::EmptySeat => break 'search,
                            Cell::Floor => {
                                step += 1;
                            }
                        }
                    }
                }
            }
        }

        total
    }

    fn invalid_cell(&self, x: i32, y: i32) -> bool {
        y < 0 || y >= self.height as i32 || x < 0 || x >= self.width as i32
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
                    s.push(('0' as u8 + (self.visible_neighbours(x, y) as u8)) as char);
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

fn update_neighbour_count(s: &Seating, x: usize, y: usize) -> Cell {
    match s.cells[s.index(x, y)] {
        Cell::OccupiedSeat => {
            if s.neighbour_count(x, y) >= 4 {
                Cell::EmptySeat
            } else {
                Cell::OccupiedSeat
            }
        }
        Cell::EmptySeat => {
            if s.neighbour_count(x, y) == 0 {
                Cell::OccupiedSeat
            } else {
                Cell::EmptySeat
            }
        }
        Cell::Floor => Cell::Floor,
    }
}

fn update_visible_neighbours(s: &Seating, x: usize, y: usize) -> Cell {
    match s.cells[s.index(x, y)] {
        Cell::OccupiedSeat => {
            if s.visible_neighbours(x, y) >= 5 {
                Cell::EmptySeat
            } else {
                Cell::OccupiedSeat
            }
        }
        Cell::EmptySeat => {
            if s.visible_neighbours(x, y) == 0 {
                Cell::OccupiedSeat
            } else {
                Cell::EmptySeat
            }
        }
        Cell::Floor => Cell::Floor,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to load!")
    }

    let file = File::open(args.get(1).unwrap()).unwrap();

    let template_seating = Seating::load_from_file(file);
    println!("Loaded seating:");
    template_seating.display();
    println!();
    template_seating.display_neighbour_counts();

    let mut count = 0;

    let mut seating = template_seating.clone();
    while seating.update(update_neighbour_count) {
        count += 1;
    }

    println!("Part 1: completed after {} steps", count);
    println!("Part 1: {} occupied seats", seating.occupied_count());

    seating = template_seating.clone();
    while seating.update(update_visible_neighbours) {
        count += 1;
    }

    println!("Part 2: completed after {} steps", count);
    println!("Part 2: {} occupied seats", seating.occupied_count());
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
            // println!();
            updated = seating.update(crate::update_neighbour_count) && count < 100;
            count += 1;
        }

        count -= 1;

        assert_eq!(5, count);
        assert_eq!(37, seating.occupied_count());
    }

    #[test]
    fn check_sample_seat_visibility() {
        let f = File::open("sample.txt").unwrap();
        let mut seating = Seating::load_from_file(f);

        let mut updated = true;
        let mut count = 0;

        while updated {
            println!("Step {}", count);
            seating.display();
            println!();

            seating.display_neighbour_counts();
            println!();

            updated = seating.update(crate::update_visible_neighbours) && count < 100;
            count += 1;
        }

        count -= 1;
        assert_eq!(6, count);
        assert_eq!(26, seating.occupied_count());
    }
}
