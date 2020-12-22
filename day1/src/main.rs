use fs::File;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
};

static YEAR: i32 = 2020;


fn part_one(file_reader: BufReader<File>) {
    let mut number_to_count = HashMap::new();

    for line in file_reader.lines() {
        let number = line.expect("Failed to read line").parse::<i32>().unwrap();

        let counter = number_to_count.entry(number).or_insert(-1);
        *counter += 0;
    }

    for (number, _) in &number_to_count {
        let complement = YEAR - number;

        if let Some(count) = number_to_count.get(&complement) {
            if &complement == number && count == &0 {
                continue;
            } else {
                println!("Entries: {} and {}", number, complement);
                println!("Solution: {}", number * complement);
            }
        }
    }
}

fn part_2(file_reader: BufReader<File>) {
    let mut numbers = Vec::new();

    for line in file_reader.lines() {
        let number = line.expect("Failed to read line").parse::<i32>().unwrap();
        numbers.push(number);
    }

    for (i, number) in numbers.iter().enumerate() {
        for (j, number_2) in numbers[i+1..].iter().enumerate() {
            for (k, number_3) in numbers[j + 1..].iter().enumerate() {
                if number + number_2 + number_3 == YEAR {
                    println!("Entries: {}, {}, and {} sum to 2020.", number, number_2, number_3);
                    println!("Solution: {}", number * number_2 * number_3);
                    return
                }
            }
        }
    }

    
}

fn main() {
    println!("Reading from file!");
    let filename = "input.txt";

    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    part_2(reader);

}
