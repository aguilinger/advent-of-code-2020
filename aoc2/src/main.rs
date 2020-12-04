use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    test_is_valid();
    test_is_valid_v2();
    test_count_valid();
    test_count_valid_v2();

    let data = read_data("src/input.txt").unwrap();
    let valid_from_input = count_valid(&data);
    println!("version 1 valid count: {}", valid_from_input);

    let valid_from_input_v2 = count_valid_v2(&data);
    println!("version 1 valid count: {}", valid_from_input_v2);
}

fn read_data(filename: &str) -> Result<Vec<PasswordVerifier>, Error> {
    let mut password_data = Vec::new();

    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    for full_line in reader.lines() {
        
        let line = &mut full_line.unwrap().to_string();
        let mut iter = line.split(" ");

        let mut bounds = iter.next().unwrap().split("-");
        let min = bounds.next().unwrap().parse::<usize>().unwrap();
        let max = bounds.last().unwrap().parse::<usize>().unwrap();
        let character = iter.next().unwrap().split(":").next().unwrap().parse::<char>().unwrap();
        let password = iter.next().unwrap().to_string();

        password_data.push(PasswordVerifier::new(min, max, character, password));
    }

    return Ok(password_data)

}

struct PasswordVerifier {
    minimum: usize,
    maximum: usize,
    character: char,
    password: String,
}

impl PasswordVerifier {
    pub fn new(minimum: usize, maximum: usize, character: char, password: String) -> PasswordVerifier {
        PasswordVerifier {
            minimum: minimum,
            maximum: maximum,
            character: character,
            password: password,
        }
    }
    pub fn is_valid(&self) -> bool {
        let number_characters = self.password.split(self.character).collect::<Vec<&str>>().len() - 1;
        return number_characters <= self.maximum && number_characters >= self.minimum;
    }

    fn character_in_password(&self, position: usize) -> char{
        let a = char::from(self.password.as_bytes()[position - 1]);
        return a
    }

    pub fn is_valid_v2(&self) -> bool {
        return 
            (self.character_in_password(self.minimum) == self.character) 
            ^ (self.character_in_password(self.maximum) == self.character);
    }
}

fn test_is_valid() {
    assert_eq!(PasswordVerifier::new(1, 3, 'a', "abcde".to_string()).is_valid(), true);
    assert_eq!(PasswordVerifier::new(1, 3, 'b', "cdefg".to_string()).is_valid(), false);
    assert_eq!(PasswordVerifier::new(2, 9, 'c', "ccccccccc".to_string()).is_valid(), true);
}

fn test_is_valid_v2() {
    assert_eq!(PasswordVerifier::new(1, 3, 'a', "abcde".to_string()).is_valid_v2(), true);
    assert_eq!(PasswordVerifier::new(1, 3, 'b', "cdefg".to_string()).is_valid_v2(), false);
    assert_eq!(PasswordVerifier::new(2, 9, 'c', "ccccccccc".to_string()).is_valid_v2(), false);
}

fn count_valid(validations: &Vec<PasswordVerifier>) -> usize {
    let num_valid = validations.into_iter().filter(|verifier| verifier.is_valid()).count();
    return num_valid;
}

fn count_valid_v2(validations: &Vec<PasswordVerifier>) -> usize {
    let num_valid = validations.into_iter().filter(|verifier| verifier.is_valid_v2()).count();
    return num_valid;
}

fn test_count_valid() {
    let list = vec![
        PasswordVerifier::new(1, 3, 'a', "abcde".to_string()),
        PasswordVerifier::new(1, 3, 'b', "cdefg".to_string()),
        PasswordVerifier::new(2, 9, 'c', "ccccccccc".to_string()),
    ];

    assert_eq!(count_valid(&list), 2)
}

fn test_count_valid_v2() {
    let list = vec![
        PasswordVerifier::new(1, 3, 'a', "abcde".to_string()),
        PasswordVerifier::new(1, 3, 'b', "cdefg".to_string()),
        PasswordVerifier::new(2, 9, 'c', "ccccccccc".to_string()),
    ];

    assert_eq!(count_valid_v2(&list), 1)

}