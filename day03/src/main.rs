use anyhow::Result;
use std::collections::HashSet;
use std::fs::read_to_string;

#[inline]
fn get_points(c: char) -> u32 {
    if c as u32 > 91 {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn get_score_part1(line: &str) -> u32 {
    let (c1, c2) = line.split_at(line.len() / 2);
    let chars: HashSet<_> = c1.chars().collect();

    c2.chars()
        .filter(|c| chars.contains(c))
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|c| get_points(c))
        .sum()
}

fn get_score_part2(lines: &[&str]) -> u32 {
    get_points(
        lines[0]
            .chars()
            .filter(|c| lines[1].contains(*c))
            .filter(|c| lines[2].contains(*c))
            .collect::<Vec<_>>()[0],
    )
}

fn part1(contents: &str) -> u32 {
    contents.lines().map(|line| get_score_part1(line)).sum()
}

fn part2(contents: &str) -> u32 {
    contents
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| get_score_part2(&lines))
        .sum()
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
    fn test_get_score_part1() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";

        assert_eq!(get_score_part1(&line), 16);
    }

    #[test]
    fn test_example() {
        let sample_data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part1(&sample_data), 157);
        assert_eq!(part2(&sample_data), 70);
    }
}
