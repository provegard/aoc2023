use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use util::Input;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    nodes: HashMap<String, Node>,
}

fn parse_input(input: &Input) -> Map {
    let lines = input.as_lines().collect_vec();
    let instructions = lines.first().unwrap().chars().collect_vec();

    let re = Regex::new(r"(?<node>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)").unwrap();

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

    let hm: HashMap<String, Node> = nodes.iter().map(|n| (n.name.to_string(), n.clone())).collect();

    Map { instructions, nodes: hm }
}

fn steps(map: &Map) -> u32 {
    // Mutable solution to avoid stack overflow. Is there a way to force tail recursion?
    let mut node_name = "AAA";
    let mut idx = 0;
    let mut step_count = 0;

    while node_name != "ZZZ" {
        let lr = map.instructions.get(idx).unwrap();
        let node = map.nodes.get(node_name).unwrap();
    
        node_name = if *lr == 'L' { &node.left } else { &node.right };

        idx = (idx + 1) % map.instructions.len();
        step_count += 1;
    }

    step_count
}

fn all_z(nodes: &Vec<&Node>) -> bool {
    nodes.iter().all(|n| n.name.ends_with("Z"))
}

fn steps_sim(map: &Map) -> u32 {
    let mut nodes = map.nodes.iter().filter(|(_, n)| n.name.ends_with("A")).map(|(_, n)| n).collect_vec();
    let mut idx = 0;
    let mut step_count = 0;

    while !all_z(&nodes) {
        let lr = map.instructions.get(idx).unwrap();

        let node_names = nodes.iter().map(|n| if *lr == 'L' { &n.left } else { &n.right }).collect_vec();
        nodes = node_names.iter().map(|nn| map.nodes.get(*nn).unwrap()).collect_vec();

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
    let map = parse_input(input);
    let result = steps_sim(&map);
    Ok(result)
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

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::load("example3")?;
        assert_eq!(part2(&input).unwrap(), 6);
        Ok(())
    }

    #[test]
    pub fn test_part2_input() -> Result<()> {
        let input = Input::load("input")?;
        assert_eq!(part2(&input).unwrap(), 0);
        Ok(())
    }
}
