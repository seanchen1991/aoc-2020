#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seat {
    Empty,
    Taken,
    Noop,
}

impl Seat {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Taken,
            _   => Seat::Noop,
        }
    }
}

pub struct Layout {
    width: usize,
    height: usize,
    seats: Vec<Vec<Seat>>
}

impl Layout {
    fn from_input(input: &str) -> Self {
        let seats: Vec<Vec<Seat>> = input.lines()
            .map(|line| {
                line.chars()
                    .fold(Vec::new(), |mut acc, c| {
                        acc.push(Seat::from_char(c));
                        acc
                    })
            })
            .collect();

        Layout {
            height: seats.len(),
            width: seats[0].len(),
            seats: seats,
        }
    }

    fn get_adjacent_taken_seats(&self, row: usize, col: usize) -> usize {
        let mut taken = 0usize; 
        
        if row > 0 {
            let n = self.seats[row-1][col];
            taken += if n == Seat::Taken { 1 } else { 0 };
        }
        
        if row > 0 && col > 0 {
            let nw = self.seats[row-1][col-1];
            taken += if nw == Seat::Taken { 1 } else { 0 };
        }

        if row > 0 && col < self.width - 1 {
            let ne = self.seats[row-1][col+1];
            taken += if ne == Seat::Taken { 1 } else { 0 };
        }
        
        if row < self.height - 1 {
            let s = self.seats[row+1][col];
            taken += if s == Seat::Taken { 1 } else { 0 };
        }
        
        if row < self.height - 1 && col > 0 {
            let sw = self.seats[row+1][col-1];
            taken += if sw == Seat::Taken { 1 } else { 0 };
        }
        
        if row < self.height - 1 && col < self.width - 1 {
            let se = self.seats[row+1][col+1];
            taken += if se == Seat::Taken { 1 } else { 0 };
        }
        
        if col < self.width - 1 {
            let e = self.seats[row][col+1];
            taken += if e == Seat::Taken { 1 } else { 0 };
        }
        
        if col > 0 {
            let w = self.seats[row][col-1];
            taken += if w == Seat::Taken { 1 } else { 0 };
        }

        taken
    }

    fn get_all_taken_seats(&self) -> usize {
        self.seats.iter()
            .fold(0, |acc, row| {
                acc + row.iter()
                    .fold(0, |acc, loc| {
                        acc + if *loc == Seat::Taken { 1 } else { 0 }
                    })
            })
    }
    
    // Returns whether any seats were toggled
    fn tick(&mut self) -> bool {
        let mut toggled = false;
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let seat = self.seats[row][col];
                let taken_adjacent_seats = self.get_adjacent_taken_seats(row, col);

                let next_seat = match (seat, taken_adjacent_seats) {
                    // Rule 1: If a seat if empty (L) and there are no occupied
                    // seats adjacent to it, the seat becomes occupied
                    (Seat::Empty, 0) => {
                        toggled = true;
                        Seat::Taken
                    },
                    // Rule 2: If a seat is occupied and four or more seats 
                    // adjacent to it are also occupied, the seat becomes empty
                    (Seat::Taken, x) if x >= 4 => {
                        toggled = true;
                        Seat::Empty
                    },
                    // Rule 3: Otherwise, the seat's state doesn't change 
                    (otherwise, _) => otherwise,
                };

                next[row][col] = next_seat;
            }
        }

        self.seats = next;
        toggled
    }

    fn run_to_completion(&mut self) {
        loop {
            if !self.tick() {
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut layout = Layout::from_input(input); 
    layout.run_to_completion();
    layout.get_all_taken_seats()
}

#[cfg(test)]
#[test]
fn test_counting_adjacent_seats() {
    let test_input = 
"#.##.
#####";

    let test_layout = Layout::from_input(test_input.trim());

    assert_eq!(test_layout.get_adjacent_taken_seats(0, 0), 2);
    assert_eq!(test_layout.get_adjacent_taken_seats(0, 2), 4);
    assert_eq!(test_layout.get_adjacent_taken_seats(0, 3), 4);
}

#[test]
fn test_counting_all_taken_seats() {
    let test_input =
"#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";
    
    let layout = Layout::from_input(test_input.trim());
    assert_eq!(layout.get_all_taken_seats(), 30);
}

#[test]
fn test_tick_one() {
    let test_input =
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    let expected =
"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    let mut test_layout = Layout::from_input(test_input.trim());
    let expected_layout = Layout::from_input(expected.trim());

    let tick = test_layout.tick();
    
    assert_eq!(tick, true);
    assert_eq!(test_layout.seats, expected_layout.seats);
}

#[test]
fn test_tick_two() {
    let test_input =
"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    let expected =
"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

    let mut test_layout = Layout::from_input(test_input.trim());
    let expected_layout = Layout::from_input(expected.trim());

    let tick = test_layout.tick();
    
    assert_eq!(tick, true);
    assert_eq!(test_layout.seats, expected_layout.seats);
}

#[test]
fn test_part_one_runs_to_completion() {
    let test_input =
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    let expected =
"#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    let mut test_layout = Layout::from_input(test_input.trim());
    let expected_layout = Layout::from_input(expected.trim());

    test_layout.run_to_completion();

    assert_eq!(test_layout.seats, expected_layout.seats);
}

#[test]
fn test_part_one() {
    let test_input = 
"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    assert_eq!(part_one(test_input.trim()), 37);
}

