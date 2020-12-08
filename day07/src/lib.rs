use regex::Regex;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};

lazy_static! {
    static ref LUGGAGE_OUTER_RE: Regex = Regex::new(r"^(?P<outer_color>.*) bags contain").unwrap();
    static ref LUGGAGE_INNER_RE: Regex = Regex::new(r"(?P<qty>\d+) (?P<color>[^,.]*) bags?[.,]").unwrap();
}

const MY_BAG: &str = "shiny gold";

struct LuggageRule {
    outer_color: String,
    contents: Vec<(usize, String)>
}

impl LuggageRule {
    fn parse_input(input: &str) -> Vec<LuggageRule> {
        input.lines()
            .map(|line| LuggageRule::from_str(line))
            .collect::<Vec<_>>()
    }

    fn from_str(s: &str) -> Self {
        let outer_color = LUGGAGE_OUTER_RE
            .captures(s)
            .unwrap()["outer_color"]
            .to_string();

        let contents = LUGGAGE_INNER_RE
            .captures_iter(s)
            .map(|capture| {
                (
                    capture["qty"]
                        .parse::<usize>()
                        .expect("regex guarantees positive integers"),
                    capture["color"].to_string(),
                ) 
            })
            .collect();

        LuggageRule {
            outer_color,
            contents
        } 
    }
}

pub fn part_one(input: &str) -> usize {
    let mut containers: HashMap<String, HashSet<String>> = HashMap::new();
    
    for rule in LuggageRule::parse_input(input) {
        for (_, bag) in &rule.contents {
            containers
                .entry(bag.clone())
                .or_default()
                .insert(rule.outer_color.clone());
        }
    }

    let mut queue: VecDeque<_> = containers[MY_BAG].iter().cloned().collect();
    let mut all_containers = HashSet::new();

    while let Some(q) = queue.pop_front() {
        if all_containers.insert(q.clone()) {
            queue.extend(containers.entry(q).or_default().iter().cloned());
        }
    }
    
    all_containers.len()
}

pub fn part_two(input: &str) -> u64 {
    let mut cache: HashMap<String, u64> = HashMap::new();
    let rules: HashMap<_, _> = LuggageRule::parse_input(input).into_iter()
        .map(|rule| (rule.outer_color.clone(), rule))
        .collect();

    query_rules(&rules, MY_BAG, &mut cache)
}

fn query_rules(
    rules: &HashMap<String, LuggageRule>,
    color: &str,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    let mut quantity = 0_u64;
    let rule = match rules.get(color) {
        Some(rule) => rule,
        None => return 0,
    };

    for (qty, color) in &rule.contents {
        let qty = *qty as u64;
        quantity += qty;

        let per_color = match cache.get(color) {
            Some(n) => *n,
            None => query_rules(rules, color, cache)
        };

        quantity += qty * per_color;
    }

    cache.insert(color.to_string(), quantity);
    
    quantity
}



