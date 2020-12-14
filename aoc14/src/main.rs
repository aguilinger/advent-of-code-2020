use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::Read;


fn main() {
    let memory_inputs = read_data("src/input.txt");
    let sum = sum_memory_from_input(&memory_inputs);
    println!("Sum is {}", sum);
    let sum_memory_mask = sum_memory_address_masked_from_input(&memory_inputs);
    println!("Sum with memory mask is {}", sum_memory_mask);
}

fn read_data(filename: &str) -> String {
    let mut file = File::open(filename)
        .expect("Something went wrong reading the file");
    let mut memory = String::new();
    file.read_to_string(&mut memory).expect("Error reading data");

    return memory;
}

fn num_to_reverse_binary(num: u128) -> Vec<u8> {
    return format!("{:b}", num).chars().rev().map(|a| a.to_digit(10).unwrap() as u8).collect();
}

fn reverse_binary_to_num(value: &mut Vec<u8>) -> u128 {
    value.reverse();
    let final_string: String = value.into_iter().map(|a| a.to_string()).collect();
    let back_to_number = u128::from_str_radix(&final_string[..], 2).unwrap();
    return back_to_number;
}

fn initialize_memory(memory_inputs: Vec<(String, u64, u128)>) -> HashMap<u64, u128> {
    let mut memory = HashMap::new();

    for memory_input in memory_inputs {
        let mut value_bits: Vec<u8> = num_to_reverse_binary(memory_input.2 as u128);
        let mask_values = memory_input.0.chars().rev().enumerate().fold(HashMap::<usize,u8>::new(), |mut acc, (index, value)| {if value != 'X' {acc.insert(index, value.to_digit(10).unwrap() as u8)} else {Some(0)}; acc});
        for (mask_key, mask_value) in mask_values.iter() {
            if *mask_key >= value_bits.len() {
                value_bits.resize_with(*mask_key + 1, Default::default);
            }
            value_bits[*mask_key] = *mask_value;
        }
        let number = reverse_binary_to_num(&mut value_bits);
        memory.insert(memory_input.1, number);
    }

    return memory;
}

fn generate_address_space<'a>(mask: &'a HashMap<usize, u8>, last_vector: &'a Vec<u8>, collection: &'a mut Vec<Vec<u8>>, max_size: usize) -> &'a mut Vec<Vec<u8>> {
    if last_vector.len() >= max_size {
        return collection;
    }
    let new_index = last_vector.len();
    collection.retain(|x| x.len() > new_index - 1);
    if mask.contains_key(&new_index) {
        let mut new_vector = last_vector.clone();
        new_vector.push(*mask.get(&new_index).unwrap());
        let next_vector = new_vector.clone();
        collection.push(new_vector);
        generate_address_space(mask, &next_vector, collection, max_size);
    }
    else {
        for i in &[0,1] {
            let mut new_vector = last_vector.clone();
            new_vector.push(*i);
            let next_vector = new_vector.clone();
            collection.push(new_vector);
            if new_index < max_size {
                generate_address_space(mask, &next_vector, collection, max_size);
            }
        }
        if new_index >= max_size {
            let fake_vector = vec![0; max_size];
            generate_address_space(mask, &fake_vector, collection, max_size);
        }
    };
    
    return collection;

}

fn initialize_memory_with_memory_masking(memory_inputs: Vec<(String, u64, u128)>) -> HashMap<u64, u128> {
    let mut memory = HashMap::new();

    for memory_input in memory_inputs {
        let address_bits: Vec<u8> = num_to_reverse_binary(memory_input.1 as u128);
        let num_xs = memory_input.0.chars().filter(|a| *a == 'X').collect::<Vec<char>>().len();
        let mask_values = memory_input.0.chars().rev().enumerate().fold(HashMap::<usize,u8>::new(), |mut acc, (index, value)| {if value != 'X' {acc.insert(index, value.to_digit(10).unwrap() as u8)} else {Some(0)}; acc});
        let mut final_constants = HashMap::new();
        for (mask_key, mask_value) in mask_values.iter() {
            if *mask_value == 0 {
                if *mask_key < address_bits.len() {
                    final_constants.insert(*mask_key, address_bits[*mask_key]);
                }
                else {
                    final_constants.insert(*mask_key, 0);
                }
            }
            else {
                final_constants.insert(*mask_key, 1);
            }
        }
        let mut collection = Vec::new();
        let starting_space = Vec::new();
        let addresses = generate_address_space(&final_constants, &starting_space, &mut collection, 36);

        let pruned_addresses: Vec<Vec<u8>> = addresses.into_iter().filter(|address| address.len() == 36).map( |a| a.to_owned()).collect();
        assert_eq!(pruned_addresses.len(), (2 as usize).pow(num_xs as u32));
        for mut address in pruned_addresses {
            let number = reverse_binary_to_num(&mut address) as u64;
            memory.insert(number, memory_input.2);
        }
    }

    return memory;
}

fn parse_program_input(input: &String) -> Vec<(String, u64, u128)> {
    let mut memory_addresses = Vec::new();
    let mut last_bitmask = "".to_string();
    let memory_regex = Regex::new(r"^mem\[(?P<memory_address>\d+)\] = (?P<value>\d+)$").unwrap();
    for line in input.lines() {
        if line.starts_with("mask = ") {
            last_bitmask = line.strip_prefix("mask = ").unwrap().to_string();
        }
        else {
            let captures = memory_regex.captures(line).unwrap();
            memory_addresses.push((
                last_bitmask.clone(), 
                captures["memory_address"].parse().unwrap(), 
                captures["value"].parse().unwrap()
            ));
        }

    }

    return memory_addresses;
}

fn sum_memory_from_input(input: &String) -> u128 {
    let parsed_input = parse_program_input(input);
    let memory = initialize_memory(parsed_input);
    return memory.values().sum();
}

fn sum_memory_address_masked_from_input(input: &String) -> u128 {
    let parsed_input = parse_program_input(input);
    let memory = initialize_memory_with_memory_masking(parsed_input);
    return memory.values().sum();
}

mod tests {
    use super::*;

    #[test]
    fn test_memory() {  
        let test_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0".to_string();
    
        let sum = sum_memory_from_input(&test_input);
        assert_eq!(sum, 165);

        let test_input_2 = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1".to_string();
        let sum = sum_memory_address_masked_from_input(&test_input_2);
        assert_eq!(sum, 208);

    let test_input_3 = "mask = X0000000000000000000000000000001001X
mem[42] = 100".to_string();
        let sum = sum_memory_address_masked_from_input(&test_input_3);
        assert_eq!(sum, 400);
        }
}