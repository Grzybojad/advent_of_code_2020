
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    tree_count( input, 1, 3 )
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    tree_count( input, 1, 1 ) *
    tree_count( input, 1, 3 ) *
    tree_count( input, 1, 5 ) *
    tree_count( input, 1, 7 ) *
    tree_count( input, 2, 1 )
}


pub fn tree_count(input: &[String], down: usize, right: usize) -> usize {
    let mut tree_count = 0;
    let mut x = 0;
    
    for y in (0..input.len()).step_by(down) {
        let mut line = input[y].chars();
        
        if line.nth(x) == Some('#') {
            tree_count += 1;
        }
    
        x += right;
        x %= input[y].len();
    }

    tree_count
}