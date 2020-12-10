pub fn part_one(map: &[Vec<char>], slope: (usize, usize)) -> u32 {
    let (mut x, mut y, mut trees_hit) = (0, 0, 0);
    let (height, width) = (map.len(), map[0].len());

    while y < height {
        if map[y][x] == '#' {
            trees_hit += 1;
        }

        x = (x + slope.0) % width;
        y += slope.1;
    }

    trees_hit
}

pub fn part_two(map: &[Vec<char>]) -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|slope| part_one(&map, *slope)).product()
}

