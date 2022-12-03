use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

pub fn day_1_1() -> Result<(), Box<dyn Error>> {
    let filename = "data/day-1.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut curr = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            max = cmp::max(max, curr);
            curr = 0;
        } else {
            curr += line.parse::<usize>()?;
        }
    }
    println!("Day 1, part 1: max: {}", max);

    Ok(())
}

pub fn day_1_2() -> Result<(), Box<dyn Error>> {
    let filename = "data/day-1.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    const N: usize = 3;
    let mut max_n = [0; N];
    let mut curr = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            for max in max_n.iter_mut() {
                if curr > *max {
                    mem::swap(max, &mut curr);
                }
            }
            curr = 0;
        } else {
            curr += line.parse::<usize>()?;
        }
    }
    println!(
        "Day 1, part 2: total: {}, values: {:?}",
        max_n.iter().sum::<usize>(),
        max_n
    );

    Ok(())
}
