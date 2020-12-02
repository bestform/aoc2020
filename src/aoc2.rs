use std::fs;

pub(crate) fn run() {
    run1();
    run2();
}

fn run1() {
    let contents = fs::read_to_string("src/input/02a.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    let mut valid = 0;
    for l in lines {
        if is_valid(l) {
            valid += 1;
        }
    }

    println!("{} valid passwords for part 1", valid);
}

fn run2() {
    let contents = fs::read_to_string("src/input/02a.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    let mut valid = 0;
    for l in lines {
        if is_valid2(l) {
            valid += 1;
        }
    }

    println!("{} valid passwords for part 2", valid);
}

struct Rule {
    min: usize,
    max: usize,
}

fn is_valid(line: &str) -> bool {
    let parts = line.split(":").collect::<Vec<&str>>();

    if parts.len() != 2 {
        return false;
    }

    let rule = parts.get(0).unwrap();
    // extract rule
    let rule_parts = rule.split(" ").collect::<Vec<&str>>();
    let from_to = rule_parts[0];
    let char_part = rule_parts[1];
    let from_to_parts = from_to.split("-").collect::<Vec<&str>>();

    let rule = Rule {
        min: from_to_parts.get(0).unwrap().parse::<usize>().unwrap(),
        max: from_to_parts.get(1).unwrap().parse::<usize>().unwrap(),
    };
    let password = parts.get(1).unwrap();
    let count = password.matches(char_part).count();

    return count >= rule.min && count <= rule.max;
}

fn is_valid2(line: &str) -> bool {
    let parts = line.split(":").collect::<Vec<&str>>();

    if parts.len() != 2 {
        return false;
    }

    let rule = parts.get(0).unwrap();
    // extract rule
    let rule_parts = rule.split(" ").collect::<Vec<&str>>();
    let from_to = rule_parts[0];
    let char_part = rule_parts[1];
    let from_to_parts = from_to.split("-").collect::<Vec<&str>>();

    let rule = Rule {
        min: from_to_parts.get(0).unwrap().parse::<usize>().unwrap(),
        max: from_to_parts.get(1).unwrap().parse::<usize>().unwrap(),
    };
    let password = parts.get(1).unwrap().trim();
    let mut found = 0;
    if password.get(rule.min - 1..rule.min).unwrap().eq(char_part) {
        found += 1
    }
    if password.get(rule.max - 1..rule.max).unwrap().eq(char_part) {
        found += 1
    }

    return found == 1;
}
