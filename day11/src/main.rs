use aocf::Aoc;
use day11::calculate_taken_seats;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(11))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let answer = calculate_taken_seats(&input.trim(), true);
        println!("Part I: {}", answer);

        let answer = calculate_taken_seats(&input.trim(), false);
        println!("Part II: {}", answer);
    }
}

