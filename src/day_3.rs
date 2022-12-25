use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_3_1() -> Result<(), Box<dyn Error>> {
    let filename = "data/day-3.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum_prio = 0;

    for line in reader.lines() {
        let line = line?;
        // Only the last line is empty, and all the others are well formatted
        if !line.is_empty() {
            let middle = line.len() / 2;
            let first: HashSet<_> = line[..middle].bytes().collect();
            let second = line[middle..].bytes().collect::<HashSet<_>>();
            let both = first
                .intersection(&second)
                .next()
                .expect("Expected exactly one char in both compartments");
            let prio = match both {
                b'a'..=b'z' => (both - b'a' + 1) as usize,
                b'A'..=b'Z' => (both - b'A' + 27) as usize,
                _ => panic!("Invalid character: {}", both),
            };
            sum_prio += prio;
        }
    }
    println!("Day 3, part 1: sum of priorities: {}", sum_prio);

    Ok(())
}

pub fn day_3_2() -> Result<(), Box<dyn Error>> {
    let filename = "data/day-3.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum_prio = 0;

    let mut line_iter = reader.lines();
    loop {
        let mut lines = Vec::new();
        let mut brk = false;
        for _ in 0..3 {
            match line_iter.next() {
                Some(line) => lines.push(line?),
                None => {
                    brk = true;
                    break;
                }
            };
        }
        if brk {
            break;
        }
        // Only the last line is empty, and all the others are well formatted
        if lines.iter().any(|line| line.is_empty()) {
            break;
        }

        let mut sets: Vec<HashSet<u8>> = Vec::new();

        for line in lines.iter() {
            sets.push(line[..].bytes().collect());
        }

        let all = sets[0]
            .intersection(&sets[1])
            .find(|&c| sets[2].contains(c))
            .expect("Expected exactly one char in all rucksacks");
        let prio = match all {
            b'a'..=b'z' => (all - b'a' + 1) as usize,
            b'A'..=b'Z' => (all - b'A' + 27) as usize,
            _ => panic!("Invalid character: {}", all),
        };
        sum_prio += prio;
    }
    println!("Day 3, part 2: sum of priorities: {}", sum_prio);

    Ok(())
}
