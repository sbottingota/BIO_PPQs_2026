use std::io;

fn next_row(row: &[char]) -> Vec<char> {
    let mut next = Vec::new();

    for i in 0..row.len() - 1 {
        if row[i] == row[i + 1] {
            next.push(row[i]);
        } else {
            if row[i] != 'R' && row[i + 1] != 'R' {
                next.push('R');
            } else if row[i] != 'G' && row[i + 1] != 'G' {
                next.push('G');
            } else {
                next.push('B');
            }
        }
    }

    next
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let mut row: Vec<char> = buf.trim().chars().collect();
    while row.len() > 1 {
        row = next_row(&row);
    }

    println!("{}", row[0]);

    Ok(())
}

