use std::collections::HashMap;
fn main() {
    let initial = "1,2,16,19,18,0".split(",").map(|a| a.parse().unwrap()).collect();
    let number = memory_game(&initial, 2020);
    println!("2020th number spoken is {}", number);

    let larger_number = memory_game(&initial, 30000000);
    println!("30000000th number spoken is {}", larger_number);

}

fn memory_game(initial_values: &Vec<u128>, stopping_iteration: usize) -> u128 {
    let mut last_seen_values = initial_values.into_iter().enumerate().fold(HashMap::new(), 
        |mut acc, (index, val)| {acc.insert(*val, index + 1); acc});

    let mut iteration = initial_values.len() + 1;
    let mut last_spoken = Vec::new();
    last_spoken.push(0);
    loop {
        if iteration == stopping_iteration {
            break;
        }
        let old_last = &last_spoken.pop().unwrap();

        if let Some(last_seen) = last_seen_values.get_mut(old_last) {
            let new_value = iteration - *last_seen;
            *last_seen = iteration;
            last_spoken.push(new_value as u128);
        }
        else {
            last_seen_values.insert(*old_last, iteration);
            last_spoken.push(0);
        }

        iteration += 1;
    }

    return *last_spoken.last().unwrap();

}


mod tests {
    use super::*;

    #[test]
    fn test_memory_game() { 
        let input_values = "0,3,6".split(",").map(|a| a.parse().unwrap()).collect(); 

        let number = memory_game(&input_values, 2020);
        assert_eq!(number, 436);

        let larger_number = memory_game(&input_values, 30000000);
        assert_eq!(larger_number, 175594);
    }
}