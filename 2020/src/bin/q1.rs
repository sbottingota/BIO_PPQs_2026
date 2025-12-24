use std::io;

const VALUES: [u32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
const SYMBOLS: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

fn to_numeral(mut n: u32) -> Vec<char> {
    let mut numeral = Vec::new();

    while n > 0 {
        for (i, value) in VALUES.iter().enumerate() {
            if n >= *value {
                n -= value;
                numeral.extend(SYMBOLS[i].chars().collect::<Vec<_>>());
                break;
            }
        }
    }

    numeral
}

fn look_and_say(numeral: &[char]) -> Vec<char> {
    let mut ret = Vec::new();

    let mut i = 0_usize;
    while i < numeral.len() {
        let mut j = i + 1;
        while j < numeral.len() && numeral[j] == numeral[i] {
            j += 1;
        }

        let len = j - i;
        ret.extend(to_numeral(len as u32));
        ret.push(numeral[i]);

        i += len;
    }

    ret
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut numeral: Vec<char> = input.split_whitespace().nth(0).unwrap().chars().collect();
    let i: u32 = input.split_whitespace().nth(1).unwrap().parse().unwrap();

    for _ in 0..i {
        numeral = look_and_say(&numeral);
    }

    println!("{} {}", 
        numeral.iter().filter(|c| **c == 'I').count(),
        numeral.iter().filter(|c| **c == 'V').count(),
    );

    Ok(())
}

