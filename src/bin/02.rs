use itertools::Itertools;

advent_of_code::solution!(2);

fn fold(acc: i32, (a, b): (i32, i32)) -> Result<i32, ()> {
    if acc >= 0 && (1..=3).contains(&(b - a)) {
        Ok(1)
    } else if acc <= 0 && (1..=3).contains(&(a - b)) {
        Ok(-1)
    } else {
        Err(())
    }
}

fn is_safe(input: &str) -> bool {
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .try_fold(0, fold)
        .is_ok()
}

fn is_safe_brute_force(input: &str) -> bool {
    let levels = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // Iterate over each index `i` in the `levels` vector
    // and check if removing the element at index `i` results in a safe sequence.
    (0..levels.len()).any(|i| {
        levels[0..i]
            .iter()
            .chain(&levels[i + 1..]) // i + 1 skips the element at index `i`
            .tuple_windows()
            .try_fold(0, |acc, (a, b)| fold(acc, (*a, *b)))
            .is_ok()
    })
}

pub fn part_one(input: &str) -> Option<i32> {
    input
        .lines()
        .filter(|line| is_safe(line))
        .count()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .filter(|line| is_safe_brute_force(line))
        .count()
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
