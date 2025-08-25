use std::io;
use std::collections::{VecDeque, HashSet};

const MAX_STACK_HEIGHT: usize = 4;

fn possible_moves_from(state: Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
    let mut moves = Vec::new();

    for start in 0..state.len() {
        if state[start].is_empty() {
            continue;
        }

        for end in 0..state.len() {
            if start == end || state[end].len() >= MAX_STACK_HEIGHT {
                continue;
            }

            let mut new_state = state.clone();

            let val = new_state[start].pop().unwrap();
            new_state[end].push(val);
            moves.push(new_state);
        }
    }

    moves
}

fn get_min_moves(start: Vec<Vec<usize>>, end: Vec<Vec<usize>>) -> usize {
    if start == end {
        return 0;
    }

    let mut moves = 1_usize;

    let mut to_visit_now = VecDeque::from([start]);
    let mut visited: HashSet<Vec<Vec<usize>>> = HashSet::new();

    let mut to_visit_next: VecDeque<Vec<Vec<usize>>> = VecDeque::new();

    while !to_visit_now.is_empty() {
        while let Some(state) = to_visit_now.pop_back() {
            visited.insert(state.clone());

            for next in possible_moves_from(state) {
                if next == end {
                    return moves;
                } else {
                    if !visited.contains(&next) {
                        to_visit_next.push_front(next);
                    }
                }
            }
        }

        to_visit_now = to_visit_next;
        to_visit_next = VecDeque::new();
        moves += 1;
    }

    panic!("Ran out of states to explore")
}

fn parse_state(s: &str) -> Vec<Vec<usize>> {
    let mut state = Vec::new();

    for stack in s.split_whitespace() {
        if stack == "0" {
            state.push(Vec::new());
        } else {
            state.push(stack.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
        }
    }

    state
}

fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let start = parse_state(&buf);

    buf.clear();
    io::stdin().read_line(&mut buf)?;
    let end = parse_state(&buf);

    println!("{}", get_min_moves(start, end));

    Ok(())
}

