// TODO: make this work for all outputs (rather than just for the first two moves)

use std::io;

#[derive(Clone)]
struct State {
    grid: [[bool; 5]; 5], // true = occupied, false = empty

    player_orders: Vec<Vec<usize>>,

    pieces: [[(isize, isize); 5]; 2],
    neutron: (isize, isize),

    current_move: usize,
}

impl State {
    const DIRECTIONS: [(isize, isize); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (-1, 0), (-1, -1)];
    const WIN_ROWS: [isize; 2] = [4, 0]; // rows on which, if the neutron lands, a player wins

    fn new(player1_order: &[usize], player2_order: &[usize]) -> Self {
        let grid = [[true; 5], [false; 5], [false, false, true, false, false], [false; 5], [true; 5]];
        let pieces = [[(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)], [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]];

        Self {
            grid,
            player_orders: vec![player1_order.to_vec(), player2_order.to_vec()],
            pieces,
            neutron: (2, 2),
            current_move: 0,
        }
    }

    fn move_piece(&mut self) {
        let player = self.current_move % 2;
        let piece = self.player_orders[player][(self.current_move / 2) % self.player_orders[player].len()];

        let (x, y) = self.pieces[player][piece];

        for direction in Self::DIRECTIONS {
            let Some((new_x, new_y)) = self.get_move(x, y, direction) else { continue };

            assert!(!self.grid[new_x as usize][new_y as usize]);

            self.grid[x as usize][y as usize] = false;
            self.grid[new_x as usize][new_y as usize] = true;
            self.pieces[player][piece] = (new_x, new_y);

            self.current_move += 1;

            return;
        }

        panic!("No valid move found")
    }

    fn move_neutron(&mut self) {
        let player = self.current_move % 2;

        // if the player can immediately win the game for themselves, do so
        for direction in Self::DIRECTIONS {
            let Some((new_x, new_y)) = self.get_move(self.neutron.0, self.neutron.1, direction) else { continue };
            if new_x == Self::WIN_ROWS[player] {
                self.grid[self.neutron.0 as usize][self.neutron.1 as usize] = false;
                self.grid[new_x as usize][new_y as usize] = true;
                self.neutron = (new_x, new_y);
                return;
            }
        }

        for direction in Self::DIRECTIONS {
            let Some((new_x, new_y)) = self.get_move(self.neutron.0, self.neutron.1, direction) else { continue };
            // if this move would lose the game for the player, continue
            if new_x == Self::WIN_ROWS[(player+1) % 2] {
                continue;
            }

            self.grid[self.neutron.0 as usize][self.neutron.1 as usize] = false;
            self.grid[new_x as usize][new_y as usize] = true;
            self.neutron = (new_x, new_y);
            return;
        }

        panic!("No valid move found")
    }

    fn get_move(&self, mut x: isize, mut y: isize, direction: (isize, isize)) -> Option<(isize, isize)> {
        // check whether the piece can move in the given direction
        if !self.is_valid_square(x + direction.0, y + direction.1) {
            return None;
        }

        while self.is_valid_square(x + direction.0, y + direction.1) {
            x += direction.0;
            y += direction.1;
        }

        Some((x, y))
    }

    fn is_game_finished(&self) -> bool {
        Self::WIN_ROWS.contains(&self.neutron.0)
    }

    fn is_valid_square(&self, x: isize, y: isize) -> bool {
        Self::is_in_bounds(x, y) && !self.grid[x as usize][y as usize]
    }

    fn is_in_bounds(x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < 5 && y < 5
    }

    fn print(&self) {
        for (x, row) in self.grid.iter().enumerate() {
            for (y, is_occupied) in row.iter().enumerate() {
                let (x, y) = (x as isize, y as isize);
                if *is_occupied {
                    if self.neutron == (x, y) {
                        print!("*");
                    } else if self.pieces[0].contains(&(x, y)) {
                        print!("F");
                    } else if self.pieces[1].contains(&(x, y)) {
                        print!("S");
                    } else {
                        panic!("Rogue piece found at ({}, {})", x, y);
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let stdin = io::stdin();

    let mut orders = Vec::new();
    for _ in 0..2 {
        buf.clear();
        stdin.read_line(&mut buf)?;

        // substract 1, as in this implementation, pieces are 0 indexed
        let order: Vec<usize> = buf.split_whitespace().map(|substr| substr.parse::<usize>().unwrap() - 1).collect();
        orders.push(order);
    }

    let mut state = State::new(&orders[0], &orders[1]);

    for _ in 0..2 {
        state.move_neutron();
        state.move_piece();
        state.print();
    }

    loop {
        state.move_neutron();
        if state.is_game_finished() {
            break;
        }

        state.move_piece();
    }

    state.print();

    Ok(())
}

