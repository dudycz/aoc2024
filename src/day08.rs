use std::io;
use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Pos(usize, usize);

fn parse_map(input: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input).unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_antinodes(antennas: &[Pos], height: usize, width: usize) -> HashSet<Pos> {
    let mut antinodes = HashSet::new();
    for (i, pos1) in antennas.iter().enumerate() {
        for pos2 in antennas.iter().skip(i + 1) {
            let dx = pos2.0 as isize - pos1.0 as isize;
            let dy = pos2.1 as isize - pos1.1 as isize;

            let new_pos1 = Pos((pos1.0 as isize - dx) as usize, (pos1.1 as isize - dy) as usize);
            let new_pos2 = Pos((pos2.0 as isize + dx) as usize, (pos2.1 as isize + dy) as usize);

            if new_pos1.0 < width && new_pos1.1 < height {
                antinodes.insert(new_pos1);
            }
            if new_pos2.0 < width && new_pos2.1 < height {
                antinodes.insert(new_pos2);
            }
        }
    }
    antinodes
}

fn calculate_harmonical_antinodes(antennas: &[Pos], height: usize, width: usize) -> HashSet<Pos> {
    let mut antinodes = antennas.iter().cloned().collect::<HashSet<Pos>>();

    for (i, pos1) in antennas.iter().enumerate() {
        for pos2 in antennas.iter().skip(i + 1) {
            let dx = pos2.0 as isize - pos1.0 as isize;
            let dy = pos2.1 as isize - pos1.1 as isize;

            let mut k = 1;
            loop {
                let new_pos1 = Pos((pos1.0 as isize - k * dx) as usize, (pos1.1 as isize - k * dy) as usize);
                let new_pos2 = Pos((pos2.0 as isize + k * dx) as usize, (pos2.1 as isize + k * dy) as usize);

                let mut added = false;
                if new_pos1.0 < width && new_pos1.1 < height {
                    antinodes.insert(new_pos1);
                    added = true;
                }
                if new_pos2.0 < width && new_pos2.1 < height {
                    antinodes.insert(new_pos2);
                    added = true;
                }

                if !added {
                    break;
                }
                k += 1;
            }
        }
    }
    antinodes
}

fn calc(map: &Vec<Vec<char>>) -> (u64, u64) {
    let mut positions: HashMap<char, Vec<Pos>> = HashMap::new();
    let height = map.len();
    let width = map[0].len();

    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch.is_alphanumeric() {
                positions.entry(ch).or_insert_with(Vec::new).push(Pos(x, y));
            }
        }
    }

    let unique_positions = positions.values()
        .flat_map(|pos_list| calculate_antinodes(pos_list, height, width))
        .collect::<HashSet<_>>();

    let harmonical_positions = positions.values()
        .flat_map(|pos_list| calculate_harmonical_antinodes(pos_list, height, width))
        .collect::<HashSet<_>>();

    (unique_positions.len() as u64, harmonical_positions.len() as u64)
}

pub fn solve(input: &str) -> io::Result<(u64, u64)> {
    let map = parse_map(input);
    let (part1, part2) = calc(&map);

    Ok((part1, part2))
}