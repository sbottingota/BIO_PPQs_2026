use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pentomino {
    squares: Vec<Vec<bool>>, // true = filled, false = unfilled
}

impl Pentomino {
    fn new(n_rows: usize, n_cols: usize) -> Self {
        Self { squares: vec![vec![false; n_cols]; n_rows], }
    }

    fn crop(&mut self) {
        self.squares.retain(|row| row.contains(&true));

        // columns
        let mut i = 0_usize;
        while i < self.squares[0].len() {
            if self.squares.iter().all(|row| row[i] == false) {
                self.squares.iter_mut().for_each(|row| { row.remove(i); });
            } else {
                i += 1;
            }
        }
    }

    // extend the pentomino in all directions, padding with false
    fn extend(&mut self, rows: usize, cols: usize) {
        for _ in 0..rows {
            self.squares.insert(0, vec![false; self.squares[0].len()]);
            self.squares.push(vec![false; self.squares[0].len()]);
        }

        for row in &mut self.squares {
            let mut new_row = vec![false; cols];
            new_row.extend(row.clone());
            new_row.extend(vec![false; cols]);
            *row = new_row;
        }
    }

    // if the pentominos overlap when merged, or aren't adjacent when merged, returns None
    // otherwise returns this pentomino merged with other, with the top left corner of other at (row, col)
    fn combine(&self, other: &Self, row: usize, col: usize) -> Option<Self> {
        let mut new = self.clone();

        let mut are_adjacent = false;

        for i in 0..other.squares.len() {
            for j in 0..other.squares[i].len() {
                if other.squares[i][j] && new.squares[i + row][j + col] {
                    return None;
                } else {
                    new.squares[i + row][j + col] = other.squares[i][j];

                    // try to verify whether the pentominoes are adjacent when merged like this, if this has not already been verified
                    if !are_adjacent && other.squares[i][j] == true {
                        if (i > 0 && self.squares[i - 1][j] == true) || (j > 0 && self.squares[i][j - 1] == true) {
                            are_adjacent = true;
                        }
                    }
                }
            }
        }

        new.crop();
        for row in &new.squares {
            println!("{}", row.iter().map(|x| if *x == true { '#' } else { ' ' }).collect::<String>());
        }

        if are_adjacent {
            Some(new)
        } else {
            None
        }
    }
}

impl From<char> for Pentomino {
    fn from(c: char) -> Self {
        Self {
            squares: match c {
                'F' => vec![
                        vec![false, true, true],
                        vec![true, true, false],
                        vec![false, true, false],
                    ],
                _   => panic!("Invalid pentomino type '{}'", c),
            },
        }
    }
}

fn main() {
    let mut p1 = Pentomino::from('F');
    p1.extend(3, 3);
    let p2 = Pentomino::from('F');

    let p3 = p1.combine(&p2, 3, 1).unwrap();

    for row in p3.squares {
        println!("{}", row.iter().map(|x| if *x == true { '#' } else { ' ' }).collect::<String>());
    }
}

