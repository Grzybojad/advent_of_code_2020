#[derive(PartialEq)]
pub enum Space {
    Floor,
    Empty,
    Occupied
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Space>> {
    let mut seat_lines = Vec::new();

    // Get cubes from input
    input.lines().for_each(|l| {
        let mut seat_line = Vec::new();
        for c in l.chars() {
            match c {
                'L' => seat_line.push(Space::Empty),
                '.' => seat_line.push(Space::Floor),
                _ => panic!()
            }
        }
        seat_lines.push(seat_line);
    });

    seat_lines
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Vec<Space>]) -> usize {
    let mut s = step_simulation1(input);
    
    let mut occupied = 1;
    let mut occupied_new = count_occupied(input);
    
    while occupied != occupied_new {
        occupied = occupied_new;
        s = step_simulation1(&s);
        occupied_new = count_occupied(&s)
    }

    count_occupied(&s)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Vec<Space>]) -> usize {
    let mut s = step_simulation2(input);

    let mut occupied = 1;
    let mut occupied_new = count_occupied(input);

    while occupied != occupied_new {
        occupied = occupied_new;
        s = step_simulation2(&s);
        occupied_new = count_occupied(&s);
    }

    count_occupied(&s)
}

fn step_simulation1(input: &[Vec<Space>]) -> Vec<Vec<Space>> {
    let mut updated_seats = Vec::new();
    
    let mut y = 0;
    input
        .into_iter()
        .for_each(|l| {
            let mut seat_line = Vec::new();
            let mut x = 0;
            for s in l {
                match s {
                    Space::Empty => {
                        match nr_of_neighbours1(input, x, y) {
                            0 => seat_line.push(Space::Occupied),
                            _ => seat_line.push(Space::Empty),
                        }
                    },
                    Space::Occupied => {
                        match nr_of_neighbours1(input, x, y) {
                            n if n >= 4 => seat_line.push(Space::Empty),
                            _ => seat_line.push(Space::Occupied),
                        }
                    },
                    Space::Floor => seat_line.push(Space::Floor),
                }
                x += 1;
            }
            updated_seats.push(seat_line);
            y += 1;
        });
    
    updated_seats
}

fn step_simulation2(input: &[Vec<Space>]) -> Vec<Vec<Space>> {
    let mut updated_seats = Vec::new();

    let mut y = 0;
    input
        .into_iter()
        .for_each(|l| {
            let mut seat_line = Vec::new();
            let mut x = 0;
            for s in l {
                match s {
                    Space::Empty => {
                        match nr_of_neighbours2(input, x, y) {
                            0 => seat_line.push(Space::Occupied),
                            _ => seat_line.push(Space::Empty),
                        }
                    },
                    Space::Occupied => {
                        match nr_of_neighbours2(input, x, y) {
                            n if n >= 5 => seat_line.push(Space::Empty),
                            _ => seat_line.push(Space::Occupied),
                        }
                    },
                    Space::Floor => seat_line.push(Space::Floor),
                }
                x += 1;
            }
            updated_seats.push(seat_line);
            y += 1;
        });

    updated_seats
}

fn nr_of_neighbours1(input: &[Vec<Space>], pos_x: i32, pos_y: i32) -> i32 {
    let mut count = 0;
    for y in pos_y - 1..=pos_y + 1 {
        for x in pos_x - 1..=pos_x + 1 {
            if x >= 0 && y >= 0 && (y as usize) < input.len() && (x as usize) < input[0].len() && !(x == pos_x && y == pos_y) {
                if input[y as usize][x as usize] == Space::Occupied {
                    count += 1;
                }
            }
        }
    }

    count
}

fn nr_of_neighbours2(input: &[Vec<Space>], pos_x: i32, pos_y: i32) -> i32 {
    let mut count = 0;
    
    let dir_x: [i32; 8] = [0, 1, 1, 1, 0,-1,-1,-1];
    let dir_y: [i32; 8] = [1, 1, 0,-1,-1,-1, 0, 1];
    
    for dir in 0..8 {
        let mut looking = true;
        
        let mut distance = 1;
        while looking {
            let x = pos_x + distance * dir_x[dir];
            let y = pos_y + distance * dir_y[dir];
            
            if x < 0 || y < 0 || x as usize >= input[0].len() || y as usize >= input.len() {
                break;
            }
            
            match input[y as usize][x as usize] {
                Space::Occupied => {
                    count += 1;
                    looking = false;
                }
                Space::Empty => looking = false,
                Space::Floor => {},
            }
            distance += 1;
        }
    }

    count
}

fn count_occupied(input: &[Vec<Space>]) -> usize {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == Space::Occupied {
                count += 1;
            }
        }
    }

    count
}

#[allow(dead_code)]
fn display(input: &[Vec<Space>]) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            match input[y][x] {
                Space::Floor => print!("."),
                Space::Empty => print!("L"),
                Space::Occupied => print!("#"),
            }
        }
        println!();
    }
    println!();
}