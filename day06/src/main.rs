use anyhow::{anyhow, Result};
use itertools::Itertools;

use util::Input;

struct Race {
    time: u64,
    distance: u64,
}

fn extract_numbers(s: &str) -> Vec<u64> {
    s
        .find(":")
        .iter()
        .flat_map(|col_idx| s.get((col_idx+1)..))
        .flat_map(|ns| ns.trim().split_ascii_whitespace())
        .map(|ns| ns.parse::<u64>().unwrap())
        .collect_vec()
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

impl Race {
    fn from_lines(lines: Vec<String>) -> Vec<Race> {
        let mut iter = lines.iter();
        let first_opt = iter.next();
        let second_opt = iter.next();

        match (first_opt, second_opt) {
            (Some(first), Some(second)) => {
                let times = extract_numbers(first);
                let distances = extract_numbers(second);
                times.iter().zip(distances.iter())
                    .map(|(time, distance)| Race { time: *time, distance: *distance })
                    .collect_vec()
            }
            _ => panic!("Malformed input"),
        }
    }

    fn from_input(input: &Input) -> Vec<Race> {
        Race::from_lines(input.as_lines().map(|s| s.to_string()).collect_vec())
    }

    fn from_input_p2(input: &Input) -> Vec<Race> {
        let lines = input.as_lines()
            .map(|line| remove_whitespace(line))
            .collect_vec();
        Race::from_lines(lines)
    }

    fn distance_from_hold_time(&self, ht: u64) -> u64 {
        if ht >= self.time { 0 } else { (self.time - ht) * ht }
    }

    fn win_count(&self) -> u32 {
        (0..self.time).filter(|ht| self.distance_from_hold_time(*ht) > self.distance).count() as u32
    }

}

fn part1(input: &Input) -> Result<u32> {
    let races = Race::from_input(input);
    let res = races.iter().fold(1, |acc, race| acc * race.win_count());
    Ok(res)
}

fn part2(input: &Input) -> Result<u32> {
    let races = Race::from_input_p2(input);

    races.first().map(|r| r.win_count()).ok_or_else(|| anyhow!("No first race"))
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "Time:      7  15   30",
            "Distance:  9  40  200",
        ]);
        assert_eq!(part1(&input).unwrap(), 288);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 505494);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "Time:      7  15   30",
            "Distance:  9  40  200",
        ]);
        assert_eq!(part2(&input).unwrap(), 71503);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 23632299);
        Ok(())
    }
}
