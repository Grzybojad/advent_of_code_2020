#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|p| {
            p
                .lines()
                .collect::<Vec<&str>>()[1..]
                .into_iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> usize {
    let mut decks = advance_combat(input);
    
    while !decks.iter().any(|d| d.is_empty()) {
        decks = advance_combat(&decks);
    }

    count_score(&decks)
}

fn copy_deck(input: &[i32]) -> Vec<i32> {
    input.into_iter().map(|&c| c.to_owned()).collect()
}

fn advance_combat(input: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut resulting_decks = Vec::new();

    resulting_decks.push(copy_deck(&input[0]));
    resulting_decks.push(copy_deck(&input[1]));
    
    let p1_card = resulting_decks[0].remove(0);
    let p2_card = resulting_decks[1].remove(0);
    
    if p1_card > p2_card {
        resulting_decks[0].push(p1_card);
        resulting_decks[0].push(p2_card);
    }
    else {
        resulting_decks[1].push(p2_card);
        resulting_decks[1].push(p1_card);
    }

    resulting_decks
}

fn count_score(input: &[Vec<i32>]) -> usize {
    input
        .into_iter()
        .map(|deck| {
            let mut i = deck.len();
            let mut points = 0;
            deck
                .into_iter()
                .for_each(|&c| {
                    points += (c as usize) * i;
                    i -= 1;
                });
            
            points
        })
        .sum()
}