use std::collections::HashMap;
use std::borrow::Borrow;

#[derive(Default)]
pub struct Rule {
    contents: HashMap<String, i32>,
    bag: String,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Rule> {
    let mut rules = Vec::new();
    
    input
        .lines()
        .for_each(|s| {
            let mut rule = Rule::default();

            let mut split_rule = s[0..s.len()-1].trim().split(" bags contain ");
            rule.bag = split_rule.next().unwrap().to_string();
            
            // Get the bag contents
            split_rule
                .next()
                .unwrap()
                .split(", ")
                .for_each(|con| {
                    let mut split_bag = con.split_whitespace();
    
                    let amount: i32 = match split_bag.next() {
                        Some("no") => 0,
                        Some(x) => x.parse().unwrap(),
                        _ => 0,
                    };
                    
                    let mut bag_name: String = String::default();
                    if amount != 0 {
                        bag_name = [split_bag.next().unwrap(), split_bag.next().unwrap()].join(" ");
                    }
                    
                    rule.contents.insert(bag_name, amount);
                });
            
            rules.push(rule);
        });
    
    rules
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Rule]) -> usize {
    let mut looking_for = ["shiny gold"].to_vec();
    
    let mut prev_found_bags = 1;
    let mut found_bags = 0;
    while prev_found_bags != found_bags {
        prev_found_bags = found_bags;
        
        found_bags = input
            .into_iter()
            .filter(|r| {
                for key in r.contents.keys() {
                    if looking_for.contains(&key.as_str()) {
                        looking_for.push(r.bag.as_str());
                        return true;
                    }
                }
                
                false
            }).count()
    }

    found_bags
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Rule]) -> usize {
    nr_of_bags(input, "shiny gold".to_string())-1
}

fn nr_of_bags(input: &[Rule], color: String) -> usize {
    let mut sum = 0;
    
    input
        .into_iter()
        .for_each(|r| {
            if r.bag == color {
                let contents = r.contents.borrow();
                
                sum = contents
                    .into_iter()
                    .map(|(b, &i)| (i as usize) * nr_of_bags(input, b.to_string()))
                    .sum();
            }
        });
    
    sum+1
}