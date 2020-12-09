use petgraph::graphmap::DiGraphMap;
use petgraph::visit::{Bfs, Dfs};
use petgraph::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    test_rule_generate();
    let data = read_data("src/input.txt");
    let rules = rule_generate_contains(&data);
    let available_bags = find_contains_num(&rules.0, "shiny gold");
    println!("bags {}", available_bags);

    let number_bags_required = find_required(&rules.1, "shiny gold");
    println!("bags required {}", number_bags_required);
}

fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename)
    .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    let rules = reader.lines().map(|line| line.unwrap()).collect();

    return rules;
}

fn rule_generate_contains(rules: &Vec<String>) -> (DiGraphMap::<&str, f32>, DiGraphMap::<&str, f32>) {
    let mut bags = DiGraphMap::<&str, f32>::new();
    let mut switch_direction_bags = DiGraphMap::<&str, f32>::new();

    let bag_rule_regex = Regex::new(r"(?P<number>\d) (?P<bag>\w+ \w+) bag[s]?").unwrap();
    for rule in rules {
        let key_value: Vec<&str> = rule.split(" contain ").collect();
        let main_bag = key_value[0].strip_suffix(" bags").unwrap();
        bags.add_node(main_bag);
        switch_direction_bags.add_node(main_bag);

        for bag_rules in key_value.last().unwrap().split(',').collect::<Vec<&str>>().iter() {
            let captured_rules = bag_rule_regex.captures(bag_rules);
            if captured_rules.is_some() {
                let rule = captured_rules.unwrap();
                let connecting_bag = &rule.name("bag").unwrap().as_str();
                bags.add_node(connecting_bag);
                switch_direction_bags.add_node(connecting_bag);
                bags.add_edge(connecting_bag, main_bag, rule["number"].parse::<f32>().unwrap().clone());
                switch_direction_bags.add_edge(main_bag, connecting_bag, rule["number"].parse::<f32>().unwrap().clone());
            }
        }
    }
    return (bags, switch_direction_bags);
}

fn find_contains_num(bag_graph: &DiGraphMap::<&str, f32>, starting_bag: &str) -> usize {
    let contained_bags = find_contains(bag_graph, starting_bag);
    return contained_bags.len() - 1;
}


fn find_contains(bag_graph: &DiGraphMap::<&str, f32>, starting_bag: &str) -> Vec<String> {
    let mut bags = Vec::new();
    let mut dfs = Bfs::new(bag_graph, starting_bag);
    while let Some(visited) = dfs.next(bag_graph) {
        bags.push(visited.to_string());
    }
    return bags;
}

fn dfs<'a>(bag_graph: &DiGraphMap::<&'a str, f32>, starting_bag: &str) -> f32 {
    let mut count = 0.0;

    let mut dfs_handler = Dfs::new(bag_graph, starting_bag);
    while let Some(visited) = dfs_handler.next(bag_graph) {
        if bag_graph.contains_edge(starting_bag, visited) {
            let edge_multiplier = bag_graph.edge_weight(starting_bag, visited).unwrap();
            count += edge_multiplier + edge_multiplier * dfs(bag_graph, visited);
        }
    }
    return count;
}

fn find_required(bag_graph: &DiGraphMap::<&str, f32>, starting_bag: &str) -> f32 {
    let mut number_of_bags = 0.0;

    for neighbor in bag_graph.neighbors_directed(starting_bag, Outgoing) {
        let edge = bag_graph.edge_weight(starting_bag, neighbor).unwrap();
        number_of_bags += edge + edge * dfs(bag_graph, neighbor);
    }
    return number_of_bags;
}

fn test_rule_generate() {
    let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.".lines().map(|s| s.to_string()).collect();

    let generate = rule_generate_contains(&rules);

    let dfs = find_contains(&generate.0, "shiny gold");

    assert_eq!(dfs, vec![
        "shiny gold".to_string(),
        "bright white".to_string(),
        "muted yellow".to_string(),
        "light red".to_string(),
        "dark orange".to_string(),
    ]);

    let number = find_required(&generate.1, "shiny gold");
    assert_eq!(number, 32.0);

    let rules_2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.".lines().map(|s| s.to_string()).collect();
    let generate_2 = rule_generate_contains(&rules_2);
    let number = find_required(&generate_2.1, "shiny gold");
    assert_eq!(number, 126.0);
}