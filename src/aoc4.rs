use std::fs;

extern crate regex;


pub(crate) fn run() {
    let contents = fs::read_to_string("src/input/04a.txt")
        .expect("Something went wrong reading the file");

    let re = regex::Regex::new(r"(?m)^\s*\n").unwrap();
    let passports = re.split(&*contents);

    let mandatory_names = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "ecl", "pid"];

    let mut valid = 0;
    for passport in passports {
        let passport_line = passport.replace("\n", " ");
        let fields = passport_line.split(" ");
        let mut names = vec![];
        for field in fields {
            let parts = field.split(":").collect::<Vec<&str>>();
            if parts.len() != 2 {
                continue;
            }
            let name = parts[0];
            let value = parts[1];
            for mandatory_name in &mandatory_names {
                if &name == mandatory_name {
                    if is_valid(name, value) {
                        names.push(name);
                    }

                }
            }
        }

        if names.len() == mandatory_names.len() {
            valid += 1;
        }
    }

    println!("Valid passports: {}", valid);
}

fn is_valid(name: &str, value: &str) -> bool {
    match name {
        "byr" => return value.len() == 4 && value.parse::<i32>().unwrap() >= 1920 && value.parse::<i32>().unwrap() <= 2002,
        "iyr" => return value.len() == 4 && value.parse::<i32>().unwrap() >= 2010 && value.parse::<i32>().unwrap() <= 2020,
        "eyr" => return value.len() == 4 && value.parse::<i32>().unwrap() >= 2020 && value.parse::<i32>().unwrap() <= 2030,
        "hgt" => {
            if value.ends_with("cm") {
                let number_str = value.split("cm").collect::<Vec<&str>>()[0];
                let re = regex::Regex::new(r"^[0-9]*$").unwrap();
                if !re.is_match(number_str) {
                    return false;
                }
                return number_str.parse::<i32>().unwrap() >= 150 && number_str.parse::<i32>().unwrap() <= 193;
            }
            if value.ends_with("in") {
                let number_str = value.split("in").collect::<Vec<&str>>()[0];
                let re = regex::Regex::new(r"^[0-9]*$").unwrap();
                if !re.is_match(number_str) {
                    return false;
                }
                return number_str.parse::<i32>().unwrap() >= 59 && number_str.parse::<i32>().unwrap() <= 76;
            }
            return false;
        }
        "hcl" => {
            let re = regex::Regex::new(r"^#[0-9,a-f]{6}$").unwrap();
            return re.is_match(value);

        }
        "ecl" => {
            let re = regex::Regex::new(r"(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)").unwrap();
            return re.is_match(value);
        }
        "pid" => {
            let re = regex::Regex::new(r"^[0-9]{9}$").unwrap();
            return re.is_match(value);
        }

        _ => panic!()
    }
}
