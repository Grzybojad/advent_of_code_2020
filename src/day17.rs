#[derive(PartialEq, Copy, Clone)]
pub struct Cube {
    pos: V3,
    active: bool
}

#[derive(PartialEq, Copy, Clone)]
pub struct V3 {
    x: i32,
    y: i32,
    z: i32,
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Cube> {
    let mut x;
    let mut y = 0;
    let mut cubes = Vec::new();
    
    // Get cubes from input
    for l in input.lines() {
        x = 0;
        for c in l.chars() {
            match c {
                '#' => cubes.push(Cube{pos: V3{x, y, z: 0}, active: true}),
                '.' => cubes.push(Cube{pos: V3{x, y, z: 0}, active: false}),
                _ => panic!()
            }
            x += 1;
        }
        y += 1;
    }

    cubes
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &[Cube]) -> usize {
    let mut cubes = add_padding(&input);
    
    for _i in 0..6 {
        cubes = step_simulation(&cubes);
    }
    
    cubes.into_iter().filter(|c| c.active).count()
}

fn step_simulation(input: &[Cube]) -> Vec<Cube> {
    let mut updated_cubes = Vec::new();

    input
        .into_iter()
        .for_each(|&c| {
            match c.active { 
                true => {
                    match nr_of_neighbours(input, &c.pos) {
                        2 | 3 => updated_cubes.push(Cube{pos: c.pos, active: true}),
                        _ => updated_cubes.push(Cube{pos: c.pos, active: false}),
                    }
                }
                false => {
                    match nr_of_neighbours(input, &c.pos) {
                        3 => updated_cubes.push(Cube{pos: c.pos, active: true}),
                        _ => updated_cubes.push(Cube{pos: c.pos, active: false}),
                    }
                }
            }
        });

    add_padding(&updated_cubes)
}

fn nr_of_neighbours(input: &[Cube], pos: &V3) -> i32 {
    let mut count = 0;
    for z in pos.z-1..=pos.z+1 {
        for y in pos.y-1..=pos.y+1 {
            for x in pos.x-1..=pos.x + 1 {
                if pos != &(V3{x, y, z}) && input.contains(&Cube { pos: V3{x, y, z}, active: true }) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn add_padding(input: &[Cube]) -> Vec<Cube> {
    let mut updated_cubes = Vec::new();
    
    input
        .into_iter()
        .for_each(|c| {
            if c.active {
                for z in c.pos.z-1..=c.pos.z+1 {
                    for y in c.pos.y-1..=c.pos.y+1 {
                        for x in c.pos.x-1..=c.pos.x+1 {
                            if !updated_cubes.contains(&Cube { pos: V3{x, y, z}, active: true }) && !updated_cubes.contains(&Cube { pos: V3{x, y, z}, active: false }) {
                                updated_cubes.push(Cube { 
                                    pos: V3{x, y, z},
                                    active: input.contains(&Cube { pos: V3{x, y, z}, active: true })
                                });
                            }
                        }
                    }
                }
            }
        });
    
    updated_cubes
}