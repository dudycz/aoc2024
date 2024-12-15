use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};

struct Columns {
    col1: Vec<i32>,
    col2: Vec<i32>,
}

fn read_columns_from_file(file_path: &str) -> Result<Columns> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers = line.split_whitespace();
        let num1: i32 = numbers
            .next()
            .ok_or_else(|| anyhow!("Invalid line format"))?
            .parse()?;
        let num2: i32 = numbers
            .next()
            .ok_or_else(|| anyhow!("Invalid line format"))?
            .parse()?;
        col1.push(num1);
        col2.push(num2);
    }

    Ok(Columns { col1, col2 })
}

fn calculate_metrics(columns: &Columns) -> (i32, i32) {
    let mut col1 = columns.col1.clone();
    let mut col2 = columns.col2.clone();

    col1.sort_unstable();
    col2.sort_unstable();

    let distance: i32 = col1
        .iter()
        .zip(col2.iter())
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum();

    let similarity_score: i32 = col1
        .iter()
        .map(|&num1| num1 * col2.iter().filter(|&&num2| num2 == num1).count() as i32)
        .sum();

    (distance, similarity_score)
}

pub fn solve(file_path: &str) -> Result<(i32, i32)> {
    let columns = read_columns_from_file(file_path)?;
    Ok(calculate_metrics(&columns))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_metrics() {
        let columns = Columns {
            col1: vec![3, 4, 2, 1, 3, 3],
            col2: vec![4, 3, 5, 3, 9, 3],
        };
        let (distance, similarity_score) = calculate_metrics(&columns);
        assert_eq!(distance, 11);
        assert_eq!(similarity_score, 31);
    }
}
