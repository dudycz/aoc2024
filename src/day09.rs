use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Block {
    file_id: Option<u16>, // None - free block, Some - file id
    size: u16,
}

// Skip even blocks if size is 0
fn decode_layout(map: &str) -> Vec<Block> {
    map.chars()
        .enumerate()
        .filter_map(|(i, c)| {
            c.to_digit(10)
                .map(|size| Block {
                    file_id: if i % 2 == 0 {
                        Some((i / 2) as u16)
                    } else {
                        None
                    },
                    size: size as u16,
                })
                .filter(|block| i % 2 == 0 || block.size != 0)
        })
        .collect()
}

fn checksum(blocks: &Vec<Block>) -> u64 {
    let mut crc = 0;
    let mut pos = 0;
    for iter in blocks.iter() {
        let file_id = iter.file_id.unwrap();
        for _ in 0..iter.size {
            crc += file_id as u64 * pos;
            pos += 1;
        }
    }
    crc
}

fn compress_disk(map: &str) -> u64 {
    let mut blocks = decode_layout(map);
    let mut free_blocks_count = blocks
        .iter()
        .filter(|block| block.file_id.is_none())
        .count();
    let mut first_free_pos = 0;

    while free_blocks_count > 0 {
        let mut last = blocks.pop().unwrap();

        if last.file_id.is_none() {
            free_blocks_count -= 1;
            continue;
        }

        while last.size > 0 {
            let (pos, free_block) = match blocks
                .iter_mut()
                .enumerate()
                .skip(first_free_pos)
                .find(|(_, block)| block.file_id.is_none())
            {
                Some((pos, block)) => (pos, block),
                None => {
                    blocks.push(last);
                    break;
                }
            };

            if last.size >= free_block.size {
                free_block.file_id = last.file_id;
                last.size -= free_block.size;
                free_blocks_count -= 1;
            } else {
                let remaining_free_space = free_block.size - last.size;
                free_block.size = last.size;
                free_block.file_id = last.file_id;
                last.size = 0;

                blocks.insert(
                    pos + 1,
                    Block {
                        file_id: None,
                        size: remaining_free_space,
                    },
                );
            }
            first_free_pos = pos;
        }
    }
    checksum(&blocks)
}

pub fn solve(input: &str) -> io::Result<(u64, u64)> {
    let mut file = File::open(input)?;
    let mut disk_map = String::new();
    file.read_to_string(&mut disk_map)?;

    let part1 = compress_disk(&disk_map);

    Ok((part1, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_disk() {
        let map = "2333133121414131402";
        assert_eq!(compress_disk(map), 1928);
    }
}
