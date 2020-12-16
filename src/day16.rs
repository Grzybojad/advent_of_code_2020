#[derive(Default, Copy, Clone)]
pub struct Range {
    start: i32,
    end: i32
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    
    let mut ranges: Vec<Range> = Vec::new();
    
    // Calculate total range from rules
    parts[0].lines().collect::<Vec<&str>>().into_iter().for_each(|r| {
        let ranges_str = r.split(": ").collect::<Vec<&str>>()[1].split(" or ").collect::<Vec<&str>>();
        
        for range_str in ranges_str {
            let start_end = range_str.split("-").collect::<Vec<&str>>();
            let range = Range{start: start_end[0].parse().unwrap(), end: start_end[1].parse().unwrap() };

            ranges.push(range);
        }
    });
    
    let combined_ranges = reduce_ranges(&mut ranges);

    let mut num = Vec::new();
    
    parts[2]
        .lines()
        .collect::<Vec<&str>>()[1..]
        .into_iter()
        .for_each(|l| {
            l
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .for_each(|i| num.push(i.parse::<i32>().unwrap()));
        });

    num.into_iter().filter(|i| !num_in_ranges(&combined_ranges, i)).collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.into_iter().sum()
}

fn intersect(r1: &Range, r2: &Range) -> bool {
    (r1.start <= r2.start && r2.start <= r1.end && r1.end <= r2.end) || 
    (r2.start <= r1.start && r1.start <= r2.end && r2.end <= r1.end) || 
    (r1.start <= r2.start && r2.start <= r2.end && r2.end <= r1.end) ||
    (r2.start <= r1.start && r1.start <= r1.end && r1.end <= r2.end)
}

fn combine_ranges(r1: &Range, r2: &Range) -> Range {
    if intersect(r1, r2) {
        Range {
            start: if r1.start < r2.start { r1.start } else { r2.start },
            end: if r1.end > r2.end {r1.end} else {r2.end}
        }
    }
    else {
        panic!();
    }
}

fn reduce_ranges(ranges: &mut Vec<Range>) -> Vec<Range> {
    let mut combos = 1;
    
    while combos != 0 {
        combos = 0;

        for i in 0..ranges.len() {
            for j in i+1..ranges.len() {
                if j < ranges.len() && intersect(&ranges[i], &ranges[j]) {
                    let combined = combine_ranges(&ranges[i], &ranges[j]);
                    ranges.remove(i);
                    ranges.remove(j-1);
                    ranges.push(combined);
                    combos += 1;
                }
            }
        }
    }
    
    ranges.to_owned()
}

fn num_in_ranges(ranges: &Vec<Range>, num: &i32) -> bool {
    ranges.into_iter().any(|r| num >= &r.start && num <= &r.end)
}