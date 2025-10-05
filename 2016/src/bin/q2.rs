use std::collections::HashMap;

const OVERCROWDING_THRESHOLD: usize = 4;

struct Grid {
    people: HashMap<(isize, isize), usize>,
}

impl Grid {
    fn new() -> Self {
        Self { people: HashMap::new() }
    }

    fn insert(&mut self, pos: (isize, isize), n: usize) {
        if let Some(x) = self.people.get_mut(&pos) {
            *x += n;
        } else {
            self.people.insert(pos, n);
        }
    }

    fn fix_overcrowding(&self) -> Self {
        let mut old_grid = self.people.clone();
        let mut new_grid = Self::new();

        let mut fixes_applied = false;

        loop {
            for (&(x, y), &n) in &old_grid {
                if n < OVERCROWDING_THRESHOLD {
                    new_grid.insert((x, y), n);
                } else {
                    new_grid.insert((x, y), n - OVERCROWDING_THRESHOLD);

                    for pos in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
                        new_grid.insert(pos, 1);
                    }

                    fixes_applied = true;
                }
            }

            if fixes_applied {
                old_grid = new_grid.people;
                new_grid = Self::new();

                fixes_applied = false;
            } else {
                return new_grid;
            }
        }
    }

    fn print(&self) {
        for x in 0..5 {
            for y in 0..5 {
                if let Some(n) = self.people.get(&(x, y)) {
                    print!("{} ", n);
                } else {
                    print!("0 ");
                }
            }

            println!();
        }
    }
}

fn get_coords(square: usize) -> (isize, isize) {
    ((square as isize - 1) / 5, (square as isize - 1) % 5)
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let &[start_pos, _seq_size, steps] = &buf
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect::<Vec<usize>>()[..]
        else {
            panic!("Invalid input");
        };

    buf.clear();
    std::io::stdin().read_line(&mut buf)?;

    let seq: Vec<usize> = buf
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect();


    let mut grid = Grid::new();

    let mut pos = start_pos;
    for i in 0..steps {
        grid.insert(get_coords(pos), 1);
        grid = grid.fix_overcrowding();

        pos += seq[i % seq.len()];
        pos += 24;
        pos %= 25;
        pos += 1;
    }

    println!();
    grid.print();

    Ok(())
}

