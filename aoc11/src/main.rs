use ndarray::Array;
use ndarray::Dim;
use std::fs::File;
use std::io::Read;

fn main() {
    test_find_stable_iteration_and_count_seats();
    let seats = read_data("src/input.txt");
    // let count = find_stable_iteration_and_count_seats(&seats, false);
    // println!("Number of occupied seats is {}", count);

    let count_2 = find_stable_iteration_and_count_seats(&seats, true);
    println!("Number of occupied seats is {}", count_2);
}

fn read_data(filename: &str) -> String {
    let mut file = File::open(filename)
        .expect("Something went wrong reading the file");
    let mut seats = String::new();
    file.read_to_string(&mut seats).expect("Error reading data");

    return seats;
}

fn generate_neighbor_index(index: (usize, usize)) -> Vec<(usize, usize)> {
    if index.0 > 0 && index.1 > 0 {
        return vec![
            (index.0 - 1 , index.1),
            (index.0 - 1 , index.1 - 1),
            (index.0 -1 , index.1 + 1),
            (index.0 + 1 , index.1),
            (index.0 + 1 , index.1 - 1),
            (index.0 + 1 , index.1 + 1),
            (index.0, index.1 - 1),
            (index.0, index.1 + 1),
        ];
    }
    else if index.0 > 0 {
        return vec![
            (index.0 - 1 , index.1),
            (index.0 -1 , index.1 + 1),
            (index.0 + 1 , index.1),
            (index.0 + 1 , index.1 + 1),
            (index.0, index.1 + 1),
        ]; 
    }
    else if index.1 > 0 {
        return vec![
            (index.0 + 1 , index.1),
            (index.0 + 1 , index.1 - 1),
            (index.0 + 1 , index.1 + 1),
            (index.0, index.1 - 1),
            (index.0, index.1 + 1),
        ];
    }
    else {
        return vec![
            (index.0 + 1 , index.1),
            (index.0 + 1 , index.1 + 1),
            (index.0, index.1 + 1),
        ];
    }
}

fn occupied_visible_seat_in_direction(index: (usize, usize), map: &Array<i8, Dim<[usize; 2]>>, x: i8, y: i8) -> bool {
    if (index.0 as i8) + x < 0
    || (index.1 as i8) + y < 0
    {
        return false;
    }

    let mut current_index = ((index.0 as i8 + x) as usize, (index.1 as i8 + y) as usize);
    loop {
        if (current_index.0 as i8) + x >= 0 
        && (current_index.1 as i8) + y >= 0 
        && map.get(current_index).is_some() {
            if *map.get(current_index).unwrap() >= 0 {
                break;
            }
            else {
                current_index = ((current_index.0 as i8 + x) as usize, (current_index.1 as i8 + y) as usize);
            }
        }
        else {
            break;
        }
    }

    if map.get(current_index) == Some(&1) {
        return true;
    }

    return false;

}

fn seats_after_iteration_v2(input_seats: &mut Array<i8, Dim<[usize; 2]>>) -> (bool, &mut Array<i8, Dim<[usize; 2]>>) {
    let mut mask = input_seats.clone();
    let original_mask = mask.clone();

    let mut changed = false;
    for (index, seat) in &mut mask.indexed_iter_mut() {
        if *seat >= 0 {
            let mut occupied_neighbors = 0;
            for direction in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                if occupied_visible_seat_in_direction(index, &original_mask, direction.0, direction.1) {
                    occupied_neighbors += 1;
                }
            }
    
            if occupied_neighbors >= 5 && *seat == 1 {
                changed = true;
                *seat = 0
            }
            else if occupied_neighbors == 0 && *seat == 0 {
                changed = true;
                *seat = 1;
            }
        }
    }
    input_seats.zip_mut_with(&mask, |a, mask| if *a >= 0 { *a = *mask });
    return (changed, input_seats);
}

fn seats_after_iteration(input_seats: &mut Array<i8, Dim<[usize; 2]>>) -> (bool, &mut Array<i8, Dim<[usize; 2]>>) {
    let mut mask = input_seats.clone();
    mask.map_mut(|elem| if *elem < 0 { *elem = 0 });
    let mut original_mask = mask.clone();

    let mut changed = false;
    for (index, seat) in &mut mask.indexed_iter_mut() {
        if seat == &mut 1 {
            let mut number_of_occupied_neighbors = 0;
            for new_index in generate_neighbor_index(index) {
                if original_mask.get(new_index) == Some(&mut 1)
                    || original_mask.get(new_index) == Some(&mut 1) {
                        number_of_occupied_neighbors += 1;
                    }
                }
            if number_of_occupied_neighbors >= 4 {
                changed = true;
                *seat = 0;
            }
        }
        else if seat == &mut 0 {
            let mut adjacent_occupied = false;
            for new_index in generate_neighbor_index(index) {
                if original_mask.get_mut(new_index) == Some(& mut 1) {
                    adjacent_occupied = true;
                }
            }
            if !adjacent_occupied {
                changed = true;
                *seat = 1;
            }
        }
    }
    input_seats.zip_mut_with(&mask, |a, mask| if *a >= 0 { *a = *mask });
    return (changed, input_seats);
}

fn find_stable_iteration(input_seats: &mut Array<i8, Dim<[usize; 2]>>, version_2: bool) -> Array<i8, Dim<[usize; 2]>> {
    let mut last_iteration = input_seats.clone();
    let mut iter_count = 0;
    loop {
        let in_loop = &mut last_iteration.clone();
        let new_iteration = match version_2 {
            false => seats_after_iteration(in_loop),
            true => seats_after_iteration_v2(in_loop),
        };
        if !new_iteration.0 {
            break;
        }
        else {
            last_iteration = new_iteration.1.clone();
        }
        iter_count +=1;
        if iter_count > 1000 {
            println!("breaking early");
            break;
        }
    }
    return last_iteration;
}

fn count_seats(input_seats: &mut Array<i8, Dim<[usize; 2]>>) -> usize {
    let mask: Array<i8, Dim<[usize; 2]>> = input_seats.map_mut(|elem| if *elem < 0 { 0 } else { *elem });
    return mask.fold(0, |acc, elem| acc + (*elem as usize));
}

fn find_stable_iteration_and_count_seats(input_seats: &String, version_2: bool) -> usize {
    let mut seat_matrix = seat_to_matrix(input_seats);
    let mut stable_seats = find_stable_iteration(&mut seat_matrix, version_2);
    return count_seats(&mut stable_seats);
}

fn seat_to_matrix(seats: &String) -> Array<i8, Dim<[usize; 2]>> {
    let n = seats.find("\n").unwrap();
    let data: Vec<i8> = seats.chars().filter(|s| s != &'\n').map(|s| match s { 'L' => 0, '.' => -1, _ => 1 }).collect();
    let m = data.len() / n;

    let matrix = Array::from_shape_vec((m, n), data).unwrap();
    return matrix;
}

fn test_find_stable_iteration_and_count_seats() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".to_string();
    let count_1 = find_stable_iteration_and_count_seats(&input, false);
    assert_eq!(count_1, 37);


    let count_2 = find_stable_iteration_and_count_seats(&input, true);
    assert_eq!(count_2, 26);
}