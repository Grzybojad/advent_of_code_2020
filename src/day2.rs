pub struct Line {
    min: usize,
    max: usize,
    char: char,
    pass: String
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();

            let min_max: Vec<&str> = parts[0].split('-').collect();

            Line {
                min: min_max[0].parse::<usize>().unwrap(),
                max: min_max[1].parse::<usize>().unwrap(),
                char: parts[1].chars().next().unwrap(),
                pass: parts[2].to_string()
            }
        }).collect()
}

#[aoc(day2, part1)]
pub fn count_valid_passwords(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|l| {
            let char_count = l.pass
                .chars()
                .filter(|c| c == &l.char)
                .count();
            
            char_count >= l.min && char_count <= l.max
        }).count()
}

#[aoc(day2, part2)]
pub fn count_valid_passwords2(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|l| {
            let a = match l.pass.chars().nth(l.min-1) {
                None => '_',
                Some(x) => x,
            };
            let b = match l.pass.chars().nth(l.max-1) {
                None => '_',
                Some(x) => x,
            };
            
            (a == l.char || b == l.char) && !(a == l.char && b == l.char)
        }).count()
}