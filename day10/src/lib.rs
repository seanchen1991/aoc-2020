use std::collections::HashMap;

pub fn part_one(joltages: &[u32]) -> u32 {
    let mut joltages = joltages.to_vec();

    joltages.sort();  
    joltages.insert(0, 0);
    joltages.push(joltages.last().unwrap() + 3);
    
    let distributions = joltages[..]
        .windows(2)
        .map(|chunk| chunk[1] - chunk[0])
        .fold(HashMap::new(), |mut map, diff| {
            *map.entry(diff).or_insert(0) += 1;
            map
        });

    distributions[&3] * distributions[&1]
}

