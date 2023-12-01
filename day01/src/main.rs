use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};

static DIGIT_NAMES: &[&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn min_of(option1: Option<usize>, option2: Option<usize>) -> Option<usize> {
    match (option1, option2) {
        (Some(a), Some(b)) => Some(std::cmp::min(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn max_of(option1: Option<usize>, option2: Option<usize>) -> Option<usize> {
    match (option1, option2) {
        (Some(a), Some(b)) => Some(std::cmp::max(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn read_input() -> Result<Vec<String>> {
    let path = "day01/input";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    return Ok(lines);
}

fn digit_value_from_name(str: &String, idx: usize) -> u32 {
    DIGIT_NAMES.iter().enumerate()
        .find(|&(_index, s)| {
            if idx + s.len() <= str.len() {
                let string_slice = &str[idx..idx + s.len()];
                return string_slice == *s;
            }
            false
        })
        .map(|(index, _s)| index as u32)
        .unwrap()
}

fn digit_value(str: &String, idx: usize) -> u32 {
    let ch = str.as_bytes()[idx] as char;
    if ch.is_digit(10) {
        match ch.to_digit(10) {
            Some(value) => value,
            None => panic!("Failed get the digit at index {} in {}", idx, str)
        }
    } else {
        digit_value_from_name(str, idx)
    }
}

fn first_digit(str: &String) -> u32 {
    let first_digit_idx = str.find(|c: char| c.is_digit(10)); // .map(|idx| digit_value(str, idx));
    let name_indexes: Vec<Option<usize>> = DIGIT_NAMES.iter().map(|&s| str.find(s)).collect();

    let min_name_index = name_indexes.iter()
        .filter_map(|&opt| opt)
        .min();
    
    
    let m = min_of(first_digit_idx, min_name_index);

    match m {
        Some(idx) => digit_value(str, idx),
        None => panic!("Failed to find the first digit (numeric or written-out)")
    }
}

fn last_digit(str: &String) -> u32 {
    let last_digit_idx = str.rfind(|c: char| c.is_digit(10)); // .map(|idx| digit_value(str, idx));
    let name_indexes: Vec<Option<usize>> = DIGIT_NAMES.iter().map(|&s| str.rfind(s)).collect();

    let max_name_index = name_indexes.iter()
        .filter_map(|&opt| opt)
        .max();
    
    let m = max_of(last_digit_idx, max_name_index);

    match m {
        Some(idx) => digit_value(str, idx),
        None => panic!("Failed to find the last digit (numeric or written-out)")
    }
}

fn main() -> Result<()> {
    let input = read_input()?;

    println!("Part 1:");
    println!("{}", part1(&input)?); // 56397

    println!("Part 2:");
    println!("{}", part2(&input)?); // 55701
    Ok(())
}


fn part1(input: &Vec<String>) -> Result<u32> {
    let value = input.iter().fold(0, |acc, s| {
        let first_digit_idx = s.find(|c: char| c.is_digit(10)).unwrap();
        let last_digit_idx = s.rfind(|c: char| c.is_digit(10)).unwrap();

        acc + 10 * digit_value(&s, first_digit_idx) + digit_value(&s, last_digit_idx)
    });
    Ok(value)
}

fn part2(input: &Vec<String>) -> Result<u32> {
    let value = input.iter().fold(0, |acc, s| {
        let first = first_digit(s);
        let last = last_digit(s);

        acc + 10 * first + last
    });
    Ok(value)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input: Vec<String> = vec!(
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        );
        assert_eq!(part1(&input).unwrap(), 142);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input: Vec<String> = vec!(
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        );
        assert_eq!(part2(&input).unwrap(), 281);
        Ok(())
    }
}
