use anyhow::Result;
use itertools::Itertools;

use util::Input;

fn parse_line(line: &str) -> Vec<i32> {
    let v = line.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect_vec();
    v
}

fn find_diffs(nums: &Vec<i32>) -> Vec<i32> {
    let v = nums.iter().zip(nums.iter().skip(1)).map(|(a, b)| b - a).collect_vec();
    v
}

fn next_num(nums: &Vec<i32>) -> i32 {
    let diffs = find_diffs(nums);
    if diffs.iter().all(|n| *n == 0) {
        *nums.last().unwrap()
    } else {
        nums.last().unwrap() + next_num(&diffs)
    }
}

fn part1(input: &Input) -> Result<i32> {
    let vecs = input.as_lines().map(|line| parse_line(line)).collect_vec();
    let res: i32 = vecs.iter().map(|v| next_num(v)).sum();
    Ok(res)
}

fn part2(input: &Input) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "0 3 6 9 12 15",
            "1 3 6 10 15 21",
            "10 13 16 21 30 45",
        ]);
        assert_eq!(part1(&input).unwrap(), 114);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 1681758908);
        Ok(())
    }

    // #[test]
    // pub fn test_part2() -> Result<()> {
    //     let input = Input::from_lines([
    //     ]);
    //     assert_eq!(part2(&input).unwrap(), 0);
    //     Ok(())
    // }
}
