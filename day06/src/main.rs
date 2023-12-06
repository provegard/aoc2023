use anyhow::Result;
use itertools::Itertools;

use util::Input;

struct Race {
    time: u32,
    distance: u32,
}

fn extract_numbers(s: &str) -> Vec<u32> {
    s
        .find(":")
        .iter()
        .flat_map(|col_idx| s.get((col_idx+1)..))
        .flat_map(|ns| ns.trim().split_ascii_whitespace())
        .map(|ns| ns.parse::<u32>().unwrap())
        .collect_vec()
}

impl Race {
    fn from_input(input: &Input) -> Vec<Race> {
        let mut iter = input.as_lines();
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

    fn distance_from_hold_time(&self, ht: u32) -> u32 {
        if ht >= self.time { 0 } else { (self.time - ht) * ht }
    }

    fn is_win(&self, distance: u32) -> bool {
        distance > self.distance
    }

    fn win_times(&self) -> Vec<u32> {
        (0..self.time).filter(|ht| self.is_win(self.distance_from_hold_time(*ht))).collect_vec()
    }

}

fn part1(input: &Input) -> Result<u32> {
    let races = Race::from_input(input);
    let res = races.iter().fold(1, |acc, race| {
        acc * race.win_times().len() as u32
    });
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

    // #[test]
    // pub fn test_part2() -> Result<()> {
    //     let input = Input::from_lines([
    //     ]);
    //     assert_eq!(part2(&input).unwrap(), 0);
    //     Ok(())
    // }
}
