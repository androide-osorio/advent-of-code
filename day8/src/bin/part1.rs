// first line represent path instructions where L = left, R = right
// second line is a list of rules where AAA = (BBB, CCC) means AAA is connected to BBB to the left and CCC to the right
// the rules are in no particular order
// the rules are not necessarily connected to the path or each other
use fancy_regex::Regex;
use std::collections::HashMap;

pub fn parse_instructions(instructions: &str) -> (&str, &str) {
    let mut input_iter = instructions.split("\n\n");
    let moves_list = input_iter.next().unwrap();
    let nodes_list = input_iter.next().unwrap();

    (moves_list, nodes_list)
}

// build adjacency list from rules
pub fn build_adjacency_list(rules: &str) -> HashMap<&str, (&str, &str)> {
    let rule_line = Regex::new(r#"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)"#).unwrap();

    let results: HashMap<&str, (&str, &str)> = rule_line
        .captures_iter(rules)
        .map(|captures| {
            let captures = captures.expect("Error running regex");
            let root = captures.get(1).unwrap().as_str();
            let left = captures.get(2).unwrap().as_str();
            let right = captures.get(3).unwrap().as_str();

            (root, (left, right))
        })
        .collect();
    results
}

// find path from start to end
pub fn find_path<'a>(
    start: &'a str,
    end: &'a str,
    moves_list: &str,
    adjacency_list: HashMap<&'a str, (&'a str, &'a str)>,
) -> Vec<String> {
    let mut i = 0;
    let mut path: Vec<String> = Vec::new();
    let mut current_node = Some(start);

    while current_node != Some(end) {
        let direction = moves_list.chars().nth(i).unwrap();
        let node_value = current_node.expect("Expected a valid node");
        let (left, right) = adjacency_list.get(node_value).unwrap();

        path.push(direction.to_string());
        i = (i + 1) % moves_list.len();

        current_node = match direction {
            'L' => Some(left),
            'R' => Some(right),
            _   => None,
        }
    }

    path
}

pub fn main() {
    println!("Part 1!");
    let data = include_str!("../data.txt");

    let (moves_list, nodes_list) = parse_instructions(data);
    let adjacency_list = build_adjacency_list(nodes_list);
    let path = find_path("AAA", "ZZZ", moves_list, adjacency_list);

    println!("Path: {:?}", path.join(""));
    println!("Path length: {}", path.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input1 = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
"#;

        let (moves_list, nodes_list) = parse_instructions(input1);

        assert_eq!(moves_list, "RL");
        assert_eq!(nodes_list, "AAA = (BBB, CCC)\nBBB = (DDD, EEE)\n");
    }

    #[test]
    fn test_build_adjacency_list() {
        let input1 = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

        let (_, nodes_list) = parse_instructions(input1);
        let adjacency_list = build_adjacency_list(nodes_list);

        assert_eq!(adjacency_list.len(), 7);
        assert_eq!(adjacency_list.get("AAA"), Some(&("BBB", "CCC")));
        assert_eq!(adjacency_list.get("BBB"), Some(&("DDD", "EEE")));
        assert_eq!(adjacency_list.get("CCC"), Some(&("ZZZ", "GGG")));
        assert_eq!(adjacency_list.get("DDD"), Some(&("DDD", "DDD")));
        assert_eq!(adjacency_list.get("EEE"), Some(&("EEE", "EEE")));
        assert_eq!(adjacency_list.get("GGG"), Some(&("GGG", "GGG")));
        assert_eq!(adjacency_list.get("ZZZ"), Some(&("ZZZ", "ZZZ")));
    }

    #[test]
    fn test_find_path() {
                let input1 = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

        let (moves_list, nodes_list) = parse_instructions(input1);
        let adjacency_list = build_adjacency_list(nodes_list);

        let path = find_path("AAA", "ZZZ", moves_list, adjacency_list);

        assert_eq!(path.len(), 2);
        assert_eq!(path, vec!["R", "L"]);
    }

    #[test]
    fn test_find_path_repeated_steps() {
                let input1 = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

        let (moves_list, nodes_list) = parse_instructions(input1);
        let adjacency_list = build_adjacency_list(nodes_list);

        let path = find_path("AAA", "ZZZ", moves_list, adjacency_list);

        assert_eq!(path.len(), 6);
        assert_eq!(path, vec!["L", "L", "R", "L", "L", "R"]);
    }
}