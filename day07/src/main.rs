use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;

use util::Input;

struct HandBid {
    hand: String,
    bid: u32
}

fn char_value(ch: &char) -> i8 {
    match ch.to_digit(10) {
        Some(d) => d as i8,
        None => {
            match ch {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Unhandled char: {}", ch),
            }
        }
    } 
}

fn hand_strength(hand: &str) -> u32 {
    let groups: Vec<(char, usize)> = hand.chars()
        .sorted()
        .group_by(|ch| *ch)
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .collect();

    let strength = match groups[..] {
        [_] => 10, // five of a kind
        [a, b] if a.1 == 4 || b.1 == 4 => 9, // four of a kind
        [_, _] => 8, // full house
        [a, b, c] if a.1 == 3 || b.1 == 3 || c.1 == 3 => 7, // three of a kind
        [_, _, _] => 6, // two pair
        [_, _, _, _] => 5, // one pair
        [_, _, _, _, _] => 4, // high card
        _ => panic!("Unhandled: {}", hand),
    };

    strength
}

fn compare_hands_asc(a: &str, b: &str) -> Ordering {
    let s1 = hand_strength(a);
    let s2 = hand_strength(b);
    if s1 < s2 { Ordering::Less }
    else if s1 > s2 { Ordering::Greater }
    else {
        let first_diff = a.chars().zip(b.chars()).find(|(a, b)| a != b);
        match first_diff {
            Some((a, b)) => {
                let diff = (char_value(&a) - char_value(&b)).signum();
                match diff {
                    -1 => Ordering::Less,
                    0 => Ordering::Equal,
                    1 => Ordering::Greater,
                    _ => panic!("Unexpected diff for {} and {}: {}", a, b, diff),
                }
            }
            None =>  Ordering::Equal,
        }
    }
}

fn compare_hands_desc(a: &str, b: &str) -> Ordering {
    compare_hands_asc(a, b).reverse()
}

fn parse_input(input: &Input) -> Vec<HandBid> {
    input.as_lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect_vec();
            match parts[..] {
                [a, b] => HandBid { hand: a.to_string(), bid: b.parse().unwrap() },
                _ => panic!("Unexpected line: {}", line),
            }
        })
        .collect_vec()
}

fn part1(input: &Input) -> Result<u32> {
    // sort so that first is strongest
    let hand_bids = parse_input(input);
    let hand_bids_ord = hand_bids
        .iter()
        .sorted_by(|a, b| compare_hands_desc(&a.hand, &b.hand))
        .collect_vec();
    let result = hand_bids_ord
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hb)| {
            let rank = hand_bids_ord.len() - idx;
            acc + rank as u32 * hb.bid
        });

    Ok(result)
}

fn part2(input: &Input) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, hand_strength};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_hand_strengh() -> Result<()> {
        assert_eq!(hand_strength("AAAAA"), 10);
        assert_eq!(hand_strength("AA8AA"), 9);
        assert_eq!(hand_strength("23332"), 8);
        assert_eq!(hand_strength("TTT98"), 7);
        assert_eq!(hand_strength("23432"), 6);
        assert_eq!(hand_strength("A23A4"), 5);
        assert_eq!(hand_strength("23456"), 4);
        Ok(())
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part1(&input).unwrap(), 6440);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 250946742);
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
