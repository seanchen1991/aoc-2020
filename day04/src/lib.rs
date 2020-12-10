use regex::Regex;

pub fn part_one(passports: &[&str]) -> usize {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    passports.iter()
        .filter(|passport| {
            keys.iter()
                .all(|key| passport.contains(key))
        })
        .count()
}

pub fn part_two(passports: &[&str]) -> usize {
    let re = [
        r"byr:(?:19[2-9][0-9]|200[0-2])",
        r"iyr:20(?:1[0-9]|20)",
        r"eyr:20(?:2[0-9]|30)",
        r"hgt:(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)",
        r"hcl:#[0-9|a-f]{6}",
        r"ecl:(?:amb|blu|brn|gry|grn|hzl|oth)",
        r"pid:[0-9]{9}(?:\n| |$)",
    ]
    .iter()
    .map(|r| Regex::new(r).expect("Invalid regex expression"));

    passports.iter()
        .filter(|passport| {
            re.clone()
                .all(move |r| r.is_match(passport))
        })
        .count()
}

