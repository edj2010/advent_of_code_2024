use advent_of_code::parse::{parsers, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct File {
    id: u64,
    location: u64,
    length: u64,
}

impl File {
    fn checksum(&self) -> u64 {
        (self.id * self.length * (2 * self.location + self.length - 1)) / 2
    }

    fn take(&mut self, length: u64, location: u64) -> Self {
        if self.length > length {
            self.length -= length;
            File {
                id: self.id,
                location,
                length,
            }
        } else {
            let old_length = self.length;
            self.length = 0;
            File {
                id: self.id,
                location,
                length: old_length,
            }
        }
    }

    fn end_location(&self) -> u64 {
        self.location + self.length
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct EmptyBlock {
    location: u64,
    length: u64,
}

#[derive(Debug)]
struct Disk {
    files: Vec<File>,
    empty_blocks: Vec<EmptyBlock>,
}

impl Disk {
    fn new() -> Self {
        Disk {
            files: Vec::new(),
            empty_blocks: Vec::new(),
        }
    }

    fn append_file(&mut self, location: u64, length: u64) {
        let (id, last_end) = self
            .files
            .last()
            .map(|f| (f.id + 1, f.end_location()))
            .unwrap_or((0, 0));
        self.files.push(File {
            id,
            location,
            length,
        });
        if location != last_end {
            self.empty_blocks.push(EmptyBlock {
                location: last_end,
                length: location - last_end,
            });
        }
    }
}

fn parse(input: &str) -> Disk {
    parsers::chars(|c| c.is_numeric())
        .map(|d| d.to_digit(10).unwrap() as u64)
        .many()
        .skip_tag("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .fold(
            (Disk::new(), 0, true),
            |(mut disk, end_idx, is_file), length| {
                if is_file {
                    disk.append_file(end_idx, length);
                    (disk, end_idx + length, false)
                } else {
                    (disk, end_idx + length, true)
                }
            },
        )
        .0
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u64 {
    let mut disk = parse(input);
    let mut checksum = 0;
    let mut front_file_idx = 0_usize;
    let mut back_file_idx = disk.files.len() - 1;
    let mut disk_location = 0;
    while disk_location < disk.files[back_file_idx].end_location()
        && front_file_idx <= back_file_idx
    {
        if disk_location < disk.files[front_file_idx].location {
            let empty_space = disk.files[front_file_idx].location - disk_location;
            while disk.files[back_file_idx].length == 0 {
                back_file_idx -= 1;
            }
            if disk.files[back_file_idx].location < disk_location {
                break;
            }
            let new_file = disk.files[back_file_idx].take(empty_space, disk_location);
            disk_location += new_file.length;
            checksum += new_file.checksum();
        } else if disk_location == disk.files[front_file_idx].location {
            disk_location += disk.files[front_file_idx].length;
            checksum += disk.files[front_file_idx].checksum();
            front_file_idx += 1;
        } else {
            panic!(
                "invalid state, disk location is {} and front file idx is {} ",
                disk_location, front_file_idx
            )
        }
    }
    checksum
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    let mut disk = parse(input);
    for file in disk.files.iter_mut().rev() {
        for empty_block in disk.empty_blocks.iter_mut() {
            if empty_block.length >= file.length && empty_block.location < file.location {
                file.location = empty_block.location;
                empty_block.location += file.length;
                empty_block.length -= file.length;
            }
        }
    }
    disk.files.into_iter().map(|f| f.checksum()).sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "2333133121414131402
";
    const DAY: Day = Day::Day09;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2858);
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            6366665108136
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            6398065450842
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| {
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }
}
