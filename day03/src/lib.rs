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

