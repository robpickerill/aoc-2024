use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "input";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // let part1 = part1(reader);
    // println!("part 1: {}", &part1);

    let part2 = part2(reader);
    println!("part 2: {}", &part2);

    Ok(())
}

fn part1(reader: BufReader<File>) -> i32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number in input"))
            .collect();
        if nums.len() == 2 {
            left_list.push(nums[0]);
            right_list.push(nums[1]);
        }
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    total_distance
}

fn part2(reader: BufReader<File>) -> i32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number in input"))
            .collect();
        if nums.len() == 2 {
            left_list.push(nums[0]);
            right_list.push(nums[1]);
        }
    }

    // Count occurrences of each number in the right list
    let mut right_counts = HashMap::new();
    for &num in &right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
        .sum();

    similarity_score
}
