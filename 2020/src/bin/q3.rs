use itertools::Itertools;
use itertools::repeat_n;

use std::io;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input)?;

    let tokens: Vec<usize> = input
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect();
    let n_letters = tokens[0];
    let max_adjacent_letters = tokens[1];
    let plan_len = tokens[2];

    input.clear();
    stdin.read_line(&mut input)?;

    let i = input.trim().parse::<usize>().unwrap() - 1; // input is 1-indexed

    let ith_plan = repeat_n(ALPHABET[..n_letters].chars(), plan_len)
        .multi_cartesian_product()
        .filter(|plan| plan
            .windows(max_adjacent_letters+1)
            .all(|window| !window.iter().all(|c| *c == window[0])))
        .nth(i)
        .unwrap();

    println!("{}", ith_plan.into_iter().collect::<String>());

    Ok(())
}

