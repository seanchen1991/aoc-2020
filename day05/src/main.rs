use aocf::Aoc;
use std::collections::HashSet;

const MAX_ROW: usize = 127;
const MAX_COL: usize = 7;

fn get_seat_id(pass: &str) -> usize {
    let row = pass.chars()
        .take(7)
        .fold((0, MAX_ROW), wittle_down);

    let col = pass.chars()
        .skip(7)
        .take(3)
        .fold((0, MAX_COL), wittle_down);

    row.0 * 8 + col.0
}

fn wittle_down(range: (usize, usize), chr: char) -> (usize, usize) {
    let midpoint = (range.1 - range.0) / 2;

    match chr {
        'F' | 'L' => (range.0, range.1 - midpoint - 1),
        'B' | 'R' => (range.0 + midpoint + 1, range.1),
        _ => range,
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(5))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let seat_ids = i.lines().map(get_seat_id);

        // Part 1
        let max_seat_id = seat_ids.clone().max().unwrap();
        println!("{}", max_seat_id);

        // Part 2
        let min_seat_id = seat_ids.clone().min().unwrap();

        let all_seat_ids: HashSet<_> = (0..(MAX_ROW * 8)).collect();
        let taken_seats: HashSet<_> = seat_ids.collect();
        
        let diff: Vec<&usize> = all_seat_ids.difference(&taken_seats)
            .filter(|d| **d > min_seat_id && **d < max_seat_id)
            .collect();

        println!("{:?}", diff);
    }
}

#[cfg(test)]
#[test]
fn test_get_seat_id() {
    let test_input = "FBFBBFFRLR";
    assert_eq!(get_seat_id(test_input), 357);

    let test_input = "BFFFBBFRRR";
    assert_eq!(get_seat_id(test_input), 567);

    let test_input = "FFFBBBFRRR";
    assert_eq!(get_seat_id(test_input), 119);

    let test_input = "BBFFBBFRLL";
    assert_eq!(get_seat_id(test_input), 820);
}
