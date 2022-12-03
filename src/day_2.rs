use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_2_1() -> Result<(), Box<dyn Error>> {
    let filename = "data/day-2.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line = line?;
        // Only the last line is empty, and all the others are well formatted
        if !line.is_empty() {
            let mut shapes = line.split_whitespace();
            let them = shapes.next().expect("Invalid input").parse::<char>()?;
            let us = shapes.next().expect("Invalid input").parse::<char>()?;

            score += match (them, us) {
                ('A', 'X') => 1 + 3, // Rock, Rock
                ('A', 'Y') => 2 + 6, // Rock, Paper
                ('A', 'Z') => 3 + 0, // Rock, Scissors
                ('B', 'X') => 1 + 0, // Paper, Rock
                ('B', 'Y') => 2 + 3, // Paper, Paper
                ('B', 'Z') => 3 + 6, // Paper, Scissors
                ('C', 'X') => 1 + 6, // Scissors, Rock
                ('C', 'Y') => 2 + 0, // Scissors, Paper
                ('C', 'Z') => 3 + 3, // Scissors, Scissors
                _ => panic!("Invalid input"),
            };
        }
    }
    println!("Day 2, part 1: total score: {}", score);

    Ok(())
}

pub fn day_2_2() -> Result<(), Box<dyn Error>> {
    let filename = "data/day-2.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line = line?;
        // Only the last line is empty, and all the others are well formatted
        if !line.is_empty() {
            let mut shapes = line.split_whitespace();
            let them = shapes.next().expect("Invalid input").parse::<char>()?;
            let result = shapes.next().expect("Invalid input").parse::<char>()?;

            score += match (them, result) {
                ('A', 'X') => 3 + 0, // Rock, Loss (Scissors)
                ('A', 'Y') => 1 + 3, // Rock, Draw (Rock)
                ('A', 'Z') => 2 + 6, // Rock, Win (Paper)
                ('B', 'X') => 1 + 0, // Paper, Loss (Rock)
                ('B', 'Y') => 2 + 3, // Paper, Draw (Paper)
                ('B', 'Z') => 3 + 6, // Paper, Win (Scissors)
                ('C', 'X') => 2 + 0, // Scissors, Loss (Paper)
                ('C', 'Y') => 3 + 3, // Scissors, Draw (Scissors)
                ('C', 'Z') => 1 + 6, // Scissors, Win (Rock)
                _ => panic!("Invalid input"),
            };
        }
    }
    println!("Day 2, part 2: total score: {}", score);

    Ok(())
}
