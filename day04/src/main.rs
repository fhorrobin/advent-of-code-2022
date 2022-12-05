use anyhow::Result;
use std::fs::read_to_string;

fn filter_part1(v: &Vec<u32>) -> bool {
    (v[0] <= v[2] && v[1] >= v[3]) || (v[2] <= v[0] && v[3] >= v[1])
}

fn filter_part2(v: &Vec<u32>) -> bool {
    v[0] <= v[3] && v[2] <= v[1]
}

fn compute(contents: &str, filter_fn: fn(&Vec<u32>) -> bool) -> usize {
    contents
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_numeric())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| filter_fn(v))
        .count()
}

fn main() -> Result<()> {
    let contents = read_to_string("input.txt")?;

    println!("Part 1: {}", compute(&contents, filter_part1));
    println!("Part 2: {}", compute(&contents, filter_part2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(compute(&contents, filter_part1), 2);
    }

    #[test]
    fn test_part2() {
        let contents = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(compute(&contents, filter_part2), 4);
    }
}
