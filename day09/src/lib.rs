use std::cmp::Ordering;
use std::iter::FromIterator;
use std::collections::HashSet;

const PREAMBLE_LENGTH: usize = 25;

pub fn part_one(nums: &[i64]) -> i64 {
    (PREAMBLE_LENGTH..nums.len()).find(|i| {
        let preamble: HashSet<&i64> = HashSet::from_iter(nums[i - PREAMBLE_LENGTH..*i].iter());
        preamble.iter().map(|num| nums[*i as usize] - *num).all(|diff| !preamble.contains(&diff))
    })
    .map(|i| nums[i])
    .expect("No answer found")
}

pub fn part_two(nums: &[i64], target: i64) -> i64 {
    let (mut i, mut j, mut sum) = (0, 0, 0);

    loop {
        match sum.cmp(&target) {
            Ordering::Greater => {
                sum -= nums[usize::min(i, nums.len() - 1)];
                i += 1;
            },
            Ordering::Less => {
                sum += nums[usize::min(j, nums.len() - 1)];
                j += 1;
            },
            Ordering::Equal => {
                let min = nums[i..j].iter()
                    .min()
                    .expect("No min found");
                let max = nums[i..j].iter()
                    .max()
                    .expect("No max found");
                return min + max;
            }
        }
    }
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
