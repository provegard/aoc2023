use anyhow::Result;
use std::collections::HashMap;

use regex::Regex;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day02/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn satisfies<T, P>(opt: Option<T>, predicate: P) -> bool where P: Fn(T) -> bool {
    match opt {
        Some(t) => predicate(t),
        None => false,
    }
}

fn color_counts(game: &Game) -> HashMap<String, u32> {
    let all_cubes = game.picks.iter().flat_map(|p| p.cubes.clone());
    let cc = all_cubes.fold(HashMap::<String, u32>::new(), |acc, pick| {
        let current_count = *acc.get(&pick.0).unwrap_or(&0u32);
        if pick.1 > current_count {
            let mut new_acc = acc.clone();
            new_acc.insert(pick.0, pick.1);
            new_acc
        } else {
            acc
        }
    });
    cc
}

fn part1(input: &Input) -> Result<u32> {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let games = input.as_lines().map(|line| parse_game(line));
    let possible_games = games.filter(|game| {
        let cc = color_counts(game);

        let possible = satisfies(cc.get("red"), |x| x <= &12u32) &&
            satisfies(cc.get("green"), |x| x <= &13u32) &&
            satisfies(cc.get("blue"), |x| x <= &14u32);

        possible
    });
    let result = possible_games.map(|game| game.id).sum();
    Ok(result)
}

fn part2(input: &Input) -> Result<u32> {
    let games = input.as_lines().map(|line| parse_game(line));
    let powers = games.map(|game| {
        let cc = color_counts(&game);

        let power = cc.iter().fold(1u32, |acc, pair| acc * pair.1);

        power
    });
    let result = powers.sum();
    Ok(result)
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Pick {
    cubes: Vec<(String, u32)>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    picks: Vec<Pick>,
}

fn parse_game(s: &str) -> Game {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    match re.captures(s) {
        Some(caps) => {
            let cubes_concat = caps.get(2).unwrap().as_str();
            let picks = cubes_concat.split("; ").fold(Vec::<Pick>::new(), |acc, cc| {
                let cubes: Vec<_> = cc.split(", ").fold(Vec::<(String, u32)>::new(), |acc, c| {
                    let parts: Vec<_> = c.split(" ").collect();
                    let count = parts.get(0).unwrap().parse::<u32>().unwrap();
                    let color = parts.get(1).unwrap();
                    let mut new_acc = acc.clone();
                    new_acc.push((color.to_string(), count));
                    new_acc
                });
                
                let mut new_acc = acc.clone();
                new_acc.push(Pick { cubes });
                new_acc
            });

            Game {
                id: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                picks,
            }
        }
        None => panic!("Cannot parse game from {}", s),
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, parse_game, Pick};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_parse_game_id() -> Result<()> {
        let game = parse_game("Game 10: 3 blue, 4 red");
        assert_eq!(game.id, 10);
        Ok(())
    }

    #[test]
    pub fn test_parse_pick() -> Result<()> {
        let game = parse_game("Game 10: 3 blue, 4 red");
        assert_eq!(game.picks, vec![
            Pick {
                cubes: vec![
                    (String::from("blue"), 3),
                    (String::from("red"), 4),
                ]
            }
        ]);
        Ok(())
    }

    #[test]
    pub fn test_parse_multiple_pick() -> Result<()> {
        let game = parse_game("Game 10: 3 blue, 4 red; 2 yellow, 1 green");
        assert_eq!(game.picks, vec![
            Pick {
                cubes: vec![
                    (String::from("blue"), 3),
                    (String::from("red"), 4),
                ]
            },
            Pick {
                cubes: vec![
                    (String::from("yellow"), 2),
                    (String::from("green"), 1),
                ]
            }
        ]);
        Ok(())
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);
        assert_eq!(part1(&input).unwrap(), 8);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 2169);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);
        assert_eq!(part2(&input).unwrap(), 2286);
        Ok(())
    }
    
    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 60948);
        Ok(())
    }
}
