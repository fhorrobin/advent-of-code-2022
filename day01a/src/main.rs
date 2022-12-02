use anyhow::Result;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn part1(contents: &str) -> Result<i64> {
    let max_sum: i64 = contents
        .split("\n\n")
        .map(|e| e.split("\n").map(|v| v.parse::<i64>().unwrap_or(0)).sum())
        .max()
        .unwrap_or(0);

    Ok(max_sum)
}

fn part2(contents: &str) -> Result<i64> {
    let mut heap = BinaryHeap::with_capacity(4);

    contents.split("\n\n").for_each(|elem| {
        let s: i64 = elem
            .split("\n")
            .map(|v| v.parse::<i64>().unwrap_or(0))
            .sum();
            
        heap.push(-s);

        if heap.len() > 3 {
            heap.pop();
        }
    });

    Ok(heap.iter().fold(0, |a, val| a + -val))
}

fn main() -> Result<()> {
    let contents = read_to_string("input.txt")?;

    println!("Part 1: {}", part1(&contents)?);
    println!("Part 2: {}", part2(&contents)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sample_data = "
10\n20\n30\n\n
10\n10\n\n
5\n20\n30\n10\n\n
10\n5
        ";

        assert_eq!(part1(&sample_data).unwrap(), 65);
    }

    #[test]
    fn test_part2() {
        let sample_data = "
10\n20\n30\n\n
10\n10\n\n
5\n20\n30\n10\n\n
10\n5\n\n
15\n5\n5
        ";

    assert_eq!(part2(&sample_data).unwrap(), 150);
    }
}