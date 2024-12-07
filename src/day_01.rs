use anyhow::Result;
use std::{collections::HashMap, fs};

pub fn load_data() -> Result<(Vec<i32>, Vec<i32>)> {
    let content = fs::read_to_string("./src/input/01.input")?;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse()?);
        right.push(parts[1].parse()?);
    }

    Ok((left, right))
}

pub fn part_a() {
    let (mut left, mut right) = load_data().unwrap();
    left.sort();
    right.sort();

    let result: i32 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("Part a: {result}");
}

pub fn part_b() {
    let (left, right) = load_data().unwrap();
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for value in right {
        *counts.entry(value).or_insert(0) += 1;
    }

    let result: i32 = left
        .into_iter()
        .map(|val| val * counts.get(&val).unwrap_or(&0))
        .sum();

    println!("Part b: {result}");
}
