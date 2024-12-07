use anyhow::Result;
use std::{cmp::Ordering, fs, num::ParseIntError, ops::Deref};

#[derive(Debug, Clone)]
struct Rules(Vec<(i32, i32)>);

impl Deref for Rules {
    type Target = Vec<(i32, i32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct PageUpdates(Vec<Vec<i32>>);

fn load_data() -> Result<(Rules, PageUpdates)> {
    let content = fs::read_to_string("./src/input/05.input")?;
    let mut rules = Vec::new();
    let mut page_updates = Vec::new();

    let mut add_rule = true;

    for line in content.lines() {
        if line.trim().is_empty() {
            add_rule = false;
            continue;
        }

        if add_rule {
            let parts = line.split('|').collect::<Vec<&str>>();
            let left = parts[0].parse::<i32>()?;
            let right = parts[1].parse::<i32>()?;
            rules.push((left, right));
        } else {
            let updates = line
                .split(',')
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()?;
            page_updates.push(updates);
        }
    }

    Ok((Rules(rules), PageUpdates(page_updates)))
}

fn generate_pairs(input: &[i32]) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            pairs.push((input[i], input[j]));
        }
    }

    pairs
}

pub fn part_a() {
    let (rules, updates) = load_data().unwrap();
    let mut mid_points = 0;

    for update in updates.0.into_iter() {
        let pairs = generate_pairs(&update);
        let is_valid = pairs
            .iter()
            .all(|pair| rules.iter().any(|rule| pair == rule));

        if is_valid {
            let index = update.len() / 2;
            mid_points += update[index];
        }
    }

    println!("Part a: {mid_points}");
}

pub fn part_b() {
    let (rules, updates) = load_data().unwrap();
    let mut invalid_list = Vec::new();

    for update in updates.0.into_iter() {
        let pairs = generate_pairs(&update);
        let is_valid = pairs
            .iter()
            .all(|pair| rules.iter().any(|rule| pair == rule));

        if !is_valid {
            invalid_list.push(update);
        }
    }

    let mut mid_points = 0;
    for mut invalid in invalid_list {
        invalid.sort_by(|left, right| {
            let rule = rules.0.iter().any(|(l, r)| l == left && r == right);
            if rule {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        let index = invalid.len() / 2;
        mid_points += invalid[index];
    }

    println!("Part b: {mid_points}");
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_part_b() {
        let (rules, _updates) = load_data().unwrap();
        let mut invalid = Vec::from_iter([97, 13, 75, 29, 47]);
        println!("Before:  {:?}", invalid);

        invalid.sort_by(|left, right| {
            let rule = rules.0.iter().any(|(l, r)| l == left && r == right);
            println!("({:?}, {:?}, {rule})", left, right);
            if rule {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        println!("After: {:?}", invalid);
    }
}
