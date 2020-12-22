use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Command {
    N(usize),
    S(usize),
    E(usize),
    W(usize),
    L(usize),
    R(usize),
    F(usize),
}

impl Command {
    fn from_string(s: String) -> Command {
        let first = &s[0..1];
        let value: usize = s[1..].parse().unwrap();
        match first {
            "N" => Command::N(value),
            "S" => Command::S(value),
            "E" => Command::E(value),
            "W" => Command::W(value),
            "L" => Command::L(value),
            "R" => Command::R(value),
            "F" => Command::F(value),
            _ => panic!("invalid command"),
        }
    }
}

fn load_commands(f: File) -> Vec<Command> {
    let mut result = Vec::new();
    for r in BufReader::new(f).lines() {
        if let Ok(line) = r {
            result.push(Command::from_string(line));
        }
    }
    result
}

struct Position {
    x: i32,
    y: i32,
    heading: i32,
}

impl Position {
    fn execute(&mut self, cmd: Command) {
        match cmd {
            Command::N(value) => self.y += value as i32,
            Command::E(value) => self.x += value as i32,
            Command::S(value) => self.y -= value as i32,
            Command::W(value) => self.x -= value as i32,
            Command::L(value) => self.heading = (self.heading + 360 - value as i32) % 360,
            Command::R(value) => self.heading = (self.heading + 360 + value as i32) % 360,
            Command::F(value) => match self.heading {
                0 => self.execute(Command::N(value)),
                90 => self.execute(Command::E(value)),
                180 => self.execute(Command::S(value)),
                270 => self.execute(Command::W(value)),
                _ => {
                    println!("Oops, heading = {}", self.heading);
                    panic!("invalid heading!")
                }
            },
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn rotate(&mut self, angle: i32) {
        let (old_x, old_y) = (self.x, self.y);

        let (c, s) = match angle {
            -270 => (0, 1),
            -180 => (-1, 0),
            -90 => (0, -1),
            0 => (1, 0),
            90 => (0, 1),
            180 => (-1, 0),
            270 => (0, -1),
            _ => panic!("invalid angle"),
        };

        self.x = c * old_x - s * old_y;
        self.y = s * old_x + c * old_y;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please specify input");
    }
    let f = File::open(args.get(1).unwrap()).unwrap();
    let cmds = load_commands(f);
    let mut pos = Position {
        x: 0,
        y: 0,
        heading: 90,
    };

    cmds.iter().for_each(|cmd| pos.execute(*cmd));

    println!("Part 1: manhattan distance: {}", pos.manhattan_distance());

    let mut waypoint_pos = Position {
        x: 10,
        y: 1,
        heading: 0,
    };
    let mut ship_pos = Position {
        x: 0,
        y: 0,
        heading: 0,
    };

    cmds.iter().for_each(|cmd| {
        match cmd {
            Command::F(value) => {
                let m = *value as i32;
                ship_pos.x = ship_pos.x + m * waypoint_pos.x;
                ship_pos.y = ship_pos.y + m * waypoint_pos.y;
            }
            Command::L(value) => waypoint_pos.rotate(*value as i32),
            Command::R(value) => waypoint_pos.rotate(-(*value as i32)),
            _ => waypoint_pos.execute(*cmd),
        }
        // println!("ship: {}, {}, waypoint: {}, {}", ship_pos.x, ship_pos.y, waypoint_pos.x, waypoint_pos.y);
    });

    println!(
        "Part 2: manhattan distance: {}",
        ship_pos.manhattan_distance()
    );
}

#[cfg(test)]
mod tests {
    use crate::{load_commands, Command, Position};
    use std::fs::File;

    #[test]
    fn test_loading() {
        let f = File::open("sample.txt").unwrap();
        let commands = load_commands(f);
        assert_eq!(Command::F(10), commands[0]);
        assert_eq!(Command::N(3), commands[1]);
        assert_eq!(Command::F(7), commands[2]);
        assert_eq!(Command::R(90), commands[3]);
        assert_eq!(Command::F(11), commands[4]);
    }

    #[test]
    fn test_execution() {
        let f = File::open("sample.txt").unwrap();
        let commands = load_commands(f);
        let mut pos = Position {
            x: 0,
            y: 0,
            heading: 90,
        };

        commands.iter().for_each(|cmd| pos.execute(*cmd));

        assert_eq!(pos.x, 17);
        assert_eq!(pos.y, -8);
        assert_eq!(pos.manhattan_distance(), 25);
    }
}
