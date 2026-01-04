fn primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = vec![2];

    for i in (3..=n).step_by(2) {
        if primes.iter().all(|p| i % *p != 0) {
            primes.push(i);
        }
    }

    primes
}

fn main() {
    println!("{:?}", primes_up_to(999_999));
}

