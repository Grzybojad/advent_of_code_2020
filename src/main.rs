use std::fs;

fn main() {
    let mut input = input_generator("./input/day1.txt");
    
    let product2 = find_product_two(&mut input, 2020);
    let product3 = find_product_three(&mut input, 2020);
    
    match product2 {
        Some(x) => println!("Pair product: {}", x.to_string()),
        None => println!("Couldn't find pair"),
    }

    match product3 {
        Some(x) => println!("Triple product: {}", x.to_string()),
        None => println!("Couldn't find triple"),
    }
}

pub fn input_generator(filename: &str) -> Vec<i32> {
    let file_text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    file_text
        .lines()
        .map(|e| e.parse().unwrap())
        .collect()
}

fn find_product_two(input: &mut Vec<i32>, sum: i32) -> Option<i32> {
    let entries = input;
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

fn find_product_three(input: &mut Vec<i32>, sum: i32) -> Option<i32> {
    let entries = input;
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