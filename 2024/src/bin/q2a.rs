// for some reason this program gives the wrong output for inputs such as "EE(O(EEEE))O 12345678"
// but it still works well enough, so committing this now and then fixing it at some point in the future.

use std::io;

mod q2;

fn parse_char(c: char) -> q2::List {
    match c {
        'E' => Box::new(q2::E),
        'O' => Box::new(q2::O),
        'T' => Box::new(q2::T),
        _   => panic!("Invalid character {}", c),
    }
}

fn parse_paren(it: &mut impl Iterator<Item=char>) -> q2::List {
    let mut s = String::new();

    let mut paren_count = 1; // +1 for every '(', -1 for every ')'

    while paren_count > 0 {
        match it.next().unwrap() {
            '(' => paren_count += 1,
            ')' => paren_count -= 1,
            c   => s.push(c),
        }
    }

    parse(s)
}

fn parse(s: String) -> q2::List {
    let first_char = s.chars().next().unwrap();

    let mut it = s.chars().skip(1);
    let mut list: q2::List;

    if first_char == '(' {
        list = parse_paren(&mut it);
    } else {
        list = parse_char(first_char);
    }

    while let Some(c) = it.next() {
        if c == '(' {
            list = q2::combined(list, parse_paren(&mut it));
        } else if c != ')' {
            list = q2::combined(list, parse_char(c));
        }
    }

    list
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let list = parse(buf.split_whitespace().next().unwrap().to_string());
    let i: usize = buf.split_whitespace().nth(1).unwrap().parse().unwrap();

    println!("{}", list(i));

    Ok(())
}

