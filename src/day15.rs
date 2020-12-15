#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut numbers = Vec::new();
    
    for i in 0..30000000 {
        if i < input.len() {
            numbers.push(input[i])
        }
        else {
            match numbers[..numbers.len()-1].iter().rposition(|p| p == numbers.last().unwrap()) {
                None => numbers.push(0),
                Some(pos) => numbers.push(i - pos-1),
            }
        }
    }

    *numbers.last().unwrap()
}