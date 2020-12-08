use std::collections::HashMap;
use crate::rd;

pub(crate) fn run() {
    let contents = rd!("src/input/06a.txt");

    let re = regex::Regex::new(r"(?m)^\s*\n").unwrap();
    let groups = re.split(&*contents);

    let mut total = 0;
    let mut total2 = 0;
    for group in groups {
        total += count_unique_answers(group.trim());
        total2 += count_everyone_for_yes(group.trim());
    }

    println!("Total: {}", total);
    println!("Total2: {}", total2);
}

fn count_unique_answers(input: &str) -> usize {
    let mut answers = HashMap::new();
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        answers.insert(c, 1);
    }

    return answers.len();
}

fn count_everyone_for_yes(input: &str) -> usize {
    let mut answers = HashMap::new();
    let mut groups = 1;
    for c in input.chars() {
        if c == '\n' {
            groups += 1;
            continue;
        }

        if let Some(a) = answers.insert(c, 1) {
           answers.insert(c, a + 1);
        }
    }

    return answers.values().fold(0, |a, v| if v == &groups { a + 1 } else { a });
}

#[test]
fn count_unique_test() {
    assert_eq!(count_unique_answers("ab\nac\na"), 3);
}

#[test]
fn count_everyone_yes() {
    assert_eq!(count_everyone_for_yes("ab\nac\na"), 1);
}