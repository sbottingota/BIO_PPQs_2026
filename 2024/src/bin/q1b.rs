fn main() {
    let mut n = 1_i32;
    let mut digits = 101_i32;

    let mut count = 0;

    while digits > 0 {
        for c in n.to_string().chars() {
            if c == '5' {
                count += 1;
            }
        }

        digits -= n.ilog10() as i32 + 1;

        n += 1;
    }

    println!("{}", count);
}

