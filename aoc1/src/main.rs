use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    test_sum_to_target();
    test_multiple_expenses();
    test_full();

    let input = read_data("src/input.txt");
    println!("{:?}", multiple_from_expenses(input.unwrap()))
}

fn read_data(filename: &str) -> Result<Vec<i64>, Error> {
    let mut int_data = Vec::new();

    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        int_data.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }

    Ok(int_data)

}

fn sum_to_target(input: Vec<i64>, target: i64) -> Vec<[i64; 2]> {
    let mut sums = Vec::new();
    for (index, expense) in input.iter().enumerate() {
        let mut potential_sum = [*expense; 2];
        for expense_2 in input[index..].iter() {
            if expense + expense_2 == target {
                potential_sum[1] = *expense_2;
                sums.push(potential_sum);
            }
        }
    }

    return sums;

}

fn multiply_expenses(expense_pairs: Vec<[i64; 2]>) -> Vec<i64> {
    let mut multiples = Vec::new();
    for expense_pair in expense_pairs.iter() {
        multiples.push(expense_pair[0] * expense_pair[1])
    }

    return multiples
}

fn multiple_from_expenses(input: Vec<i64>) -> Vec<i64> {
    let sums = sum_to_target(input, 2020);
    let multiples = multiply_expenses(sums);
    return multiples
}

fn test_sum_to_target() {
    let test_input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(sum_to_target(test_input, 2020), vec![[1721, 299]])
}

fn test_multiple_expenses() {
    let test_input = vec![[1721, 299]];
    assert_eq!(multiply_expenses(test_input), vec![514579])
}

fn test_full() {
    let test_input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(multiple_from_expenses(test_input), vec![514579])
}

