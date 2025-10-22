// TODO: make it correctly recognize which inputs are impossible without hanging

use std::io;
use std::collections::{VecDeque, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Off,
    Dim,
    Bright,
}

impl State {
    fn next(&self) -> Self {
        match self {
            Self::Off => Self::Dim,
            Self::Dim => Self::Bright,
            Self::Bright => Self::Off,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Grid {
    squares: [[State; Self::GRID_SIZE]; Self::GRID_SIZE],
}

impl Grid {
    const GRID_SIZE: usize = 5;

    fn new(states: &str) -> Self {
        let mut squares = [[const { State::Off }; Self::GRID_SIZE]; Self::GRID_SIZE];

        for c in states.chars() {
            let (x, y) = Self::get_square(c);

            if c.is_uppercase() {
                squares[x][y] = State::Bright;
            } else if c.is_lowercase() {
                squares[x][y] = State::Dim;
            } else {
                panic!("Invalid character '{}' found while trying to parse input", c);
            }
        }

        Self { squares }
    }

    fn pressed(&self, x: usize, y: usize) -> Self {
        let mut new = self.clone();
        new.squares[x][y] = new.squares[x][y].next();

        for (neighbor_x, neighbor_y) in Self::get_neighbors(x, y) {
            new.squares[neighbor_x][neighbor_y] = new.squares[neighbor_x][neighbor_y].next();
        }

        new
    }

    fn is_unlocked(&self) -> bool {
        for row in &self.squares {
            if !row.iter().all(|square| *square == State::Off) {
                return false;
            }
        }

        true
    }

    fn get_square(id: char) -> (usize, usize) {
        let square_idx = (id.to_ascii_uppercase() as u32 as usize) - ('A' as u32 as usize);
        (square_idx / Self::GRID_SIZE, square_idx % Self::GRID_SIZE)
    }

    fn get_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push((x-1, y));
        }
        if y > 0 {
            neighbors.push((x, y-1));
        }
        if x < Self::GRID_SIZE - 1 {
            neighbors.push((x+1, y));
        }
        if y < Self::GRID_SIZE - 1 {
            neighbors.push((x, y+1));
        }

        neighbors
    }

    fn print(&self) {
        for row in &self.squares {
            for square in row {
                print!("{} ", match square {
                    State::Off => '_',
                    State::Dim => 'x',
                    State::Bright => 'X',
                });
            }
            println!();
        }
    }
}

fn unlock(start: Grid) -> Option<String> {
    let mut to_visit = VecDeque::from([(start, String::new())]);
    let mut seen = HashSet::from([start]);

    while let Some((grid, path)) = to_visit.pop_front() {
        // grid.print();
        // println!("{}\n", path);

        for i in 0..Grid::GRID_SIZE * Grid::GRID_SIZE {
            let id = char::from_u32(i as u32 + 'a' as u32).unwrap();
            
            for press_times in (1..=2).rev() {
                if path.chars().filter(|c| c.to_ascii_lowercase() == id).count() == 0 { // ensure that every square is only pressed at most twice
                    let mut new_grid = grid.pressed(i / Grid::GRID_SIZE, i % Grid::GRID_SIZE);

                    if press_times == 2 {
                        new_grid = new_grid.pressed(i / Grid::GRID_SIZE, i % Grid::GRID_SIZE);
                    }

                    let mut new_path = path.clone();
                    new_path.push(if press_times == 1 { id } else { id.to_ascii_uppercase() });

                    if new_grid.is_unlocked() {
                        return Some(new_path);
                    }

                    if !seen.contains(&new_grid) {
                        seen.insert(new_grid.clone());
                        to_visit.push_back((new_grid, new_path));
                    }
                }
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let grid = Grid::new(buf.trim());

    println!("{}", unlock(grid).unwrap_or("IMPOSSIBLE".to_string()));
    
    Ok(())
}

