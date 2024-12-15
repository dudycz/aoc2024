use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_and_execute(program: &str) -> (i64, i64) {
    let re = Regex::new(r#"(?<mul>mul\((\d{1,3}),(\d{1,3})\))|(?<dont>don't\(\))|(?<do>do\(\))"#)
        .unwrap();
    let mut uncorrupted_result = 0;
    let mut enhanced_result = 0;
    let mut dont = false;

    for cap in re.captures_iter(program) {
        if let Some(_) = cap.name("mul") {
            let x: i64 = cap[2].parse().unwrap();
            let y: i64 = cap[3].parse().unwrap();
            uncorrupted_result += x * y;
            if !dont {
                enhanced_result += x * y;
            }
        } else if cap.name("dont").is_some() {
            dont = true;
        } else if cap.name("do").is_some() {
            dont = false;
        }
    }

    (uncorrupted_result, enhanced_result)
}

pub fn solve(file_path: &str) -> Result<(i64, i64)> {
    let program = read_file(file_path)?;
    Ok(parse_and_execute(&program))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let program = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(parse_and_execute(&program), (161, 48));
    }
}
