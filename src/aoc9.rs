use crate::rd;
use std::borrow::Borrow;

pub(crate) fn run() {
    let content = rd!("src/input/09a.txt");
    let stream = code_stream_from_input(&*content, 25);

    let res = first_invalid_number(stream.borrow());

    match res {
        None => { println!("Stream is all valid.")}
        Some(n) => {
            println!("Found invalid number: {}", n);
            let sum = find_contiguous_numbers(stream.borrow(), n);
            match sum {
                None => { println!("Did not found numbers that sum up to {}", n)}
                Some(mut numbers) => {
                    println!("Found {:?}", numbers);
                    numbers.sort();
                    println!("Sum of smallest and largest: {}", numbers.first().unwrap() + numbers.last().unwrap());

                }
            }
        }
    }

}

struct CodeStream {
    numbers: Vec<i64>,
    preamble: i64
}

fn first_invalid_number(stream:&CodeStream) -> Option<i64> {
    let preamble = stream.preamble.clone() as usize;
    let numbers = stream.numbers.clone();

    for (pos, number) in numbers.clone().into_iter().enumerate() {
        if pos < preamble {
            continue
        }
        let sums = possible_sums(numbers[pos-preamble..pos].to_vec());
        if !sums.contains(&number) {
            return Some(number);
        }

    }
    None
}

fn find_contiguous_numbers(stream:&CodeStream, sum:i64) -> Option<Vec<i64>> {
    let numbers = stream.numbers.clone();
    for last in 0..numbers.len() {
        let mut current_sum = 0;
        for first in (0..=last).rev() {
            current_sum += numbers[first];
            if current_sum == sum {
                return Some(numbers.clone()[first..=last].to_vec());
            }
        }
    }

    None
}

fn possible_sums(candidates: Vec<i64>) -> Vec<i64> {
    let mut sums:Vec<i64> = Vec::new();
    for i in 0..candidates.len() {
        for j in i..candidates.len() {
            if candidates[j] == candidates[i] {
                continue;
            }
            sums.push((candidates[j]+candidates[i]) as i64);
        }
    }

    sums
}

fn code_stream_from_input(input: &str, preamble: i64) -> CodeStream {
    let lines = input.lines();
    let mut numbers:Vec<i64> = Vec::new();
    for line in lines {
        numbers.push(line.trim().parse::<i64>().unwrap());
    }

    CodeStream {
        preamble,
        numbers
    }
}

#[test]
fn test_code_stream_creation() {
    let stream = code_stream_from_input("1\n2\n3", 1);
    assert_eq!(stream.preamble, 1);
    assert_eq!(stream.numbers, vec![1, 2, 3]);
}

#[test]
fn test_find_first_invalid_number() {
    let stream = code_stream_from_input("35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576", 5);

    let invalid = first_invalid_number(&stream);
    assert_eq!(invalid, Some(127));
}

#[test]
fn test_find_contiguous() {
    let stream = code_stream_from_input("35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576", 5);

    let numbers = find_contiguous_numbers(&stream, 127);

    assert_eq!(numbers, Some(vec![15, 25, 47, 40]));
}