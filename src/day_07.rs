use anyhow::Result;
use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
struct Equation {
    pub target: u128,
    pub values: Vec<u128>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(":").collect();
        let target = parts[0].parse()?;
        let values = parts[1]
            .trim()
            .split(" ")
            .into_iter()
            .map(|v| v.parse().unwrap())
            .collect();

        Ok(Equation { target, values })
    }
}

fn evaluate(equation: &Equation, index: usize, current: u128) -> bool {
    if current == equation.target && index == equation.values.len() {
        return true;
    }

    if index == equation.values.len() {
        return false;
    }

    let value = equation.values[index];
    evaluate(equation, index + 1, current + value)
        || evaluate(equation, index + 1, current * value)
        || evaluate(equation, index + 1, merge(current, value))
}

fn merge(left: u128, right: u128) -> u128 {
    format!("{left}{right}").parse().unwrap()
}

fn load_data() -> Result<Vec<Equation>> {
    let content = fs::read_to_string("./src/input/07.input")?;

    let equations = content
        .lines()
        .map(Equation::from_str)
        .collect::<Result<Vec<Equation>>>()?;

    Ok(equations)
}

pub fn part_a() {
    let equations = load_data().unwrap();

    let total: u128 = equations
        .iter()
        .filter(|eq| evaluate(eq, 0, 0))
        .map(|eq| eq.target)
        .sum();

    println!("Part a: {total}");
}

pub fn part_b() {
    let equations = load_data().unwrap();

    let total: u128 = equations
        .iter()
        .filter(|eq| evaluate(eq, 0, 0))
        .map(|eq| eq.target)
        .sum();

    println!("Part b: {total}");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_equations() {
        let equations = load_data().unwrap();

        let total: u128 = equations
            .iter()
            .filter(|eq| evaluate(eq, 0, 0))
            .map(|eq| eq.target)
            .sum();

        println!("Total: {}", total);
    }

    #[test]
    fn test_merge() {
        assert_eq!(merge(10, 19), 1019);
        assert_eq!(merge(11 * 6 * 16, 20), 105620);
    }

    #[test]
    fn one_instance() {
        let equation = Equation {
            target: 8396187,
            values: Vec::from([7, 904, 477, 3, 148, 87, 2]),
        };

        let result = evaluate(&equation, 0, 0);
        assert_eq!(result, false);
    }
}
