use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};

type Level = Vec<u32>;
type Reports = Vec<Level>;

fn read_reports_from_file(file_path: &str) -> Result<Reports> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut reports = Reports::new();

    for line in reader.lines() {
        let line = line?;
        let levels = line
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Level, _>>()?;
        reports.push(levels);
    }

    Ok(reports)
}

fn is_safe_report(levels: &[u32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..levels.len() {
        let diff = (levels[i] as i32 - levels[i - 1] as i32).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if levels[i] > levels[i - 1] {
            is_decreasing = false;
        } else if levels[i] < levels[i - 1] {
            is_increasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn can_be_safe_by_removing_one(levels: &[u32]) -> bool {
    for i in 0..levels.len() {
        let mut temp_levels = levels.to_vec();
        temp_levels.remove(i);
        if is_safe_report(&temp_levels) {
            return true;
        }
    }
    false
}

fn count_safe_reports(reports: &Reports) -> (i32, i32) {
    let mut safe_count = 0;
    let mut unsafe_tolerance_count = 0;

    for levels in reports {
        if is_safe_report(levels) {
            safe_count += 1;
        } else if can_be_safe_by_removing_one(levels) {
            unsafe_tolerance_count += 1;
        }
    }

    (safe_count, unsafe_tolerance_count)
}

pub fn solve(file_path: &str) -> Result<(i32, i32)> {
    let reports = read_reports_from_file(file_path)?;
    let (safe, toleranced) = count_safe_reports(&reports);
    Ok((safe, safe + toleranced))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_report() {
        assert!(is_safe_report(&[7, 6, 4, 2, 1])); // safe (decreasing)
        assert!(!is_safe_report(&[1, 2, 7, 8, 9])); // not safe (difference > 3)
        assert!(!is_safe_report(&[9, 7, 6, 2, 1])); // not safe (difference > 3)
        assert!(!is_safe_report(&[1, 3, 2, 4, 5])); // not safe (not all increasing or decreasing)
        assert!(!is_safe_report(&[8, 6, 4, 4, 1])); // not safe (difference < 1)
        assert!(is_safe_report(&[1, 3, 6, 7, 9])); // safe (increasing)
        assert!(is_safe_report(&[1])); // safe (single element)
        assert!(is_safe_report(&[])); // safe (empty)
    }

    #[test]
    fn test_can_be_safe_by_removing_one() {
        assert!(!can_be_safe_by_removing_one(&[1, 2, 7, 8, 9])); // not safe
        assert!(!can_be_safe_by_removing_one(&[9, 7, 6, 2, 1])); // not safe
        assert!(can_be_safe_by_removing_one(&[1, 3, 2, 4, 5])); // becomes safe by removing 2
        assert!(can_be_safe_by_removing_one(&[8, 6, 4, 4, 1])); // can be safe by removing 4
    }

    #[test]
    fn test_count_safe_reports() {
        let reports = vec![
            vec![7, 6, 4, 2, 1], // safe (decreasing)
            vec![1, 2, 7, 8, 9], // not safe (difference > 3)
            vec![9, 7, 6, 2, 1], // not safe (difference > 3)
            vec![1, 3, 2, 4, 5], // not safe (not all increasing or decreasing)
            vec![8, 6, 4, 4, 1], // not safe (difference < 1)
            vec![1, 3, 6, 7, 9], // safe (increasing)
        ];
        assert_eq!(count_safe_reports(&reports), (2, 2));
    }
}
