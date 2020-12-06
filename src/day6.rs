#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n" )
        .map(|group| {
            let single = group.split("\n");
            
            single.map(|s| {
                let mut answers: Vec<char> = s.chars().filter(|c| c.is_alphabetic()).collect();
                answers.sort();
                answers.dedup();
                answers.into_iter().collect()
            }).collect()
            
        }).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<String>]) -> usize {
    input
        .into_iter()
        .map(|group| {
            let mut ans: Vec<char> = group.join("").chars().collect();
            ans.sort();
            ans.dedup();
            ans.len()
        }).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Vec<String>]) -> usize {
    input
        .into_iter()
        .map(|group| {
            let common: String = 
                group[0]
                    .chars()
                    .filter(|k| 
                        group[1..].iter().all(|s| 
                            (*s).contains(*k))
                    ).collect();
            
            common.len()
        })
        .sum()
}