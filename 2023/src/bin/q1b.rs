fn main() {
    let mut a = 1;
    let mut b = 1;

    while a + b < 1_000_000 {
        let c = a + b;
        a = b;
        b = c;
    }

    println!("{}", b);
}

