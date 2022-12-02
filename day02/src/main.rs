use anyhow::{Result};
use std::fs::read_to_string;

const SCORE_TABLE_PART_1: [u32; 9] = [4, 8, 3, 1, 5, 9, 7, 2, 6];
const SCORE_TABLE_PART_2: [u32; 9] = [3, 4, 8, 1, 5, 9, 2, 6, 7];

fn solve(contents: &str, score_table: &[u32]) -> u32 {
    contents.split("\n")
        .filter(|line| line.len() >= 3)
        .map(|line| 3 * (line.as_bytes()[0] - 'A' as u8) + line.as_bytes()[2] - 'X' as u8)
        .map(|i| score_table[i as usize]).sum()
}

fn main() -> Result<()>{
    let contents = read_to_string("input.txt")?;
    
    println!("Part 1: {}", solve(&contents, &SCORE_TABLE_PART_1));
    println!("Part 2: {}", solve(&contents, &SCORE_TABLE_PART_2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sample_data = "
A Y
B X
C Z
";
    
        assert_eq!(solve(&sample_data, &SCORE_TABLE_PART_1), 15);
    }

    #[test]
    fn test_part2() {
        let sample_data = "
A Y
B X
C Z
";
    
        assert_eq!(solve(&sample_data, &SCORE_TABLE_PART_2), 12);
    }
}
