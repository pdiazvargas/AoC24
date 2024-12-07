use anyhow::Result;
use std::fs;

pub struct Reports(Vec<i32>);

impl Reports {
    pub fn is_valid_part_a(&self) -> bool {
        let mut diffs: Vec<i32> = Vec::with_capacity(self.0.len() - 1);

        for index in 1..self.0.len() {
            let prev = self.0[index - 1];
            let current = self.0[index];
            diffs.push(current - prev);
        }

        let is_increasing = diffs.iter().all(|v| *v > 0);
        let is_decreasing = diffs.iter().all(|v| *v < 0);
        let within_bounds = diffs.iter().map(|v| v.abs()).all(|v| v >= 1 && v <= 3);

        (is_increasing || is_decreasing) && within_bounds
    }

    /// Tring to be smart and find the faulty element and removing it taking into account
    /// that the index of the faulty element is either the on that we find in the diff array
    /// or the next one.
    pub fn is_valid_part_b(&self) -> bool {
        let mut diffs: Vec<i32> = Vec::with_capacity(self.0.len() - 1);

        for index in 1..self.0.len() {
            let prev = self.0[index - 1];
            let current = self.0[index];
            diffs.push(current - prev);
        }

        let is_increasing = diffs.iter().all(|v| *v > 0);
        let is_decreasing = diffs.iter().all(|v| *v < 0);
        let within_bounds = diffs.iter().map(|v| v.abs()).all(|v| v >= 1 && v <= 3);

        let is_valid = (is_increasing || is_decreasing) && within_bounds;
        if is_valid {
            true
        } else {
            let count_positive = diffs.iter().filter(|v| **v > 0).count();
            let count_negative = diffs.iter().filter(|v| **v < 0).count();
            let count_zero = diffs.iter().filter(|v| **v == 0).count();
            let count_out_of_bounds = diffs.iter().map(|v| v.abs()).filter(|v| *v > 3).count();

            let pad: Option<usize> = if count_negative == 1 {
                Some(diffs.iter().position(|v| *v < 0).unwrap())
            } else if count_positive == 1 {
                Some(diffs.iter().position(|v| *v > 0).unwrap())
            } else if count_zero == 1 {
                Some(diffs.iter().position(|v| *v == 0).unwrap())
            } else if count_out_of_bounds == 1 {
                Some(diffs.iter().position(|v| *v < 1 || *v > 3).unwrap())
            } else {
                None
            };

            if let Some(index) = pad {
                let mut raw1 = self.0.clone();
                let mut raw2 = self.0.clone();
                raw1.remove(index);
                raw2.remove(index + 1);
                Reports(raw1).is_valid_part_a() || Reports(raw2).is_valid_part_a()
            } else {
                false
            }
        }
    }

    pub fn is_valid_part_b_brute(&self) -> bool {
        if self.is_valid_part_a() {
            true
        } else {
            for index in 0..self.0.len() {
                let mut raw = self.0.clone();
                raw.remove(index);
                if Reports(raw).is_valid_part_a() {
                    return true;
                }
            }

            false
        }
    }
}

pub fn load_data() -> Result<Vec<Reports>> {
    let content = fs::read_to_string("./src/input/02.input")?;
    let mut reports: Vec<Reports> = Vec::new();

    for line in content.lines() {
        let parts: Vec<i32> = line.split(" ").map(|v| v.parse().unwrap()).collect();
        reports.push(Reports(parts));
    }

    Ok(reports)
}

pub fn part_a() {
    let reports = load_data().unwrap();
    let safe_reports: Vec<&Reports> = reports.iter().filter(|r| r.is_valid_part_a()).collect();
    println!("Part a: {:?}", safe_reports.len());
}

pub fn part_b() {
    let reports = load_data().unwrap();
    let safe_reports: Vec<&Reports> = reports
        .iter()
        .filter(|r| r.is_valid_part_b_brute())
        .collect();
    println!("Part b: {:?}", safe_reports.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid_part_b() {
        let reports = Reports(vec![7, 6, 4, 2, 1]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_type_1_repair() {
        // removing 3 will make this report valid
        let reports = Reports(vec![1, 3, 2, 4, 5]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_type_2_repair() {
        // removing 4 will make this report valid
        let reports = Reports(vec![8, 6, 4, 4, 1]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_start_repair() {
        // Removing first element will make this valid
        let reports = Reports(vec![1, 6, 4, 2, 1]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_end_repair() {
        // Removing last element will make this valid
        let reports = Reports(vec![8, 6, 4, 2, 4]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_mid_repair() {
        // Removing last 6 will make this valid
        let reports = Reports(vec![8, 6, 4, 6, 1]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_usafe_reports() {
        // Removing last element will make this valid
        let reports = Reports(vec![1, 2, 7, 8, 9]);
        assert_eq!(reports.is_valid_part_b(), false);

        let reports = Reports(vec![9, 7, 6, 2, 1]);
        assert_eq!(reports.is_valid_part_b(), false);
    }

    #[test]
    fn test_part_b_type_3_repair_end() {
        // Removing last element will make this valid
        // diffs: [1, 1, 1, 1, 2, 1, 4]
        let reports = Reports(vec![19, 20, 21, 22, 23, 25, 26, 30]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_type_3_repair_start() {
        // Removing first element will make this valid
        let reports = Reports(vec![30, 26, 25, 23, 22, 21, 20, 19]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_type_3_repair_mid() {
        // Removing first element will make this valid
        let reports = Reports(vec![26, 25, 23, 30, 22, 21, 20, 19]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_type_3_repair_other() {
        // Removing first element will make this valid
        let reports = Reports(vec![26, 25, 23, 22, 21, 20, 30, 19]);
        assert_eq!(reports.is_valid_part_b(), true);
    }

    #[test]
    fn test_part_b_cases() {
        let reports = [
            Reports(vec![48, 46, 47, 49, 51, 54, 56]).is_valid_part_b(),
            Reports(vec![1, 1, 2, 3, 4, 5]).is_valid_part_b(),
            Reports(vec![1, 2, 3, 4, 5, 5]).is_valid_part_b(),
            Reports(vec![5, 1, 2, 3, 4, 5]).is_valid_part_b(),
            Reports(vec![1, 4, 3, 2, 1]).is_valid_part_b(),
            Reports(vec![1, 6, 7, 8, 9]).is_valid_part_b(),
            Reports(vec![1, 2, 3, 4, 3]).is_valid_part_b(),
            Reports(vec![9, 8, 7, 6, 7]).is_valid_part_b(),
            Reports(vec![7, 10, 8, 10, 11]).is_valid_part_b(),
            Reports(vec![29, 28, 27, 25, 26, 25, 22, 20]).is_valid_part_b(),
            Reports(vec![7, 10, 8, 10, 11]).is_valid_part_b(),
            Reports(vec![29, 28, 27, 25, 26, 25, 22, 20]).is_valid_part_b(),
            Reports(vec![8, 9, 10, 11]).is_valid_part_b(),
            Reports(vec![1, 2, 3, 4, 5, 5]).is_valid_part_b(),
        ];

        assert!(reports.iter().all(|b| *b == true));
    }
}
