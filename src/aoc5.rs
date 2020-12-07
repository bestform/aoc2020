use std::fs;

pub(crate) fn run() {
    let contents = fs::read_to_string("src/input/05a.txt")
        .expect("Something went wrong reading the file");

    let mut highest_seat = 0;
    let mut found_seats = vec![];

    for input in contents.lines() {
        let seat = find_seat(input, 128, 8);
        if seat > highest_seat {
            highest_seat = seat;
        }
        found_seats.push(seat);
    }
    found_seats.sort();
    let mut current_seat = 0;
    for seat in found_seats {
        if seat != current_seat + 1 {
            println!("missing: {}", current_seat + 1);
        }
        current_seat = seat;
    }

    println!("Highest seat: {}", highest_seat);
}

fn find_seat(seq: &str, total_rows: i32, total_seats: i32) -> i32 {
    let mut current_row_low = 0;
    let mut current_row_high = total_rows - 1;

    let mut current_seat_low = 0;
    let mut current_seat_high = total_seats - 1;


    for c in seq.chars() {
        match c {
            'B' => {
                current_row_low = (current_row_high - current_row_low) / 2 + current_row_low + 1;
            }
            'F' => {
                current_row_high = (current_row_high - current_row_low) / 2 + current_row_low;
            }
            'R' => {
                current_seat_low = (current_seat_high - current_seat_low) / 2 + current_seat_low + 1;
            }
            'L' => {
                current_seat_high = (current_seat_high - current_seat_low) / 2 + current_seat_low;
            }
            _ => {
                println!("Unexpected: {}", c);
                panic!()
            }
        }

        //println!("{}: low: {}, high: {}", c, current_seat_low, current_seat_high);

        if current_row_low == current_row_high && current_seat_low == current_seat_high {
            return current_row_low * 8 + current_seat_low;
        }
    }

    println!("could not find seat for {}", seq);
    panic!();
}

#[test]
fn example() {
    assert_eq!(find_seat("FBFBBFFRLR", 128, 8), 357);
}

