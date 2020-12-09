use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let preamble = 25;
    let mut queue = VecDeque::with_capacity(preamble);
    let mut input_iter = input.into_iter();
    
    for  _i in 0..preamble {
        let str = input_iter.next().unwrap();
        queue.push_back(str);
    }
    
    for _i in preamble..input_iter.len() {
        let curr = input_iter.next().unwrap();
        if !can_construct_from(&queue, *curr) {
            return *curr;
        }
        
        queue.pop_front();
        queue.push_back(curr);
    }
    
    0
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let invalid = solve_part1(input);
    
    println!("{}", input.len());
    
    for a in 0..input.len() {
        let mut sum = input[a];
        let mut smallest = input[a];
        let mut largest = input[a];
        
        for b in a+1..input.len() {
            sum += input[b];
            
            if input[b] > largest { largest = input[b]; }
            if input[b] < smallest { smallest = input[b]; }
            
            if sum == invalid {
                return smallest + largest;
            }
            else if sum > invalid {
                break;
            }
        }
    }
    
    0
}

fn can_construct_from(vd: &VecDeque<&usize>, target: usize) -> bool {
    for a in 0..vd.len() {
        for b in a+1..vd.len() {
            if *vd.get(a).unwrap() + *vd.get(b).unwrap() == target {
                return true;
            }
        }
    }
    
    false
}