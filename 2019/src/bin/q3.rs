use std::io;

use itertools::Itertools;

fn is_valid_chain(chain: &[char]) -> bool {
    for i in 0..chain.len() - 2 {
        for j in i+1..chain.len() - 1 {
            for k in j+1..chain.len() {
                if chain[i] < chain[j] && chain[j] < chain[k] {
                    return false;
                }
            }
        }
    }

    true
}

fn count_blockchains(start: &[char], length: usize) -> usize {
    if start.len() == length {
        return 1;
    }

    let mut count = 0;

    let charset: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWYXZ"
        .chars()
        .take(length)
        .filter(|c| !start.contains(&c))
        .collect();

    for end in charset.into_iter().permutations(length - start.len()) {
        let chain: Vec<char> = start.into_iter().copied().chain(end.into_iter()).collect();

        if is_valid_chain(&chain) {
            count += 1;
        }
    }

    count
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let length: usize = buf
        .split_whitespace()
        .next()
        .unwrap()
        .parse()   
        .unwrap();

    let start: Vec<char> = buf
        .split_whitespace()
        .nth(1)
        .unwrap()
        .chars()
        .collect();

    println!("{}", count_blockchains(&start, length));

    Ok(())
}

