#[derive(Default)]
pub struct SeatPos {
    col: Vec<char>,
    row: Vec<char>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<SeatPos> {
    input
        .lines()
        .map(|l| {
            let mut seat_pos = SeatPos::default();
            let pass: Vec<char> = l.chars().collect();

            // Get the row
            for i in 0..7 {
                seat_pos.row.push(pass[i]);
            }
            
            // Get the column
            for i in 7..10 {
                seat_pos.col.push(pass[i]);
            }

            seat_pos
        }).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[SeatPos]) -> Option<i32> {
    seat_ids(input).into_iter().max()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[SeatPos]) -> i32 {
    let ids = seat_ids(input);
    let min_id = seat_ids(input).into_iter().min().unwrap();
    let max_id = seat_ids(input).into_iter().max().unwrap();
    
    for i in min_id..max_id {
        if !ids.contains(&i) {
            return i;
        }
    }
    
    -1
}

fn seat_ids(input: &[SeatPos]) -> Vec<i32> {
    input
        .into_iter()
        .map(|s| {
            let mut upper = 127;
            let mut lower = 0;
            let mut step = 64;

            // Get row
            for i in 0..7 {
                match s.row[i] {
                    'F' => upper -= step,
                    'B' => lower += step,
                    _ => {},
                }
                step /= 2;
            }
            let row = upper;

            // Reset the ranges
            upper = 7;
            lower = 0;
            step = 4;

            // Get column
            for i in 0..3 {
                match s.col[i] {
                    'L' => upper -= step,
                    'R' => lower += step,
                    _ => {},
                }
                step /= 2;
            }
            let col = upper;

            row * 8 + col
        }).collect()
}