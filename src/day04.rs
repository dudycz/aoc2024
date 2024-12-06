use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    Ok(grid)
}

fn find_word(grid: &[Vec<char>], word: &str) -> usize {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                if (0..word_len).all(|i| {
                    let x = r as isize + dr * i as isize;
                    let y = c as isize + dc * i as isize;
                    x >= 0 && x < rows as isize && y >= 0 && y < cols as isize && grid[x as usize][y as usize] == word_chars[i]
                }) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find_x_shaped_mas(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            // Check if the center is 'A'
            if grid[r][c] == 'A' {
                // Get diagonal values
                let tl = grid[r - 1][c - 1];
                let tr = grid[r - 1][c + 1];
                let bl = grid[r + 1][c - 1];
                let br = grid[r + 1][c + 1];

                // Check for MAS or SAM on both diagonals
                let valid_tl_br = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
                let valid_tr_bl = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

                // X is valid only if both diagonals are valid
                if valid_tl_br && valid_tr_bl {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn solve(file_path: &str) -> io::Result<(usize, usize)> {
    let grid = load_grid_from_file(file_path)?;
    let xmas_count = find_word(&grid, "XMAS");
    let samx_count = find_word(&grid, "SAMX");
    let mas = find_x_shaped_mas(&grid);
    Ok((xmas_count+samx_count, mas))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_horizontal() {
        let grid = vec![
            vec!['x', 'm', 'a', 's', 'a', 'b'],
            vec!['a', 'b', 'c', 'd', 'e', 'f'],
            vec!['.', 'x', 'm', 'a', 's', 'a'],
            vec!['x', '.', 'm', 'a', 's', 'a'],            
        ];
        let word = "xmas";
        let result = find_word(&grid, word);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_find_word_vertical() {
        let grid = vec![
            vec!['.', '.'],
            vec!['x', 'a'],
            vec!['m', 'b'],
            vec!['a', 'c'],
            vec!['s', 'd'],
        ];
        let word = "xmas";
        let result = find_word(&grid, word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_word_diagonal_down_right() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', 'x', '.', '.', '.', '.'],
            vec!['.', '.', 'm', '.', '.', '.'],
            vec!['.', '.', '.', 'a', '.', '.'],
            vec!['.', '.', '.', '.', 's', '.'],
            vec!['.', '.', '.', '.', '.', 'a'],
        ];
        let word = "xmas";
        let result = find_word(&grid, word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_word_diagonal_down_left() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'x', '.'],
            vec!['.', '.', '.', 'm', '.', '.'],
            vec!['.', '.', 'a', '.', '.', '.'],
            vec!['.', 's', '.', '.', '.', '.'],
            vec!['a', '.', '.', '.', '.', '.'],
        ];
        let word = "xmas";
        let result = find_word(&grid, word);
        assert_eq!(result, 1);
    }
    
    #[test]
    fn test_find_word() {
        let grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];

        let result = find_word(&grid, "XMAS") + find_word(&grid, "SAMX");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_find_x_crossed_word() {
        let grid = vec![
            vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
            vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let result = find_x_shaped_mas(&grid);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_find_x_crossed_possibilities() {
        let grid = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];

        let result = find_x_shaped_mas(&grid);
        assert_eq!(result, 0);
    }
}
