﻿#[derive(PartialEq, Copy, Clone)]
pub struct Cube {
    // pos: V3,
    pos: V4,
    active: bool
}

// Commented out part 1 stuff, it's the same as part2, just switch V4 to V3 and remove all the w component stuff
// #[derive(PartialEq, Copy, Clone)]
// pub struct V3 {
//     x: i32,
//     y: i32,
//     z: i32,
// }

#[derive(PartialEq, Copy, Clone)]
pub struct V4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
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
                '#' => cubes.push(Cube{pos: V4 { x, y, z: 0, w: 0 }, active: true}),
                '.' => cubes.push(Cube{pos: V4 { x, y, z: 0, w: 0 }, active: false}),
                _ => panic!()
            }
            x += 1;
        }
        y += 1;
    }

    cubes
}

// #[aoc(day17, part1)]
// pub fn solve_part1(input: &[Cube]) -> usize {
//     let mut cubes = add_padding(&input);
//     
//     for _i in 0..6 {
//         cubes = step_simulation(&cubes);
//     }
//     
//     cubes.into_iter().filter(|c| c.active).count()
// }

#[aoc(day17, part2)]
pub fn solve_part2(input: &[Cube]) -> usize {
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

fn nr_of_neighbours(input: &[Cube], pos: &V4) -> i32 {
    let mut count = 0;
    for w in pos.w-1..=pos.w+1 {
        for z in pos.z - 1..=pos.z + 1 {
            for y in pos.y - 1..=pos.y + 1 {
                for x in pos.x - 1..=pos.x + 1 {
                    if pos != &(V4 { x, y, z, w }) && input.contains(&Cube { pos: V4 { x, y, z, w }, active: true }) {
                        count += 1;
                    }
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
                for w in c.pos.w-1..=c.pos.w+1 {
                    for z in c.pos.z - 1..=c.pos.z + 1 {
                        for y in c.pos.y - 1..=c.pos.y + 1 {
                            for x in c.pos.x - 1..=c.pos.x + 1 {
                                if !updated_cubes.contains(&Cube { pos: V4 { x, y, z, w }, active: true }) && !updated_cubes.contains(&Cube { pos: V4 { x, y, z, w }, active: false }) {
                                    updated_cubes.push(Cube {
                                        pos: V4 { x, y, z, w },
                                        active: input.contains(&Cube { pos: V4 { x, y, z, w }, active: true })
                                    });
                                }
                            }
                        }
                    }
                }
            }
        });
    
    updated_cubes
}