use anyhow::Result;
use std::fs::read_to_string;

#[inline]
fn get_ind(c: u8) -> usize {
    if c > 91 {
        c as usize - 97
    } else {
        c as usize - 39
    }
}

fn get_score_part1(line: &str) -> u32 {
    let (c1, c2) = line.split_at(line.len() / 2);
    let mut bitmap_c1 = [false; 52];
    let mut bitmap_c2 = [false; 52];

    c1.as_bytes()
        .iter()
        .for_each(|&c| bitmap_c1[get_ind(c)] |= true);
    c2.as_bytes()
        .iter()
        .for_each(|&c| bitmap_c2[get_ind(c)] |= bitmap_c1[get_ind(c)]);

    bitmap_c2
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &v)| acc + (i as u32 + 1) * v as u32)
}

fn get_score_part2(lines: &[&str]) -> u32 {
    (get_ind(
        lines[0]
            .chars()
            .filter(|&c| lines[1].contains(c) && lines[2].contains(c))
            .next()
            .unwrap() as u8,
    ) + 1) as u32
}

fn part2(contents: &str) -> u32 {
    contents
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| get_score_part2(&lines))
        .sum()
}

fn part1(contents: &str) -> u32 {
    contents.lines().map(|line| get_score_part1(line)).sum()
}

fn main() -> Result<()> {
    let contents = read_to_string("input.txt")?;

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part1(&contents), 157);
        assert_eq!(part2(&contents), 70);
    }
}
