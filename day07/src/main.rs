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
                '*' => 1, // Joker
                _ => panic!("Unhandled char: {}", ch),
            }
        }
    } 
}

fn card_combos(count: usize) -> Vec<Vec<char>> {
    let cards = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    let vec = cards
        .iter()
        .combinations_with_replacement(count)
        .map(|combo| combo.into_iter().cloned().collect())
        .collect_vec();
    vec
}

fn hand_jokered(hand: &str) -> Vec<String> {
    // Find the indexes of joker chars in the hand.
    let joker_indexes = hand.chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '*')
        .map(|(idx, _)| idx)
        .collect_vec();

    let joker_count = joker_indexes.len();
    if joker_count == 0 {
        vec![hand.to_string()]
    } else {
        let joker_combos = card_combos(joker_count);
        joker_combos.iter().map(|combo| {
            let new_chars = hand.chars().enumerate().map(|(idx, ch)| {
                if ch == '*' {
                    // replace with a joker char
                    match joker_indexes.iter().find_position(|i| **i == idx) {
                        Some(tup) => *combo.get(tup.0).unwrap(),
                        None => panic!("invalid index")
                    }
                } else {
                    ch
                }
            });
            let s: String = new_chars.collect();
            s
        })
        .collect_vec()
    }
}

fn hand_strength(hand: &str) -> u32 {

    let hands = hand_jokered(hand);

    let strengh_opt = hands.iter().map(|h| {
        let groups: Vec<(char, usize)> = h.chars()
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
    }).max();

    match strengh_opt {
        Some(s) => s as u32,
        None => 0
    }
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

fn winnings(hand_bids: &Vec<HandBid>) -> u32 {
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
    result
}

fn part1(input: &Input) -> Result<u32> {
    // sort so that first is strongest
    let hand_bids = parse_input(input);
    let result = winnings(&hand_bids);

    Ok(result)
}

fn part2(input: &Input) -> Result<u32> {
    let hand_bids = parse_input(input)
        .iter()
        .map(|hb| {
            let jokered = hb.hand.replace("J", "*");
            HandBid { hand: jokered, bid: hb.bid }
        })
        .collect_vec();

    let result = winnings(&hand_bids);
    Ok(result)
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
    pub fn test_hand_strengh_joker() -> Result<()> {
        assert_eq!(hand_strength("AAAA*"), 10);
        assert_eq!(hand_strength("AAA**"), 10);
        assert_eq!(hand_strength("AA***"), 10);
        assert_eq!(hand_strength("A****"), 10);
        assert_eq!(hand_strength("*****"), 10);
        assert_eq!(hand_strength("Q**Q2"), 9);
        assert_eq!(hand_strength("T55*5"), 9);
        assert_eq!(hand_strength("KT**T"), 9);
        assert_eq!(hand_strength("1234*"), 5);
        assert_eq!(hand_strength("123**"), 7);
        assert_eq!(hand_strength("12***"), 9);
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

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::load("example")?;
        assert_eq!(part2(&input).unwrap(), 5905);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 251824095);
        Ok(())
    }
}
