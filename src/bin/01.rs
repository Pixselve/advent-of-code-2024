use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut first_list = vec![];
    let mut second_list = vec![];
    input.lines().for_each(|line| {
        let parsed_line: Vec<u32> = line
            .split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect();
        first_list.push(parsed_line[0]);
        second_list.push(parsed_line[1])
    });

    first_list.sort();
    second_list.sort();

    assert_eq!(first_list.len(), second_list.len());

    let result = first_list
        .iter()
        .zip(second_list)
        .map(|(first, second)| first.abs_diff(second))
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut first_list = vec![];
    let mut second_list = vec![];
    input.lines().for_each(|line| {
        let parsed_line: Vec<u32> = line
            .split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect();
        first_list.push(parsed_line[0]);
        second_list.push(parsed_line[1])
    });

    let mut second_list_map = HashMap::new();

    for x in second_list {
        second_list_map.insert(x, second_list_map.get(&x).unwrap_or(&0) + 1);
    }

    let score = first_list
        .iter()
        .map(|x| x * second_list_map.get(x).unwrap_or(&0))
        .sum();

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
