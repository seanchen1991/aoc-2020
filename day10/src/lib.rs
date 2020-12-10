use std::collections::HashMap;

pub fn part_one(joltages: &[usize]) -> usize { 
    let distributions = joltages
        .windows(2)
        .map(|chunk| chunk[1] - chunk[0])
        .fold(HashMap::new(), |mut map, diff| {
            *map.entry(diff).or_insert(0) += 1;
            map
        });

    distributions[&3] * distributions[&1]
}

pub fn part_two(joltages: &[usize]) -> usize {
    let len = joltages.len();
    let mx = joltages[len - 1];
    let mut dp = vec![0usize; mx + 1];

    for j in joltages {
        match j {
            0 => dp[*j] = 1,
            1 => dp[*j] += dp[j - 1],
            2 => dp[*j] += dp[j - 1] + dp[j - 2],
            _ => dp[*j] = dp[j - 1] + dp[j - 2] + dp[j - 3],
        }
    }

    dp[mx]
}

