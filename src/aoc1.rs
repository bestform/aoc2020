use std::fs;

pub(crate) fn run() {
    let contents = fs::read_to_string("src/input/01a.txt")
        .expect("Something went wrong reading the file");

    let numbers_as_strings = contents.split("\n");
    let numbers = numbers_as_strings.flat_map(|n| n.parse::<i32>()).collect::<Vec<i32>>();

    let total = numbers.len();
    for i in 0..total - 1 {
        for j in i + 1..total - 1 {
            let a = numbers[i];
            let b = numbers[j];
            if a + b == 2020 {
                println!("{} + {} = 2020", a, b);
                println!(" => {}", a * b);
            }
        }
    }

    for i in 0..total - 1 {
        for j in i + 1..total - 1 {
            for k in j + 1..total - 1 {
                let a = numbers[i];
                let b = numbers[j];
                let c = numbers[k];
                if a + b + c == 2020 {
                    println!("{} + {} + {} = 2020", a, b, c);
                    println!(" => {}", a * b * c);
                }
            }
        }
    }
}
