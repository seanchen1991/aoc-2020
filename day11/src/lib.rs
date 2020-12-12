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
    
    fn north_seat(&self, row: usize, col: usize) -> (i32, i32) {
        if row == 0 { return (-1, -1); }

        let mut north_los = row;
        
        while north_los > 0 {
            north_los -= 1;
            if self.is_seat_taken(north_los, col) {
                return (north_los as i32, col as i32);
            }
        }

        (-1, -1)
    }

    fn south_seat(&self, row: usize, col: usize) -> (i32, i32) {
        let mut south_los = row;

        while south_los < self.height - 1 {
            south_los += 1;
            if self.is_seat_taken(south_los, col) {
                return (south_los as i32, col as i32);
            } 
        }

        (-1, -1)
    }

    fn east_seat(&self, row: usize, col: usize) -> (i32, i32) {
        let mut east_los = col;

        while east_los < self.width - 1 {
            east_los += 1;
            if self.is_seat_taken(row, east_los) {
                return (row as i32, east_los as i32);
            }
        }
        
        (-1, -1)
    }

    fn west_seat(&self, row: usize, col: usize) -> (i32, i32) {
        if col == 0 { return (-1, -1); }

        let mut west_los = col;
        
        while west_los > 0 {
            west_los -= 1;
            if self.is_seat_taken(row, west_los) {
                return (row as i32, west_los as i32);
            }
        }
        
        (-1, -1)
    }

    fn nw_seat(&self, row: usize, col: usize) -> (i32, i32) {
        if row == 0 || col == 0 { return (-1, -1); }

        let mut north_los = row;
        let mut west_los = col;

        while north_los > 0 && west_los > 0 {
            north_los -= 1;
            west_los -= 1;
            if self.is_seat_taken(north_los, west_los) {
                return (north_los as i32, west_los as i32);
            } 
        }

        (-1, -1)
    } 

    fn ne_seat(&self, row: usize, col: usize) -> (i32, i32) {
        if row == 0 || col == self.width - 1 {
            return (-1, -1);
        } 

        let mut north_los = row;
        let mut east_los = col;

        while north_los > 0 && east_los < self.width - 1 {
            north_los -= 1;
            east_los += 1;
            if self.is_seat_taken(north_los, east_los) {
                return (north_los as i32, east_los as i32);
            }
        }

        (-1, -1)
    }

    fn sw_seat(&self, row: usize, col: usize) -> (i32, i32) {
        if row == self.height - 1 || col == 0 {
            return (-1, -1);
        } 

        let mut south_los = row;
        let mut west_los = col;

        while south_los < self.height - 1 && west_los > 0 {
            south_los += 1;
            west_los -= 1;
            if self.is_seat_taken(south_los, west_los) {
                return (south_los as i32, west_los as i32);
            }
        }

        (-1, -1)
    }

    fn se_seat(&self, row: usize, col: usize) -> (i32, i32) {
        if row == self.height - 1 || col == self.width - 1 {
            return (-1, -1);
        }

        let mut south_los = row;
        let mut east_los = col;

        while south_los < self.height - 1 && east_los < self.width - 1 {
            south_los += 1;
            east_los += 1;
            if self.is_seat_taken(south_los, east_los) {
                return (south_los as i32, east_los as i32);
            }
        }

        (-1, -1)
    }

    fn is_seat_taken(&self, row: usize, col: usize) -> bool {
        self.seats[row][col] == Seat::Taken
    }

    fn count_adjacent_taken_seats(&self, row: usize, col: usize) -> usize {
        // change these to isizes to avoid overflowing
        // let row = row as isize;
        // let col = col as isize;
        // let height = (self.height - 1) as isize;
        // let width = (self.width - 1) as isize;
        let mut taken = 0usize; 

        // for i in row - 1..row + 2 {
        //     for j in col - 1..col + 2 {
        //         if i >= 0 && i <= height && j >= 0 && j <= width {
        //             if i != row && j != col {
        //                 taken += if self.seats[i as usize][j as usize] == Seat::Taken { 1 } else { 0 };
        //             }
        //         } 
        //     }
        // }
        
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

    fn count_diagonally_taken_seats(&self, row: usize, col: usize) -> usize {
        let mut taken = 0;

        taken += if self.north_seat(row, col) != (-1, -1) { 1 } else { 0 };
        taken += if self.south_seat(row, col) != (-1, -1) { 1 } else { 0 };
        taken += if self.east_seat(row, col) != (-1, -1) { 1 } else { 0 };
        taken += if self.west_seat(row, col) != (-1, -1) { 1 } else { 0 };
        taken += if self.nw_seat(row, col) != (-1, -1) { 1 } else { 0 };
        taken += if self.ne_seat(row, col) != (-1, -1) { 1 } else { 0 };
        taken += if self.sw_seat(row, col) != (-1, -1) { 1 } else { 0 };
        taken += if self.se_seat(row, col) != (-1, -1) { 1 } else { 0 };

        taken
    } 

    fn count_all_taken_seats(&self) -> usize {
        self.seats.iter()
            .fold(0, |acc, row| {
                acc + row.iter()
                    .fold(0, |acc, loc| {
                        acc + if *loc == Seat::Taken { 1 } else { 0 }
                    })
            })
    }
    
    // Returns whether any seats were toggled
    fn tick(&mut self, for_part_one: bool) -> bool {
        let mut toggled = false;
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let seat = self.seats[row][col];
                let taken_seats = if for_part_one { 
                    self.count_adjacent_taken_seats(row, col)
                } else {
                    self.count_diagonally_taken_seats(row, col)
                };

                let next_seat = match (seat, taken_seats) {
                    // Rule 1: If a seat if empty (L) and there are no occupied
                    // seats adjacent to it, the seat becomes occupied
                    (Seat::Empty, 0) => {
                        toggled = true;
                        Seat::Taken
                    },
                    // Rule 2: If a seat is occupied and four or more seats 
                    // adjacent to it are also occupied, the seat becomes empty
                    (Seat::Taken, x) => {
                        if for_part_one {
                            if x >= 4 {
                                toggled = true;
                                Seat::Empty
                            } else {
                                Seat::Taken
                            }
                        } else {
                            if x >= 5 {
                                toggled = true;
                                Seat::Empty
                            } else {
                                Seat::Taken
                            }
                        }
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

    fn run_to_completion(&mut self, for_part_one: bool) {
        loop {
            if !self.tick(for_part_one) {
                break;
            }
        }
    }
}

pub fn calculate_taken_seats(input: &str, for_part_one: bool) -> usize {
    let mut layout = Layout::from_input(input); 
    layout.run_to_completion(for_part_one);
    layout.count_all_taken_seats()
}

#[cfg(test)]
#[test]
fn test_counting_adjacent_seats() {
    let test_input = 
"#.##.
#####";

    let test_layout = Layout::from_input(test_input.trim());

    assert_eq!(test_layout.count_adjacent_taken_seats(0, 0), 2);
    assert_eq!(test_layout.count_adjacent_taken_seats(0, 2), 4);
    assert_eq!(test_layout.count_adjacent_taken_seats(0, 3), 4);
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
    assert_eq!(layout.count_all_taken_seats(), 30);
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

    let tick = test_layout.tick(true);
    
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

    let tick = test_layout.tick(true);
    
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

    test_layout.run_to_completion(true);

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

    assert_eq!(calculate_taken_seats(test_input.trim(), true), 37);
}

#[test]
fn test_tick_one_part_two() {
    let test_input =
"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

    let expected = 
"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

    let mut test_layout = Layout::from_input(test_input.trim());
    let expected_layout = Layout::from_input(expected.trim());

    test_layout.run_to_completion(false);

    assert_eq!(test_layout.seats, expected_layout.seats);
}

#[test]
fn test_part_two() {
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

    assert_eq!(calculate_taken_seats(test_input.trim(), false), 26);
}

