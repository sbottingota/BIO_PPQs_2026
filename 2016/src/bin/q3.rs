use std::collections::{HashSet, VecDeque};

// check if the difference between p and q is a power of 2
fn are_connected(p: usize, q: usize) -> bool {
    let diff = p.abs_diff(q);
    diff != 0 && (diff & (diff-1)) == 0
}

fn primes_up_to(n: usize) -> Vec<usize> {
    let mut primes = vec![2];

    let mut i = 3_usize;
    while i <= n {
        if primes.iter().all(|p| i % p != 0) {
            primes.push(i);
        }

        i += 2;
    }

    primes
}

// get the length of the shortest path from start to end, using primes less than or equal to limit
fn shortest_path_len(start: usize, end: usize, limit: usize) -> usize {
    let primes = primes_up_to(limit);

    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from([(start, 1)]); // (prime, path length so far)
    let mut seen: HashSet<usize> = HashSet::from([start]);

    while let Some((p, len)) = to_visit.pop_front() {
        for &q in &primes {
            if are_connected(p, q) && !seen.contains(&q) {
                if q == end {
                    return len + 1;
                } else {
                    to_visit.push_back((q, len + 1));
                    seen.insert(q);
                }
            }
        }
    }

    panic!("No path between {} and {} found through primes less than or equal to {}", start, end, limit)
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let &[limit, p, q] = &buf
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect::<Vec<usize>>()[..]
        else {
            panic!("Error parsing input");
        };

    println!("{}", shortest_path_len(p, q, limit));    

    Ok(())
}

