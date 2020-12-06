#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n" )
        .map(|group| {
            let mut answers: Vec<char> = group.chars().filter(|c| c.is_alphabetic()).collect();
            answers.sort();
            answers.dedup();
            answers.into_iter().collect()
        }).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input
        .into_iter()
        .map(|l| l.len())
        .sum()
}