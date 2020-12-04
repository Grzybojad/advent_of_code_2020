use regex::Regex;

#[derive(Debug, Default, Clone)]
pub struct Passport {
    byr: i32, // (Birth Year)
    iyr: i32, // (Issue Year)
    eyr: i32, // (Expiration Year)
    hgt: String, // (Height)
    hcl: String, // (Hair Color)
    ecl: String, // (Eye Color)
    pid: String, // (Passport ID)
    cid: String, // (Country ID)
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> { 
    let mut passports = Vec::new();
    
    input
        .split("\n\n" ) // Get the passports
        .for_each(|pass_data| {
            let mut passport: Passport = Passport::default();
            
            pass_data
                .split_whitespace()
                .for_each(|elem| {
                    let elem_data: Vec<&str> = elem.split(':').collect();
                    
                    match elem_data[0] {
                        "byr" => passport.byr = elem_data[1].parse::<i32>().unwrap(),
                        "iyr" => passport.iyr = elem_data[1].parse::<i32>().unwrap(),
                        "eyr" => passport.eyr = elem_data[1].parse::<i32>().unwrap(),
                        "hgt" => passport.hgt = elem_data[1].to_string(),
                        "hcl" => passport.hcl = elem_data[1].to_string(),
                        "ecl" => passport.ecl = elem_data[1].to_string(),
                        "pid" => passport.pid = elem_data[1].to_string(),
                        "cid" => passport.cid = elem_data[1].to_string(),
                        _ => {}
                    }
                });

            passports.push(passport);
        });

    passports
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Passport]) -> usize {
    input.into_iter().filter(|p| {
        p.byr != 0 && 
        p.iyr != 0 &&
        p.eyr != 0 &&
        p.hgt != "" &&
        p.hcl != "" &&
        p.ecl != "" &&
        p.pid != ""
    } ).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Passport]) -> usize {
    input.into_iter().filter(|p| {
        let re_hcl = Regex::new(r"^#[0-9a-f]{6}").unwrap();

        byr_check(p.byr) && 
        iyr_check(p.iyr) &&
        eyr_check( p.eyr ) &&
        hgt_check(p.hgt.as_str()) &&
        re_hcl.is_match(&p.hcl) &&
        ecl_check( &p.ecl.as_str()) &&
        is_string_number(&p.pid) && p.pid.len() == 9
    } ).count()
}

fn is_string_number(input: &String) -> bool {
    input.chars().all(|c| c.is_numeric())
}

fn byr_check(input: i32) -> bool { input >= 1920 && input <= 2002 }
fn iyr_check(input: i32) -> bool { input >= 2010 && input <= 2020 }
fn eyr_check(input: i32) -> bool { input >= 2020 && input <= 2030 }

fn hgt_check(input: &str) -> bool {
    let mut hgt_value = 0;
    let mut it = input.chars().peekable();
    
    while let Some('0'..='9') = it.peek() {
        let c = it.next().unwrap();
        hgt_value *= 10;
        hgt_value += (c as u8 - b'0') as i32;
    }

    let hgt_measure: String = input.chars().rev().take(2).collect();
    
    match hgt_measure.as_str() {
        "mc" => hgt_value >= 150 && hgt_value <= 193,   // cm
        "ni" => hgt_value >= 59 && hgt_value <= 76,     // in
        _ => false,
    }
}

fn ecl_check(input: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
}
