use rustc_hash::{FxHashMap, FxHashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),  // right
    (0, 1),  // down
    (-1, 0), // left
    (0, -1), // up
];

fn parse_input(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        result.push(line.chars().collect());
    }

    Ok(result)
}

pub fn walk_map(
    map: &Vec<Vec<char>>,
    start: (i32, i32),
    mut dir: usize,
    obstacle: Option<(i32, i32)>,
) -> Option<Vec<(i32, i32, usize)>> {
    let mut path = Vec::new();
    let mut current = start;
    let mut visited = FxHashSet::default();
    path.push((current.0, current.1, dir));
    visited.insert((current.0, current.1, dir));

    loop {
        let (dx, dy) = DIRECTIONS[dir];
        let new_x = current.0 + dx;
        let new_y = current.1 + dy;

        // Check if we are out of bounds
        if new_x < 0 || new_y < 0 || new_x >= map.len() as i32 || new_y >= map[0].len() as i32 {
            break;
        }

        // Check if we hit a wall
        if map[new_y as usize][new_x as usize] == '#' || Some((new_x, new_y)) == (obstacle) {
            dir = (dir + 1) % 4;
            continue;
        }

        current = (new_x, new_y);
        if visited.contains(&(current.0, current.1, dir)) {
            return None; // Cycle detected
        }
        path.push((current.0, current.1, dir));
        visited.insert((current.0, current.1, dir));
    }

    Some(path)
}

pub fn starting_point(map: &Vec<Vec<char>>) -> (i32, i32, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            let dir = match ch {
                '>' => 0,
                'v' => 1,
                '<' => 2,
                '^' => 3,
                _ => continue,
            };
            return (x as i32, y as i32, dir);
        }
    }
    panic!("No starting point found");
}

fn part2(map: &Vec<Vec<char>>, path: &Vec<(i32, i32, usize)>) -> u32 {
    let mut counts: FxHashMap<(i32, i32), Vec<usize>> = Default::default();
    for (id, (x, y, _)) in path.iter().enumerate() {
        counts.entry((*x, *y)).or_default().push(id);
    }
    let crossings: FxHashMap<(i32, i32), usize> = counts
        .into_iter()
        .filter(|(_, c)| c.len() > 1)
        .map(|(p, c)| (p, c[0]))
        .collect();

    path.iter()
        .enumerate()
        .filter(|(id, (x, y, _dir))| {
            if *id == 0 {
                return false;
            }
            if let Some(&prev_id) = crossings.get(&(*x, *y)) {
                if prev_id != *id {
                    return false;
                }
            }
            let (start_x, start_y, start_dir) = path[id - 1];
            walk_map(map, (start_x, start_y), start_dir, Some((*x, *y))).is_none()
        })
        .count() as u32
}

pub fn solve(input: &str) -> io::Result<(u32, u32)> {
    let map = parse_input(input)?;
    let (x, y, dir) = starting_point(&map.clone());
    let mut path = walk_map(&map, (x, y), dir, None).unwrap();
    let obstacles = part2(&map, &path);

    path.sort();
    path.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    Ok((path.len() as u32, obstacles))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_map() {
        let map = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];
        let start = starting_point(&map);
        let path = walk_map(&map, (start.0, start.1), start.2, None).unwrap();
        let obstacles = part2(&map, &path);

        assert_eq!(obstacles, 6);
    }
}
