use std::io;

fn fibonacci_up_to(n: usize) -> Vec<usize> {
    let mut ret = vec![1];

    let mut a = 1_usize;
    let mut b = 1_usize;

    while a + b <= n {
        let c = a + b;
        ret.push(c);

        a = b;
        b = c;
    }

    ret
}

fn zeckendorf(mut n: usize) -> Vec<usize> {
    let mut repr = Vec::new();

    let mut fibonacci = fibonacci_up_to(n);

    while n > 0 {
        let idx = fibonacci.partition_point(|&x| x <= n) - 1; // largest fibonacci number which is <= n
        repr.push(fibonacci[idx]);
        n -= fibonacci[idx];

        if idx >= 1 {
            fibonacci.remove(idx - 1); // ensure adjacent fibonacci numbers aren't used
        }
    }

    repr
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf)?;

    let repr = zeckendorf(buf.trim().parse().unwrap());

    for n in repr {
        print!("{} ", n);
    }
    println!();

    Ok(())
}

