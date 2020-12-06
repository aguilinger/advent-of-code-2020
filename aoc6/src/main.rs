use std::collections::HashSet;
use std::fs::File;
use std::io::{Read};

fn main() {
    test_group_questions_count();
    test_group_questions_count_v2();
    test_sum_of_group_answers();

    let answers = read_data("src/input.txt");
    println!("version 1 count {}", sum_of_group_answers(&answers, false));
    println!("version 2 count {}", sum_of_group_answers(&answers, true));
}


fn read_data(filename: &str) -> String {
    let mut file = File::open(filename)
        .expect("Something went wrong reading the file");
    let mut answer_block = String::new();
    file.read_to_string(&mut answer_block).expect("Error reading data");

    return answer_block;
}

fn sum_of_group_answers(group_answer_blocks: &String, version_2: bool) -> usize {
    let mut sum = 0;

    for record in group_answer_blocks.split("\n\n"){
        if version_2 {
            sum += group_questions_count_v2(record.to_string());
        }
        else {
            sum += group_questions_count(record.to_string());
        }
    }

    return sum;

}

fn group_questions_count(group_answers: String) -> usize {
    let mut question_set = HashSet::new();

    for person_answer in group_answers.lines() {
        for question in person_answer.chars() {
            question_set.insert(question);
        }
    }

    return question_set.len()
}

fn group_questions_count_v2(group_answers: String) -> usize {
    let mut question_set = HashSet::new();

    let groups = group_answers.lines().collect::<Vec<&str>>();

    for group_1_answers in groups.first().unwrap().chars() {
        question_set.insert(group_1_answers);
    }

    let mut surviving_question_set = question_set;
    for person_answer in groups[1..].iter() {
        let mut new_surviving_question_set = HashSet::new();
        for answer in person_answer.chars() {
            if surviving_question_set.contains(&answer) {
                new_surviving_question_set.insert(answer);
            }
        }

        surviving_question_set = new_surviving_question_set;

    }

    return surviving_question_set.len()
}

fn test_group_questions_count() {
    let group_1 = "abc".to_string();
    assert_eq!(group_questions_count(group_1), 3);

    let group_2 = "a
b
c".to_string();
    assert_eq!(group_questions_count(group_2), 3);

    let group_3 = "ab
ac".to_string();
    assert_eq!(group_questions_count(group_3), 3);


    let group_4 = "a
a
a
a".to_string();
    assert_eq!(group_questions_count(group_4), 1);

    let group_5 = "b".to_string();
    assert_eq!(group_questions_count(group_5), 1);
}

fn test_group_questions_count_v2() {
    let group_1 = "abc".to_string();
    assert_eq!(group_questions_count_v2(group_1), 3);

    let group_2 = "a
b
c".to_string();
    assert_eq!(group_questions_count_v2(group_2), 0);

    let group_3 = "ab
ac".to_string();
    assert_eq!(group_questions_count_v2(group_3), 1);

    let group_4 = "a
a
a
a".to_string();
    assert_eq!(group_questions_count_v2(group_4), 1);

    let group_5 = "b".to_string();
    assert_eq!(group_questions_count_v2(group_5), 1);
}

fn test_sum_of_group_answers() {
    let block = "abc

a
b
c

ab
ac

a
a
a
a

b".to_string();
    assert_eq!(sum_of_group_answers(&block, false), 11);

    assert_eq!(sum_of_group_answers(&block, true), 6);
}