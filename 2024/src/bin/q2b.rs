mod q2;

fn main() {
    #[allow(non_snake_case)]
    let TT = q2::combined(Box::new(q2::T), Box::new(q2::T));

    let mut count = 0;
    for i in 1..200 {
        if TT(i) == 2 {
            count += 1;
        }
    }

    println!("{}", count);
}

