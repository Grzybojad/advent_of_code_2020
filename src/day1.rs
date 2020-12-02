#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|e| e.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn find_product_two(input: &[i32]) -> Option<i32> {
    let sum = 2020;

    let mut v = input.to_vec();
    let entries = v.as_mut_slice();
    entries.sort();

    for i in 0..entries.len() {
        for j in i+1..entries.len() {
            if entries[i] + entries[j] == sum {
                println!("Found pair {} and {}", entries[i], entries[j]);
                return Some(entries[i] * entries[j]);
            }
            else if entries[i] + entries[j] > sum {
                break;
            }
        }
    }

    None
}

#[aoc(day1, part2)]
fn find_product_three(input: &[i32]) -> Option<i32> {
    let sum = 2020;
    
    let mut v = input.to_vec();
    let entries = v.as_mut_slice();
    entries.sort();

    for i in 0..entries.len() {
        for j in i+1..entries.len() {
            for k in j+1..entries.len() {
                if entries[i] + entries[j] + entries[k] == sum {
                    println!("Found triple {}, {} and {}", entries[i], entries[j], entries[k]);
                    return Some(entries[i] * entries[j] * entries[k]);
                }
                else if entries[i] + entries[j] + entries[k] > sum {
                    break;
                }
            }
        }
    }

    None
}