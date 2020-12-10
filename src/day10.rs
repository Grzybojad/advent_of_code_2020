#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut out: Vec<i32> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    out.sort();
    out
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut one_jump = 0;
    let mut three_jump = 0;
    let mut prev_jolt = 0;
    
    let mut input_iter = input.into_iter();
    
    while let Some(jolt) = input_iter.next() {
        match jolt - prev_jolt {
            3 => three_jump += 1,
            1 => one_jump += 1,
            _ => panic!(),
        }
        
        prev_jolt = *jolt;
    }
    
    one_jump * (three_jump+1)
}
