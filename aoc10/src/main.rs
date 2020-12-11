
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet};
use std::iter::FromIterator;
use factorial::Factorial;



fn main() {
    test();

    let jolts = read_data("src/input.txt");
    let ranges = range_finder(&jolts);
    println!("Multiple is {}", ranges.0 * ranges.1);
    let arrangements = count_valid_arrangements(&jolts);
    println!("Number of arrangements is {}", arrangements);
}

fn read_data(filename: &str) -> Vec<u64> {
    let file = File::open(filename)
    .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    let jolts = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

    return jolts;
}

fn range_finder(jolts: &Vec<u64>) -> (u64, u64) {
    let mut sorted_jolts = jolts.clone();
    sorted_jolts.sort();
    let mut previous_jolt = 0;
    let mut one_jolt_step = 0;
    let mut three_jolt_step = 1;
    for jolt in sorted_jolts {
        if jolt - previous_jolt == 1 {
            one_jolt_step += 1;
        }
        else if jolt - previous_jolt == 3 {
            three_jolt_step += 1;
        }
        previous_jolt = jolt;
    }

    return (one_jolt_step, three_jolt_step);

}

fn choose(n: u128, k: u128) -> u128 {
    if n >= k {
        return (n.factorial() as u128)/(k.factorial() * (n - k).factorial());
    }
    else {
        return 0;
    }
}

fn count_valid_arrangements(jolts: &Vec<u64>) -> u128 {
    let mut sorted_jolts = jolts.clone();
    sorted_jolts.push(0);
    sorted_jolts.sort();
    let largest = sorted_jolts.last().unwrap();

    let jolts_hash: HashSet<u64> = HashSet::from_iter(sorted_jolts.clone());
    let mut counts_per_section = Vec::new();
    let mut section_size = 1;
    for jolt in sorted_jolts[..sorted_jolts.len()].iter() {
        if jolt != largest && jolts_hash.contains(&(jolt + 1)) {
            section_size += 1;
        }
        else {
            let mut count_per_section = 1;
            if section_size > 2 {
                count_per_section = 2;

                if section_size == 4 {
                    count_per_section = 4;
                }
                if section_size > 4 {
                    count_per_section = choose(section_size - 1, 3);
                    for i in 1..section_size-3 {
                        count_per_section += choose(section_size - 1 - i, 3 - i);
                    }
                }

            }
            else {
                section_size = 1;
            }

            section_size = 1;
            counts_per_section.push(count_per_section);
        }

    }

    return counts_per_section.iter().fold(1, |acc, x| acc * x);
}

fn test() {
    let input_1 = "16
10
15
5
1
11
7
19
6
12
4".lines().map(|l| l.parse().unwrap()).collect();
    assert_eq!(range_finder(&input_1), (7, 5));
    assert_eq!(count_valid_arrangements(&input_1), 8);

    let input_2 = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3".lines().map(|l| l.parse().unwrap()).collect();
    assert_eq!(range_finder(&input_2), (22, 10));
    assert_eq!(count_valid_arrangements(&input_2), 19208);

}