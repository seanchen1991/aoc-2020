use std::iter::FromIterator;
use std::collections::HashSet;

const PREAMBLE_LENGTH: usize = 25;

pub fn part_one(input: &str) -> i64 {
    let nums: Vec<_> = input.lines()
        .map(|num| num.parse::<i64>().expect("Failed to parse a line of input"))
        .collect();

    (PREAMBLE_LENGTH..nums.len()).find(|i| {
        let preamble: HashSet<&i64> = HashSet::from_iter(nums[i - PREAMBLE_LENGTH..*i].iter());
        preamble.iter().map(|num| nums[*i as usize] - *num).all(|diff| !preamble.contains(&diff))
    })
    .map(|i| nums[i])
    .expect("No answer found")
}

#[cfg(test)]
#[test]
// gotta change the PREAMBLE_LENGTH constant to 5 for this to work
fn test_part_one() {
    let test_input ="35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    assert_eq!(part_one(&test_input), 127); 
}
