use std::io;

fn get_dial(n: usize) -> Vec<char> {
    let mut circle: Vec<char> = ('A'..='Z').collect();
    let mut dial: Vec<char> = Vec::new();

    let mut i = 0_usize;

    while !circle.is_empty() {
        i = (i + n - 1) % circle.len();
        dial.push(circle[i]);
        circle.remove(i);
    }

    dial
}

fn encrypt(word: &[usize], dial: &[char]) -> String {
    let mut encrypted = String::new();

    for (offset, i) in word.iter().enumerate() {
        encrypted.push(dial[(i + offset) % dial.len()]);
    }

    encrypted
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut tokens = input.split_whitespace();

    let n: usize = tokens
        .next().unwrap()
        .parse().unwrap();

    let word: Vec<usize> = tokens
        .next().unwrap()
        .chars()
        .map(|c| (c as u32 - 'A' as u32) as usize)
        .collect();


    let dial = get_dial(n);

    println!("{}", dial[..6].iter().collect::<String>());
    println!("{}", encrypt(&word, &dial));

    Ok(())
}

