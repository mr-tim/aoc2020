use crate::Cell::{Empty, Tree};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[repr(u8)]
#[derive(Clone, Copy)]
enum Cell {
    Empty = 0,
    Tree = 1,
}

struct Slope {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Slope {
    fn get_index(&self, x: usize, y: usize) -> usize {
        (x % self.width) + (y * self.width)
    }

    fn count_trees(&self, right: usize, down: usize) -> u64 {
        let mut x = 0;
        let mut y = 0;

        let mut count = 0;

        while y < self.height {
            let cell = self.cells[self.get_index(x, y)];
            count += cell as u64;
            x = (x + right) % self.width;
            y += down;
        }

        count
    }

    fn load(file: &File) -> Slope {
        let lines = BufReader::new(file).lines();
        let mut width = 0;
        let mut height = 0;
        let mut cells: Vec<Cell> = Vec::new();
        for r in lines {
            let line = r.unwrap();
            if width == 0 {
                width = line.len();
            } else {
                assert_eq!(width, line.len());
            }
            cells.extend(line.chars().map(|c| if c == '#' { Tree } else { Empty }));
            height += 1;
        }
        Slope {
            cells,
            width,
            height,
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

    let slope = Slope::load(&file);

    let tree_count = slope.count_trees(3, 1);

    println!("Part 1: Encountered {} trees", tree_count);

    println!("Part 2:");
    let dirs: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product = 0;
    for (right, down) in dirs {
        let count = slope.count_trees(right, down);
        println!("right {}, down {}: {}", right, down, count);
        if product == 0 {
            product = count;
        } else {
            product *= count;
        }
    }
    println!("Total product: {}", product);
}

#[cfg(test)]
mod tests {
    use crate::Slope;
    use std::fs::File;

    #[test]
    fn test_sample_count() {
        let f = File::open("sample.txt").unwrap();
        let slope = Slope::load(&f);
        let count = slope.count_trees(3, 1);
        assert_eq!(count, 7);
    }
}
