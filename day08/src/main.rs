use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use util::Input;
use num_integer::lcm;
use tailcall::tailcall;

#[derive(Debug, Clone)]
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

    Map { instructions, nodes }
}

fn find_next_node_name<'a>(map: &'a Map, instr_idx: usize, node_name: &str) -> &'a str {
    let lr = map.instructions.get(instr_idx).unwrap();
    let node = map.nodes.iter().find(|n| n.name == node_name).unwrap();

    if *lr == 'L' { &node.left } else { &node.right }
}

fn steps(map: &Map) -> u32 {

    #[tailcall]
    fn steps_inner(map: &Map, node_name: &str, idx: usize, step_count: u32) -> u32 {
        match node_name {
            "ZZZ" => step_count,
            _ => {
                let next_node_name = find_next_node_name(map, idx, node_name);
                let next_idx = (idx + 1) % map.instructions.len();
                steps_inner(map, next_node_name, next_idx, step_count + 1)
            }
        }
    }

    steps_inner(map, "AAA", 0, 0)
}

fn find_period(map: &Map, initial_node_name: &str) -> u32 {

    #[tailcall]
    fn find_period_inner(map: &Map, node_name: &str, idx: usize, step_count: u32) -> u32 {
        if node_name.ends_with("Z") {
            step_count
        } else {
            let next_node_name = find_next_node_name(map, idx, node_name);
            let next_idx = (idx + 1) % map.instructions.len();
            find_period_inner(map, next_node_name, next_idx, step_count + 1)
        }
    }

    find_period_inner(map, initial_node_name, 0, 0)
}

fn steps_sim(map: &Map) -> u64 {
    let node_names = map.nodes.iter().filter(|n| n.name.ends_with("A")).collect_vec();

    let periods = node_names.iter().map(|nn| find_period(map, &nn.name)).collect_vec();

    periods.iter().fold(1u64, |acc, p| lcm(acc, *p as u64))
}

fn part1(input: &Input) -> Result<u32> {
    let map = parse_input(input);
    let result = steps(&map);
    Ok(result)
}

fn part2(input: &Input) -> Result<u64> {
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
        assert_eq!(part2(&input).unwrap(), 14935034899483u64);
        Ok(())
    }
}
