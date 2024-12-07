use anyhow::Result;
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Donot,
}

pub fn load_data() -> Result<String> {
    let content = fs::read_to_string("./src/input/03.input")?;
    Ok(content)
}

fn parse_a(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|v| v.extract())
        .map(|(_, [left, right])| {
            let a = left.parse::<i32>().unwrap();
            let b = right.parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

fn parse_b(input: &str) -> Vec<Instruction> {
    let mul_re = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)").unwrap();
    let re = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)|do\(\)|don\'t\(\)").unwrap();

    re.find_iter(input)
        .map(|value| {
            let left = value.as_str();
            match left {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Donot,
                _ => {
                    let (_, [left, right]) = mul_re.captures(left).unwrap().extract();
                    let left = left.parse::<i32>().unwrap();
                    let right = right.parse::<i32>().unwrap();
                    Instruction::Mul(left, right)
                }
            }
        })
        .collect()
}

pub fn part_a() {
    let input = load_data().unwrap();
    let result = parse_a(&input);
    println!("Part a: {:?}", result);
}

pub fn part_b() {
    let input = load_data().unwrap();
    let instructions = parse_b(&input);
    let mut result = 0;
    let mut execute = true;

    for instruction in instructions {
        match instruction {
            Instruction::Mul(left, right) => {
                if !execute {
                    continue;
                }

                result += left * right;
            }
            Instruction::Do => {
                execute = true;
            }
            Instruction::Donot => {
                execute = false;
            }
        }
    }

    println!("Part b: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_b_can_parse_do() {
        let input = "do()";
        let instructions = parse_b(input);

        assert_eq!(instructions, vec![Instruction::Do]);
    }

    #[test]
    fn test_part_b_can_parse_dont() {
        let input = "don't()";
        let instructions = parse_b(input);

        assert_eq!(instructions, vec![Instruction::Donot]);
    }

    #[test]
    fn test_part_b_cases() {
        let input = "mul(2,3)|do()|mul(4,5)|don't()";
        let instructions = parse_b(input);

        assert_eq!(
            instructions,
            vec![
                Instruction::Mul(2, 3),
                Instruction::Do,
                Instruction::Mul(4, 5),
                Instruction::Donot,
            ]
        );
    }

    #[test]
    fn test_part_b_cases_1() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let instructions = parse_b(input);

        assert_eq!(
            instructions,
            vec![
                Instruction::Mul(2, 4),
                Instruction::Donot,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Do,
                Instruction::Mul(8, 5),
            ]
        );
    }
}
