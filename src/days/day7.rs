use std::path::Path as Path;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::common::io;

fn build_tree_from_rules(lines: &Vec<String>) -> HashMap<String, HashMap<String, i32>> {
    let mut root: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in lines {
        let mut clone = line.to_string();
        clone = clone.replace("bag.", "");
        clone = clone.replace("bags.", "");
        clone = clone.replace(" bags contain ", "|");
        clone = clone.replace("bag,", ":");
        clone = clone.replace("bags,", ":");
        clone = clone.replace("no other", "");

        let tokens: Vec<&str> = clone.split('|').collect();
        let root_color = tokens[0].trim().to_string();
        let contain_rules_string = tokens[1].trim();
        let contain_rules: Vec<&str> = contain_rules_string.split(':').collect();

        log::debug!("{}", root_color);
        log::debug!("{}", clone);
        log::debug!("{}", contain_rules_string);
        
        let mut contain: HashMap<String, i32> = HashMap::new();
        if contain_rules_string.is_empty() {
            log::debug!("skipping");
        } else {
            for rule in contain_rules {
                let mut rule_tokens: Vec<&str> = rule.trim().split(' ').collect();
                let count: i32 = rule_tokens.get(0).unwrap().trim().parse().unwrap();
                let color_words: Vec<&str> = rule_tokens.drain(1..rule_tokens.len()).collect();
                contain.insert(color_words.join(" "), count);
            } 
        }
        root.insert(root_color, contain);
    }

    return root;
}

fn look_for_shiny_gold(tree: &HashMap<String, HashMap<String, i32>>, subtree: &HashMap<String, i32>) -> i32 {
    let mut total = 1;
    for key in subtree.keys() {
        log::debug!("going deeper into color: {}", key);
        let cost = look_for_shiny_gold(&tree, &tree[key]);
        total += cost * subtree[key];
    }
    return total;
}

fn part1(tree: &HashMap<String, HashMap<String, i32>>) -> i32 {
    log::info!("Running Part 1");
    let mut count = 0;
    for color in tree.keys() {
        log::debug!("Exploring color: {}", color);
        
        let mut queue: VecDeque<&HashMap<String, i32>> = VecDeque::new();
        queue.push_back(&tree[color]);
        let mut found = false;

        while !queue.is_empty() {
            let contains_rules = queue.pop_back().unwrap();

            if contains_rules.contains_key("shiny gold") {
                log::debug!("The eagle has landed");
                found = true;
                continue;
            }

            for rule in contains_rules.keys() {
                log::debug!("going deeper into color: {}", rule);
                queue.push_back(&tree[rule]);
            }
        }

        if found {
            count += 1;
        }
    }

    log::info!("Found {} different bags that can contain shiny gold", count);
    return count;
}

fn part2(tree: &HashMap<String, HashMap<String, i32>>) -> i32 {
    log::info!("Running Part 2");
    let color = "shiny gold";

    log::debug!("Exploring color: {}", color);
    let subtree = &tree[color];

    // Need to remove the bag itself since we are starting count total at 1
    // This means that we need to exclude the bag itself hence -1
    let cost = look_for_shiny_gold(&tree, &subtree) - 1;
    log::info!("Found that the cost of {} is {}", color, cost);
    return cost;
}

pub fn run(filename: impl AsRef<Path>) {
    let lines = io::lines_from_file(filename);
    let tree = build_tree_from_rules(&lines);

    part1(&tree);
    part2(&tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_tree_from_rules_test() {
        let lines = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags."
        ].iter().map(|x| x.to_string()).collect();
        let tree = build_tree_from_rules(&lines);

        assert_eq!(2, tree["light red"].len());
        assert_eq!(2, tree["dark orange"].len());
        assert_eq!(1, tree["bright white"].len());
        assert_eq!(2, tree["muted yellow"].len());
        assert_eq!(2, tree["shiny gold"].len());
    }

    #[test]
    fn part1_test() {
        let lines = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags."
        ].iter().map(|x| x.to_string()).collect();
        let tree = build_tree_from_rules(&lines);

        assert_eq!(4, part1(&tree));
    }

    #[test]
    fn part2_test() {
        let lines = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ].iter().map(|x| x.to_string()).collect();
        let tree = build_tree_from_rules(&lines);

        env_logger::init();
        assert_eq!(126, part2(&tree));
    }
}