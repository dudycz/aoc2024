use itertools::Itertools;
use rayon::prelude::*;
use std::fs;
use std::io;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn parse_input(input: &str) -> io::Result<Vec<(u64, Vec<u32>)>> {
    fs::read_to_string(input)?
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let result_number = parts
                .next()
                .unwrap()
                .trim()
                .parse::<u64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| {
                    n.parse::<u32>()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok((result_number, numbers))
        })
        .collect()
}

fn check_equation(result: u64, numbers: &Vec<u32>, ops: &Vec<Operator>) -> bool {
    let op_num = numbers.len() - 1;
    std::iter::repeat(ops)
        .take(op_num)
        .multi_cartesian_product()
        .par_bridge()
        .any(|perm| {
            let mut current = numbers[0] as u64;
            for (i, &op) in perm.iter().enumerate() {
                match op {
                    Operator::Add => current += numbers[i + 1] as u64,
                    Operator::Mul => current *= numbers[i + 1] as u64,
                    Operator::Concat => {
                        current = format!("{}{}", current, numbers[i + 1])
                            .parse::<u64>()
                            .unwrap()
                    }
                }
            }
            current == result
        })
}

pub fn solve(input: &str) -> io::Result<(u64, u64)> {
    let part1_ops = vec![Operator::Add, Operator::Mul];
    let part2_ops = vec![Operator::Add, Operator::Mul, Operator::Concat];
    let calibrations = parse_input(input)?;
    let (part1, part2): (u64, u64) = calibrations
        .par_iter()
        .map(|(result, numbers)| {
            let mut p1 = 0;
            let mut p2 = 0;
            if check_equation(*result, numbers, &part1_ops) {
                p1 = *result;
            }
            if check_equation(*result, numbers, &part2_ops) {
                p2 = *result;
            }
            (p1, p2)
        })
        .reduce(|| (0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2));

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_equation() {
        let ops = vec![Operator::Add, Operator::Mul, Operator::Concat];
        assert_eq!(check_equation(190, &vec![10, 19], &ops), true);
        assert_eq!(check_equation(3267, &vec![81, 40, 27], &ops), true);
        assert_eq!(check_equation(83, &vec![17, 5], &ops), false);
        assert_eq!(check_equation(156, &vec![15, 6], &ops), true);
        assert_eq!(check_equation(7290, &vec![6, 8, 6, 15], &ops), true);
        assert_eq!(check_equation(161011, &vec![16, 10, 13], &ops), false);
        assert_eq!(check_equation(192, &vec![17, 8, 14], &ops), true);
        assert_eq!(check_equation(21037, &vec![9, 7, 18, 13], &ops), false);
        assert_eq!(check_equation(292, &vec![11, 6, 16, 20], &ops), true);
    }
}
