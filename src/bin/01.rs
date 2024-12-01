advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lists: (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        lists.0.push(numbers[0].parse::<i32>().unwrap());
        lists.1.push(numbers[1].parse::<i32>().unwrap());
    }

    lists
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut lists = parse_input(&input);

    lists.0.sort();
    lists.1.sort();

    let result: i32 = lists.0.iter()
        .zip(lists.1.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
