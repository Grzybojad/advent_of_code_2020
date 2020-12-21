#[derive(Default)]
pub struct Tile {
    id: i32,
    sides: [Vec<bool>; 4], // 0: top, 1: right, 2: bottom, 3: left
    pieces: Vec<Vec<bool>>
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .into_iter()
        .map(|s| {
            let mut tile = Tile::default();
            
            let lines: Vec<&str> = s.lines().collect();
            let mut lines_iter = lines.into_iter();
            
            let mut id_text = lines_iter.next().unwrap().split_whitespace().next_back().unwrap().to_string();
            id_text.pop();
            tile.id = id_text.parse().unwrap();
            
            let mut y = 0;
            while let Some(l) = lines_iter.next() {
                let mut line = Vec::new();

                let mut x = 0;
                l.chars().for_each(|c| {
                    let shape = match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!(),
                    };
                    
                    if y == 0                   { tile.sides[0].push(shape); }
                    if x == l.len()-1           { tile.sides[1].push(shape); }
                    if y == lines_iter.len()-1  { tile.sides[2].push(shape); }
                    if x == 0                   { tile.sides[3].push(shape); }

                    line.push(shape);
                    x += 1;
                });
                tile.pieces.push(line);
                
                y += 1;
            }
            tile
        }).collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[Tile]) -> usize {
    input.len()
}