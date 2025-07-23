use std::io;
use std::fmt;
use std::ops::AddAssign;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Forward,
    Right,
    Back,
    Left,
}

impl Direction {
    const N_ITEMS: usize = 4;

    fn id(&self) -> usize {
        match self {
            Self::Forward => 0,
            Self::Right => 1,
            Self::Back => 2,
            Self::Left => 3,
        }
    }

    fn from_id(id: usize) -> Self {
        match id {
            0 => Self::Forward,
            1 => Self::Right,
            2 => Self::Back,
            3 => Self::Left,
            _ => panic!("Invalid direction id {}", id),
        }
    }

    fn move_square(&self, square: (isize, isize)) -> (isize, isize) {
        match self {
            Self::Forward => (square.0, square.1 + 1),
            Self::Back => (square.0, square.1 - 1),
            Self::Left => (square.0 - 1, square.1),
            Self::Right => (square.0 + 1, square.1),
        }
    }
}

impl AddAssign for Direction {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::from_id((self.id() + rhs.id()) % Self::N_ITEMS)
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'F' => Self::Forward,
            'R' => Self::Right,
            'L' => Self::Left,
            _   => panic!("Invalid char {}", c)
        }
    }
}


struct Game {
    trail: Vec<(isize, isize)>,

    explorer_pos: (isize, isize),
    explorer_direction: Direction,

    trail_len: usize,
    instructions: Vec<Direction>,
    n_moves: usize,

    move_count: usize,
}

impl Game {
    fn new(trail_len: usize, instructions: &str, n_moves: usize) -> Self {
        Self {
            trail: Vec::new(),

            explorer_pos: (0, 0),
            explorer_direction: Direction::Forward,
            
            trail_len,
            instructions: instructions.chars().map(|c| Direction::from(c)).collect(),
            n_moves,

            move_count: 0,
        }
    }

    // returns whether explorer can still move
    fn play_move(&mut self) -> bool {
        self.trail.push(self.explorer_pos);
        if self.trail.len() > self.trail_len {
            self.trail.remove(0);
        }

        let instruction = self.instructions[self.move_count % self.instructions.len()];
        self.explorer_direction += instruction;

        let mut can_move = false;
        for _ in 0..4 {
            let next_square = self.explorer_direction.move_square(self.explorer_pos);
            if !self.trail.contains(&next_square) {
                can_move = true;
                self.explorer_pos = next_square;
                break;
            }

            self.explorer_direction += Direction::Right;
        }

        self.move_count += 1;
        can_move && self.move_count < self.n_moves
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("trail", &self.trail)
            .field("explorer_pos", &self.explorer_pos)
            .field("explorer_direction", &self.explorer_direction)
            .finish()
    }
}


fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;    

    let trail_len: usize = buf.split_whitespace().nth(0).unwrap().parse().unwrap();
    let instructions = buf.split_whitespace().nth(1).unwrap();
    let n_moves: usize = buf.split_whitespace().nth(2).unwrap().parse().unwrap();

    let mut game = Game::new(trail_len, instructions, n_moves);

    let mut can_continue = true;
    while can_continue {
        can_continue = game.play_move();
    }

    println!("{:?}", game.explorer_pos);

    Ok(())
}

