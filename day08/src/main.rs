use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use util::Input;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    nodes: Vec<Node>,
}

fn parse_input(input: &Input) -> Map {
    let lines = input.as_lines().collect_vec();
    let instructions = lines.first().unwrap().chars().collect_vec();

    let re = Regex::new(r"(?<node>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)").unwrap();

    let nodes = lines.iter().skip(2).map(|line| {
        match re.captures(line) {
            Some(caps) => {
                let name = caps["node"].to_string();
                let left = caps["left"].to_string();
                let right = caps["right"].to_string();
                Node { name, left, right}
            },
            None => panic!("Unrecognized: {}", line),
        }
    }).collect_vec();

    Map { instructions, nodes }
}

fn steps(map: &Map) -> u32 {
    // Mutable solution to avoid stack overflow. Is there a way to force tail recursion?
    let mut node_name = "AAA";
    let mut idx = 0;
    let mut step_count = 0;

    while node_name != "ZZZ" {
        let lr = map.instructions.get(idx).unwrap();
        let node = map.nodes.iter().find(|n| n.name == node_name).unwrap();
    
        node_name = if *lr == 'L' { &node.left } else { &node.right };

        idx = (idx + 1) % map.instructions.len();
        step_count += 1;
    }

    step_count
}

fn part1(input: &Input) -> Result<u32> {
    let map = parse_input(input);
    let result = steps(&map);
    Ok(result)
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
    pub fn test_part1_q() -> Result<()> {
        let input = Input::load("example1")?;
        assert_eq!(part1(&input).unwrap(), 2);
        Ok(())
    }

    #[test]
    pub fn test_part1_b() -> Result<()> {
        let input = Input::load("example2")?;
        assert_eq!(part1(&input).unwrap(), 6);
        Ok(())
    }

    #[test]
    pub fn test_part1_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part1(&input).unwrap(), 15517);
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
