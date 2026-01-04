use std::io;

fn primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = vec![2];

    for i in (3..=n).step_by(2) {
        if primes.iter().all(|p| i % *p != 0) {
            primes.push(i);
        }
    }

    primes
}

// returns the product of the distinct prime factors of n
fn prime_factor_prod(n: u32) -> u32 {
    let primes = primes_up_to(n);

    if *primes.last().unwrap() == n {
        return n;
    }

    primes
        .into_iter()
        .filter(|&p| p <= n / 2 && n % p == 0)
        .product()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u32 = input
        .trim()
        .parse()
        .unwrap();

    println!("{}", prime_factor_prod(n));

    Ok(())
}

