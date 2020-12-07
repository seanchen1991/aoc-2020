use aocf::Aoc;

#[derive(Debug)]
struct Policy {
    start: usize,
    end: usize,
    letter: char,
}

impl Policy {
    fn from_str(pol: &str) -> Self {
        let parts: Vec<&str> = pol.split(' ').collect();
        let range = parts[0];
        let letter = parts[1];

        let range_parts: Vec<&str> = range.split('-').collect();
        let start = range_parts[0];
        let end = range_parts[1];
       
        Policy {
            start: start.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap(),
            letter: letter.chars().last().unwrap(),
        }
    }

    fn check_part_one(&self, pw: &str) -> bool {
        let nletters = pw.chars()
            .filter(|c| *c == self.letter)
            .count();

        nletters >= self.start && nletters <= self.end
    }

    fn check_part_two(&self, pw: &str) -> bool {
        let mut found = 0;

        if let Some(letter) = pw.chars().nth(self.start - 1) {
            if letter == self.letter {
                found += 1;
            }
        }
        
        if let Some(letter) = pw.chars().nth(self.end - 1) {
            if letter == self.letter {
                found += 1;
            }
        }

        found == 1
    }
}

fn part_one(input: &str) -> usize {
    input.lines()
        .fold(0, |mut acc, line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let pol = parts[0];
            let password = parts[1];
            let policy = Policy::from_str(pol);
            
            if policy.check_part_one(password) {
                acc += 1;
            }     

            acc
        })
}

fn part_two(input: &str) -> usize {
    input.lines()
        .fold(0, |mut acc, line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let pol = parts[0];
            let password = parts[1];
            let policy = Policy::from_str(pol);
            
            if policy.check_part_two(password) {
                acc += 1;
            }     

            acc
        })
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(2))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let answer = part_one(&input.trim());
        println!("{}", answer);

        let answer = part_two(&input.trim());
        println!("{}", answer);
    }
}
