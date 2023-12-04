use anyhow::Result;
use std::collections::{HashSet, HashMap};

use util::Input;

struct Card {
    id: u32,
    winning: HashSet<u32>,
    on_hand: Vec<u32>,
}

fn to_card(line: &str) -> Card {
    let col_idx = line.find(":").unwrap();
    let pipe_idx = line.find("|").unwrap();
    let winning: HashSet<_> = line[col_idx+1..pipe_idx].trim().split_ascii_whitespace().map(|str_num| str_num.parse::<u32>().unwrap()).collect();
    let on_hand: Vec<_> = line[pipe_idx+1..].trim().split_ascii_whitespace().map(|str_num| str_num.parse::<u32>().unwrap()).collect();
    let id = line[..col_idx].replace("Card", "").trim().parse::<u32>().unwrap();

    return Card { id, winning, on_hand };
}

fn part1(input: &Input) -> Result<u32> {
    let result: u32 = input.as_lines()
        .map(|line| {
            let card = to_card(line);

            let points = card.on_hand.iter().fold(0, |acc, num| {
                let is_winning = card.winning.contains(num);
                let ret = if is_winning {
                    if acc == 0 { 1 } else { acc * 2 }
                } else { acc };
                ret
            });
            points
        })
        .sum();
    Ok(result)
}

fn part2(input: &Input) -> Result<u32> {
    let mut card_counts = HashMap::<u32, u32>::new();

    let cards: Vec<_> = input.as_lines().map(|line| to_card(line)).collect();

    // Set initial counts
    for card in &cards {
        card_counts.insert(card.id, 1);
    }

    for card in &cards {
        let winning_card_count = card_counts.get(&card.id).unwrap().clone();
        let win_count = card.on_hand.iter().fold(0u32, |acc, num| if card.winning.contains(num) { acc + 1 } else { acc });
        let next = card.id + 1;
        for following_card_id in next..(next + win_count) {
            let following_card_count = card_counts.get(&following_card_id).unwrap();
            card_counts.insert(following_card_id, following_card_count + winning_card_count);
        }
    }

    let result = card_counts.values().sum();
    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]);
        assert_eq!(part1(&input).unwrap(), 13);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 21105);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]);
        assert_eq!(part2(&input).unwrap(), 30);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 5329815);
        Ok(())
    }
}
