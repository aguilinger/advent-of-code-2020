#![feature(deque_range)]
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    test_find_invalid_sums();
    let numbers = read_data("src/input.txt");
    let invalid = find_invalid_sums_number(&numbers, 25);
    println!("Invalid number {}", invalid);

    let mut set = find_contiguous_set_adds_to_sum(&numbers, invalid);
    let weakness = add_min_and_max(&mut set);
    println!("Encryption weakness {}", weakness);
}


fn read_data(filename: &str) -> Vec<u128> {
    let file = File::open(filename)
    .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    let numbers = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

    return numbers;
}

fn find_contiguous_set_adds_to_sum(numbers: &Vec<u128>, target: u128) -> Vec<u128> {
    let mut shortened_numbers_reverse = numbers.clone();
    shortened_numbers_reverse.truncate(numbers.iter().position(|n| n == &target).unwrap());
    shortened_numbers_reverse.reverse();
    let mut shortened_numbers_reverse_queue = VecDeque::from(shortened_numbers_reverse.clone());

    let mut found_set = Vec::new();
    loop {
        let mut sum = 0;
        let mut over = false;
        let mut found = false;
        let mut depth = 0;
        for number in shortened_numbers_reverse_queue.iter() {
            depth += 1;
            sum += number;
            if sum == target {
                found = true;
                break;
            }
            else if sum > target {
                over = true;
                break;
            }
        }

        if over {
            shortened_numbers_reverse_queue.pop_front();
            over = false;
            sum = 0;
            depth = 0;
        }

        if found {
            shortened_numbers_reverse_queue.truncate(depth);
            found_set = Vec::from(shortened_numbers_reverse_queue);
            break;
        }

    }
    return found_set;
}

fn add_min_and_max(numbers: &mut Vec<u128>) -> u128 {
    numbers.sort();
    return numbers.first().unwrap() + numbers.last().unwrap();

}

fn find_invalid_sums_number(numbers: &Vec<u128>, buffer: usize) -> u128 {
    let mut last_values_buffer = VecDeque::with_capacity(buffer);
    for initial_buffer in numbers[..buffer].iter() {
        last_values_buffer.push_back(initial_buffer);
    }
    let mut invalid: u128 = 0;
    for check_number in numbers[buffer..].iter() {
        let mut valid_number = false;
        for (index_1, buffer_number_1) in last_values_buffer.iter().enumerate() {
            for buffer_number_2 in last_values_buffer.range(index_1..) {
                if *buffer_number_1 + *buffer_number_2 == *check_number {
                    valid_number = true;
                    break;
                }
            }
        }
        if !valid_number {
            invalid = *check_number;
            break;
        }
        last_values_buffer.pop_front();
        last_values_buffer.push_back(check_number);
    }
    return invalid;
}

fn test_find_invalid_sums() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576".lines().map(|n| n.parse().unwrap()).collect();
        let invalid = find_invalid_sums_number(&input, 5);
        assert_eq!(invalid, 127);

        let mut set = find_contiguous_set_adds_to_sum(&input, invalid);
        let weakness = add_min_and_max(&mut set);
        assert_eq!(weakness, 62);
}