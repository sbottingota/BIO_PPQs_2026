use std::io;
use std::collections::HashSet;

// returns whether x lies between a and b
fn is_between(x: u32, a: u32, b: u32) -> bool {
    (a < x && x < b) || (b < x && x < a)
}

fn transformations(serial_num: &[u32]) -> Vec<Vec<u32>> {
    let mut ret = Vec::new();
    for i in 0..serial_num.len() - 1 {
        if i > 0 && is_between(serial_num[i-1], serial_num[i], serial_num[i+1])
            || i < serial_num.len() - 2 && is_between(serial_num[i+2], serial_num[i], serial_num[i+1]) {

            let mut new_num = serial_num.to_vec();
            (new_num[i], new_num[i+1]) = (new_num[i+1], new_num[i]);
            ret.push(new_num);
        }
    }

    ret
}

fn max_dist(serial_num: &[u32]) -> u32 {
    let mut to_visit = vec![serial_num.to_vec()];
    let mut seen = HashSet::from([serial_num.to_vec()]);

    for i in 0.. {
        let mut new_nums = Vec::new();

        for num in to_visit {
            for transformation in transformations(&num) {
                if !seen.contains(&transformation) {
                    new_nums.push(transformation.clone());
                    seen.insert(transformation);
                }
            }
        }

        if new_nums.is_empty() {
            return i;
        } else {
            to_visit = new_nums;
        }
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input)?;
    input.clear(); // discard first line of input

    stdin.read_line(&mut input)?;

    let serial_num: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!("{}", max_dist(&serial_num));

    Ok(())
}

