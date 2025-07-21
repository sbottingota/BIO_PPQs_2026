use std::io;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let &[mut n, mut i] = &buf
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect::<Vec<usize>>()[..]
        else { panic!("Invalid input") };

    i -= 1; // input uses 1 based indexing

    loop {
        let n_digits = n.ilog10() as usize + 1;
        if i < n_digits {
            break
        }

        i -= n_digits;
        
        n += 1;
    }

    println!("{}", n.to_string().chars().nth(i).unwrap());

    Ok(())
}

