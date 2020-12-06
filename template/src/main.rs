use aocf::Aoc;
use std::collections::HashSet;

fn two_sum(v: &[i32], target: i32) -> (i32, i32) {
    let mut set = HashSet::new();    

    for val in v {
        let complement = target - val;

        if set.contains(&complement) {
            return (*val, complement);
        } else {
            set.insert(val);
        }
    }

    (-1, -1)
}

fn three_sum(v: &mut [i32], target: i32) -> (i32, i32, i32) {
    v.sort();

    for i in 0..v.len() - 2 {
        let a = v[i];
        let mut l = i + 1;
        let mut r = v.len() - 1;

        while l < r {
            let sum = a + v[l] + v[r];

            if sum == target {
                return (a, v[l], v[r]);
            }

            if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    (-1, -1, -1)
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(1))
        .init()
        .unwrap();

    if let Ok(input) = aoc.get_input(false) {
        let mut expenses: Vec<i32> = input.lines()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        // Part I
        let (x, y) = two_sum(&expenses, 2020);
        let answer = x * y;
        
        println!("{}", answer);

        // Part II
        let (a, b, c) = three_sum(&mut expenses, 2020); 
        let answer = a * b * c;

        println!("{}", answer);
    }
}
