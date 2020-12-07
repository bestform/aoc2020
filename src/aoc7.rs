use std::collections::HashMap;
use std::fs;

pub(crate) fn run() {
    let contents = fs::read_to_string("src/input/07a.txt")
        .expect("Something went wrong reading the file");

    let mut rules:Vec<Rule> = vec![];
    for input in contents.lines() {
        let rule = rule_from_str(input.trim());
        rules.push(rule);
    }

    let contained = contained_in(&rules, "shiny gold".to_string());
    let has_contained_bags = has_bags_inside(&rules, "shiny gold".to_string());
    println!("Contained in {} bags.", contained.len());
    println!("Has {} bags inside.", has_contained_bags);
}

#[derive(Debug)]
struct Rule {
    color: String,
    contains: HashMap<String, i32>
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color &&
            self.contains == other.contains
    }
}

fn rule_from_str(input: &str) -> Rule {
    let mut rule = Rule{ color: "".to_string(), contains: HashMap::new() };
    let parts = input.split(" contain ")
        .collect::<Vec<&str>>();

    rule.color = parts.get(0)
        .unwrap()
        .replace(" bags", "")
        .to_string();

    if parts.get(1).unwrap() == &"no other bags." {
        return rule;
    }

    let containing_parts = parts.get(1).unwrap().split(", ");

    for p in containing_parts {
        let mut parts = p.split(" ");
        let num = parts.next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let color = parts.collect::<Vec<_>>()
            .join(" ")
            .replace(" bags", "")
            .replace(" bag", "")
            .replace(".", "")
            .to_string();
        rule.contains.insert(color, num);
    }

    return rule;
}

fn has_bags_inside(rules: &Vec<Rule>, color: String) -> i32 {
    let mut total = 0;
    for r in rules {
        if r.color == color {
            for c in &r.contains {
                total += c.1;
                total += has_bags_inside(rules, c.0.to_string()) * c.1;
            }
        }
    }

    return total;
}

fn contained_in(rules: &Vec<Rule>, color: String) -> Vec<String> {
    let mut containing_colors: Vec<String> = vec![];

    for rule in rules {
        if rule.contains
            .keys()
            .find(|v| v.to_string() == color.to_string())
            .is_some() {
            containing_colors.push(rule.color.to_string());
            containing_colors.extend(contained_in(rules, rule.color.to_string()));
        }
    }

    let mut rule_hash:HashMap<String, i32> = HashMap::new();
    for r in containing_colors {
        rule_hash.insert(r.to_string(), 1);
    }

    return rule_hash.keys().map(|v| v.to_string()).collect();
}

#[test]
fn test_rule_without_containing() {
    assert_eq!(rule_from_str("faded blue bags contain no other bags."),
        Rule{
            color: "faded blue".to_string(),
            contains: HashMap::new()
        }
    );
}

#[test]
fn test_rule_containing() {
    let mut expected_contains = HashMap::new();
    expected_contains.insert("faded blue".to_string(), 5);
    expected_contains.insert("dotted black".to_string(), 6);
    assert_eq!(rule_from_str("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
               Rule{
                   color: "vibrant plum".to_string(),
                   contains: expected_contains
               }
    );
}

#[test]
fn test_rule_containing_1() {
    let mut expected_contains = HashMap::new();
    expected_contains.insert("faded blue".to_string(), 1);
    assert_eq!(rule_from_str("vibrant plum bags contain 1 faded blue bag."),
               Rule{
                   color: "vibrant plum".to_string(),
                   contains: expected_contains
               }
    );

}#[test]
fn test_found_bug() {
    let mut expected_contains = HashMap::new();
    expected_contains.insert("mirrored chartreuse".to_string(), 3);
    expected_contains.insert("faded maroon".to_string(), 5);
    expected_contains.insert("dotted magenta".to_string(), 3);
    expected_contains.insert("bright brown".to_string(), 4);
    let rule = rule_from_str("drab crimson bags contain 3 mirrored chartreuse bags, 5 faded maroon bags, 3 dotted magenta bags, 4 bright brown bags.");
    assert_eq!(rule,
               Rule{
                   color: "drab crimson".to_string(),
                   contains: expected_contains
               }
    );
}

#[test]
fn example() {
    let mut rules: Vec<Rule> = vec![];
    rules.push(rule_from_str("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."));
    rules.push(rule_from_str("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."));
    rules.push(rule_from_str("light red bags contain 1 bright white bag, 2 muted yellow bags."));
    rules.push(rule_from_str("dark orange bags contain 3 bright white bags, 4 muted yellow bags."));
    rules.push(rule_from_str("bright white bags contain 1 shiny gold bag."));
    rules.push(rule_from_str("dark olive bags contain 3 faded blue bags, 4 dotted black bags."));
    rules.push(rule_from_str("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."));
    rules.push(rule_from_str("faded blue bags contain no other bags."));
    rules.push(rule_from_str("dotted black bags contain no other bags."));

    assert_eq!(contained_in(&rules, "shiny gold".to_string()).len(), 4);
}

#[test]
fn example2() {
    let mut rules: Vec<Rule> = vec![];
    rules.push(rule_from_str("shiny gold bags contain 2 dark red bags."));
    rules.push(rule_from_str("dark red bags contain 2 dark orange bags."));
    rules.push(rule_from_str("dark orange bags contain 2 dark yellow bags."));
    rules.push(rule_from_str("dark yellow bags contain 2 dark green bags."));
    rules.push(rule_from_str("dark green bags contain 2 dark blue bags."));
    rules.push(rule_from_str("dark blue bags contain 2 dark violet bags."));
    rules.push(rule_from_str("dark violet bags contain no other bags."));

    assert_eq!(has_bags_inside(&rules, "shiny gold".to_string()), 126);
}
