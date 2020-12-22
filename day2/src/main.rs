use anyhow::{Context, Error, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
struct Constraint {
    letter: char,
    lower_bound: i32,
    upper_bound: i32,
}

impl FromStr for Constraint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Inputs look something like: 1-3 a
        let bounds_and_letter = s.split(" ").collect::<Vec<&str>>();

        let letter = bounds_and_letter[1].chars().collect::<Vec<char>>()[0];

        let bounds = bounds_and_letter[0];

        let lower_and_upper_bounds = bounds.split("-").collect::<Vec<&str>>();

        let lower_bound: i32 = lower_and_upper_bounds[0]
            .parse()
            .expect("Failed to get lower bound");
        let upper_bound: i32 = lower_and_upper_bounds[1]
            .parse()
            .expect("Failed to get upper bound");

        Ok(Constraint {
            letter: letter,
            lower_bound,
            upper_bound,
        })
    }
}

impl Constraint {
    pub fn is_password_valid(&self, password: &str) -> bool {
        let mut letters_to_count = HashMap::new();

        for char in password.chars() {
            let counter = letters_to_count.entry(char).or_insert(0);
            *counter += 1;
        }

        let count = *letters_to_count.get(&self.letter).unwrap_or(&0);

        if count <= self.upper_bound && count >= self.lower_bound {
            true
        } else {
            false
        }
    }
}

/// The constraint from part 2 of the problem. It's ugly, I know, whatever.
struct Constraint2 {
    index_1: usize,
    index_2: usize,
    letter: char,
}

impl FromStr for Constraint2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Inputs look something like: 1-3 a
        let index_and_letter = s.split(" ").collect::<Vec<&str>>();

        let letter = index_and_letter[1].chars().collect::<Vec<char>>()[0];

        let bounds = index_and_letter[0];

        let lower_and_upper_idx = bounds.split("-").collect::<Vec<&str>>();

        let index_1: usize = lower_and_upper_idx[0]
            .parse()
            .expect("Failed to get lower bound");
        let index_2: usize = lower_and_upper_idx[1]
            .parse()
            .expect("Failed to get upper bound");

        Ok(Constraint2 {
            index_1,
            index_2,
            letter,
        })
    }
}

impl Constraint2 {
    fn is_password_valid(&self, password: &str) -> bool {
        // Taking advantage of the fact that my shitty string splitting leaves
        // a space in front of the password.
        let characters: Vec<char> = password.chars().collect::<Vec<char>>();

        let char_1 = *characters.get(self.index_1).expect("oopsie");
        let char_2 = *characters.get(self.index_2).expect("big oopsie");

        if char_1 == self.letter && char_2 == self.letter {
            return false;
        } else {
            if char_1 == self.letter || char_2 == self.letter {
                return true;
            } else {
                return false;
            }
        }
    }
}

/// Reads all the contents of the file into memory as a vector of strings.
/// Shitty, don't use anywhere you care about.
fn load_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.expect("Failed to read line"));
    }

    lines
}

fn is_password_valid_part_1(line: &str) -> Result<bool> {
    let constraint_and_password = line.split(":").collect::<Vec<&str>>();

    let constraint =
        Constraint::from_str(constraint_and_password[0]).context("Failed to parse constraint")?;

    println!("====================");
    println!("Line: {}", line);
    println!("Constraint: {:#?}", constraint);
    let password = constraint_and_password[1];
    println!("Password: {}", password);
    println!("Is valid: {}", constraint.is_password_valid(password));

    Ok(constraint.is_password_valid(password))
}

fn is_password_valid_part_2(line: &str) -> Result<bool> {
    let constraint_and_password = line.split(":").collect::<Vec<&str>>();

    let constraint =
        Constraint2::from_str(constraint_and_password[0]).context("Failed to parse constraint")?;
    let password = constraint_and_password[1];

    Ok(constraint.is_password_valid(password))
}

fn main() {
    let file = load_file("input.txt");

    let mut count = 0;
    for line in file {
        if is_password_valid_part_2(&line).expect("Failed to check if password was valid :(") {
            count += 1;
        }
    }
    println!("Count: {}", count)
}
