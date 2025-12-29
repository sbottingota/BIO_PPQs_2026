use std::io;

fn total_repaid(interest: u32, repayment: u32) -> u32 {
    let mut owed = 100_00;
    let mut repaid = 0;

    while owed > 0 {
        owed += (owed * interest).div_ceil(100);
        let mut repayment = (owed * repayment).div_ceil(100);
        if repayment < 50_00 {
            repayment = 50_00;
        }

        repaid += repayment.min(owed);
        owed -= repayment.min(owed);
    }

    repaid
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let tokens: Vec<u32> = input
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect();

    let interest = tokens[0];
    let repayment = tokens[1];

    println!("{:.2}", total_repaid(interest, repayment) as f64 / 100.0);

    Ok(())
}

