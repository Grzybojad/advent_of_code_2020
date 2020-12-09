#[derive(Default)]
pub struct Instruction {
    oper: String,
    arg: i32,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    input
        .lines()
        .for_each(|l| {
            let mut inst = Instruction::default();
            
            let mut split_inst = l.split_whitespace();

            inst.oper = split_inst.next().unwrap().to_string();

            let arg_text = split_inst.next().unwrap();
            let sign = arg_text.chars().next().unwrap();
            let nr: i32 = arg_text[1..].parse().unwrap();
            
            inst.arg = nr * match sign {
                '+' => 1,
                '-' => -1,
                _ => 0,
            };
            
            instructions.push(inst);
        });

    instructions
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut visited: Vec<bool> = Vec::new();
    for _i in 0..input.len() {
        visited.push(false);
    }
    
    let mut acc = 0;
    let mut pc = 0;
    
    while !visited[pc] {
        visited[pc] = true;
        
        match input[pc].oper.as_str() {
            "acc" => acc += input[pc].arg,
            "jmp" => pc += (input[pc].arg as usize) - 1,
            _ => {},
        }
        pc += 1;
    }
    acc
}