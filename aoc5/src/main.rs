use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    test_seat();
    test_largest_seat_id();

    let passes = read_data("src/input.txt");
    let largest_seat = largest_seat_id(&passes);
    println!("{}", largest_seat);

    let filled_plane = fill_plane(&passes);
    println!("{:?}", filled_plane);

}

fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename)
    .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    let passes = reader.lines().map(|line| line.unwrap()).collect();

    return passes;
}

struct Seat {
    boarding_pass: Vec<String>,
    row: Option<u8>,
    column: Option<u8>,
    seat_id: Option<u16>,
}

impl Seat {
    pub fn new(boarding_pass: Vec<String>) -> Seat {
        Seat {
            boarding_pass: boarding_pass,
            row: None,
            column: None,
            seat_id: None,
        }
    }

    pub fn calculate_row(&mut self) -> u8 {
        if self.row.is_some() {
            return self.row.unwrap();
        }

        let mut bounds = &mut (u8::from(0), u8::from(127));
        for half in self.boarding_pass[..7].iter() {
            binary_reduce(&mut bounds, half == "F");
        }

        let mut row = bounds.1;
        if self.boarding_pass[6] == "F" {
            row = bounds.0;
        }

        self.row = Some(row);
        return row
    }

    pub fn calculate_column(&mut self) -> u8 {
        if self.column.is_some() {
            return self.column.unwrap();
        }

        let mut bounds = &mut (u8::from(0), u8::from(7));
        for half in self.boarding_pass[7..].iter() {
            binary_reduce(&mut bounds, half == "L");
        }

        let mut column = bounds.1;
        if self.boarding_pass.last().unwrap() == "L" {
            column = bounds.0;
        }

        self.column = Some(column);
        return column
    }

    pub fn calculate_seat_id(&mut self) -> u16 {
        if self.seat_id.is_some() {
            return self.seat_id.unwrap();
        }

        let column = self.calculate_column() as u16;
        let row = self.calculate_row() as u16;

        let seat_id: u16 = (row * 8) + column;

        self.seat_id = Some(seat_id);
        return seat_id;
    }
}

fn binary_reduce(bounds: &mut (u8, u8), take_lower: bool) -> &mut (u8, u8) {
    if take_lower {
        let old_bounds = (bounds.0, bounds.1);
        bounds.1 = f32::round(old_bounds.1 as f32 - (old_bounds.1 as f32 - old_bounds.0 as f32 + 1.0) / 2.0) as u8;
    }
    else {
        let old_bounds = (bounds.0, bounds.1);
        bounds.0 = f32::round(old_bounds.0 as f32 + (old_bounds.1 as f32 - old_bounds.0 as f32 + 1.0) / 2.0) as u8;
    }
    return bounds;
}

fn largest_seat_id(passes: &Vec<String>) -> u16 {
    let mut max_seat_id = 0;
    for pass in passes {
        let mut seat = Seat::new(pass.chars().map(|c| c.to_string()).collect());
        if seat.calculate_seat_id() > max_seat_id {
            max_seat_id = seat.calculate_seat_id();
        }
    }

    return max_seat_id;
}

fn initialize_empty_plane() -> HashMap<u8, HashMap<u8, u16>> {
    let mut plane = HashMap::new();
    for row in 0..127 {
        plane.entry(row).or_insert(HashMap::new());
    }
    return plane;
}

fn fill_plane(passes: &Vec<String>) -> HashMap<u8, HashMap<u8, u16>> {
    let mut plane = initialize_empty_plane();
    for pass in passes {
        let mut seat = Seat::new(pass.chars().map(|c| c.to_string()).collect());
        seat.calculate_seat_id();
        let column = plane.get_mut(&seat.row.unwrap()).unwrap();
        column.entry(seat.column.unwrap()).or_insert(seat.seat_id.unwrap());
    }

    return plane;
}

fn test_seat() {
    let mut seat = Seat::new("FBFBBFFRLR".chars().map(|c| c.to_string()).collect());
    assert_eq!(seat.calculate_row(), 44);
    assert_eq!(seat.calculate_column(), 5);
    assert_eq!(seat.calculate_seat_id(), 357);

    let mut seat = Seat::new("BFFFBBFRRR".chars().map(|c| c.to_string()).collect());
    assert_eq!(seat.calculate_row(), 70);
    assert_eq!(seat.calculate_column(), 7);
    assert_eq!(seat.calculate_seat_id(), 567);

    let mut seat = Seat::new("FFFBBBFRRR".chars().map(|c| c.to_string()).collect());
    assert_eq!(seat.calculate_row(), 14);
    assert_eq!(seat.calculate_column(), 7);
    assert_eq!(seat.calculate_seat_id(), 119);

    let mut seat = Seat::new("BBFFBBFRLL".chars().map(|c| c.to_string()).collect());
    assert_eq!(seat.calculate_row(), 102);
    assert_eq!(seat.calculate_column(), 4);
    assert_eq!(seat.calculate_seat_id(), 820);
}

fn test_largest_seat_id() {
    assert_eq!(largest_seat_id(&vec![
        "FBFBBFFRLR".to_string(),
        "BFFFBBFRRR".to_string(),
        "FFFBBBFRRR".to_string(),
        "BBFFBBFRLL".to_string(),
    ]), 820)
}