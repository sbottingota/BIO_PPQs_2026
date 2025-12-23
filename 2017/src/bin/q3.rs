use std::io;
use std::cmp::min;

use memoize::memoize;


type Parcel = Vec<u32>;

#[memoize]
fn parcels(weight: u32, parts: u32, min_item_weight: u32, max_item_weight: u32) -> Vec<Parcel> {
    if parts == 1 {
        if weight >= min_item_weight && weight <= max_item_weight {
            return vec![vec![weight]];
        } else {
            return vec![];
        }
    }

    let mut ret = Vec::new();

    for i in min_item_weight..=min(max_item_weight, weight - 1) {
        for mut parcel in parcels(weight - i, parts - 1, i, max_item_weight) {
            parcel.push(i);
            ret.push(parcel);
        }
    }

    ret
}

#[memoize]
fn parcel_groups(n_parcels: u32, max_item_weight: u32, total_items: u32, parcel_weight: u32) -> Vec<Vec<Parcel>> {
    if n_parcels == 1 {
        return parcels(parcel_weight, total_items, 1, max_item_weight)
            .into_iter()
            .map(|parcel| vec![parcel])
            .collect();
    }

    let mut groups = Vec::new();

    for parcel_size in 1..=total_items - n_parcels + 1 {
        let subgroups = parcel_groups(n_parcels - 1, max_item_weight, total_items - parcel_size, parcel_weight);

        for parcel in parcels(parcel_weight, parcel_size, 1, max_item_weight) {
            for mut group in subgroups.clone() {
                group.push(parcel.clone());
                groups.push(group);
            }
        }
    }

    groups
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let tokens: Vec<u32> = input
        .split_whitespace()
        .map(|substr| substr.parse().unwrap())
        .collect();

    let n_parcels = tokens[0];
    let max_item_weight = tokens[1];
    let total_items = tokens[2];
    let parcel_weight = tokens[3];

    let groups = parcel_groups(n_parcels, max_item_weight, total_items, parcel_weight);

    // println!("{:?}", groups);
    println!("{}", groups.len());

    Ok(())
}
