// use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let data = read_data("src/input.txt");
    let ship = sail_ship(&data);
    println!("Manhattan distance {}", ship.manhattan_distance());

    let ship_with_waypoint = sail_ship_with_waypoint(&data);
    println!("Manhattan distance with waypoint {}", ship_with_waypoint.manhattan_distance());
}

fn read_data(filename: &str) -> Vec<(char, u128)> {
    let file = File::open(filename)
    .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    let directions = reader.lines().map(|a| a.unwrap()).map(|a| (a.chars().next().unwrap(), a.split_at(1).1.parse::<u128>().unwrap())).collect();

    return directions;
}

trait Direction {
    fn get_east(&self) -> u128;
    fn get_west(&self) -> u128;
    fn get_north(&self) -> u128;
    fn get_south(&self) -> u128;

    fn forward(&mut self, amount: u128);
    fn north(&mut self, amount: u128);
    fn south(&mut self, amount: u128);
    fn east(&mut self, amount: u128);
    fn west(&mut self, amount: u128);

    fn manhattan_distance(&self) -> u128  {
        let east_west = self.get_east() as i32 - self.get_west() as i32;
        let north_south = self.get_north() as i32 - self.get_south() as i32;
        return east_west.abs() as u128 + north_south.abs() as u128;
    }
}

struct SelfSailingShip {
    direction: u8,
    east: u128,
    west: u128,
    north: u128,
    south: u128,
}

impl SelfSailingShip {

    pub fn turn(&mut self, degrees: i32) {
        let degrees_modifier = degrees / 90;
        let current_direction = self.direction;
        let new_degrees = modulus(current_direction as i32 + degrees_modifier, 4) as u8;
        self.direction = new_degrees;
    }
}

impl SelfSailingShip {
    // let dirs: HashMap<u8, char> = [(0 as u8, 'N'), (1 as u8, 'W'), (2 as u8, 'S'), (3 as u8, 'E')].iter().cloned().collect();
    fn new() -> SelfSailingShip {
        SelfSailingShip {
            direction: 3,
            east: 0,
            west: 0,
            north: 0,
            south: 0,
        }
    }

    fn take_action(&mut self, instruction: (char, u128)) {
        match instruction.0 {
            'N' => self.north(instruction.1),
            'S' => self.south(instruction.1),
            'E' => self.east(instruction.1),
            'W' => self.west(instruction.1),
            'F' => self.forward(instruction.1),
            'L' => self.turn(instruction.1 as i32),
            'R' => self.turn(-1 * instruction.1 as i32),
            _ => println!("I don't understand"),
        }
    }
}

impl Direction for SelfSailingShip {


    fn get_east(&self) -> u128 {
        return self.east;
    }
    fn get_west(&self) -> u128 {
        return self.west;
    }
    fn get_north(&self) -> u128 {
        return self.north;
    }
    fn get_south(&self) -> u128 {
        return self.south;
    }

    fn forward(&mut self, amount: u128) {
        if self.direction == 0 {
            self.north(amount);
        }
        else if self.direction == 1 {
            self.west(amount);
        }
        else if self.direction == 2 {
            self.south(amount);
        }
        else if self.direction == 3 {
            self.east(amount);
        }
    }

    fn east(&mut self, amount: u128) {
        self.east += amount;
    }
    fn west(&mut self, amount: u128) {
        self.west += amount;
    }
    fn north(&mut self, amount: u128) {
        self.north += amount;
    }
    fn south(&mut self, amount: u128) {
        self.south += amount;
    }
}

struct WayPoint {
    east: u128,
    west: u128,
    north: u128,
    south: u128,
}

impl WayPoint {
    pub fn new() -> WayPoint {
        WayPoint {
            east: 10,
            west: 0,
            north: 1,
            south: 0,
        }
    }

    pub fn total_west(&self) -> i32 {
        return self.west as i32 - self.east as i32;
    }

    pub fn total_north(&self) -> i32 {
        return self.north as i32 - self.south as i32;
    }

    fn east(&mut self, amount: u128) {
        self.east += amount;
    }
    fn west(&mut self, amount: u128) {
        self.west += amount;
    }
    fn north(&mut self, amount: u128) {
        self.north += amount;
    }
    fn south(&mut self, amount: u128) {
        self.south += amount;
    }
    pub fn display_location(&self) {
        println!("North: {}, West: {}, South: {}, East: {}", self.north, self.west, self.south, self.east);
    }
    pub fn rotate(&mut self, amount: i32) {
        let old_north = self.north;
        let old_south = self.south;
        let old_east = self.east;
        let old_west = self.west;
        match modulus(amount, 360) {
            270 => {self.east = old_north; self.south = old_east; self.west = old_south; self.north = old_west},
            180 => {self.east = old_west; self.south = old_north; self.west = old_east; self.north = old_south},
            90 => {self.east = old_south; self.south = old_west; self.west = old_north; self.north = old_east},
            _ => println!("I don't know what to do"),
        }
    }

}

struct ShipWithWayPoint {
    waypoint: WayPoint,
    east: u128,
    west: u128,
    north: u128,
    south: u128,
}

impl ShipWithWayPoint {
    fn new() -> ShipWithWayPoint {
        ShipWithWayPoint {
            waypoint: WayPoint::new(),
            east: 0,
            west: 0,
            north: 0,
            south: 0,
        }
    }
    fn take_action(&mut self, instruction: (char, u128)) {
        match instruction.0 {
            'N' => self.waypoint.north(instruction.1),
            'S' => self.waypoint.south(instruction.1),
            'E' => self.waypoint.east(instruction.1),
            'W' => self.waypoint.west(instruction.1),
            'F' => self.forward(instruction.1),
            'L' => self.waypoint.rotate(instruction.1 as i32),
            'R' => self.waypoint.rotate(-1 * instruction.1 as i32),
            _ => println!("I don't understand"),
        }
    }
}
impl Direction for ShipWithWayPoint {
    // let dirs: HashMap<u8, char> = [(0 as u8, 'N'), (1 as u8, 'W'), (2 as u8, 'S'), (3 as u8, 'E')].iter().cloned().collect();

    fn get_east(&self) -> u128 {
        return self.east;
    }
    fn get_west(&self) -> u128 {
        return self.west;
    }
    fn get_north(&self) -> u128 {
        return self.north;
    }
    fn get_south(&self) -> u128 {
        return self.south;
    }

    fn forward(&mut self, amount: u128) {
        let east_west = self.waypoint.total_west();
        if east_west > 0 {
            self.east(east_west as u128 * amount as u128);
        }
        else {
            self.west(east_west.abs() as u128 * amount as u128);
        }

        let north_south = self.waypoint.total_north();
        if north_south > 0 {
            self.north(north_south as u128 * amount as u128);
        }
        else {
            self.south(north_south.abs() as u128 * amount as u128);
        }
    }

    fn east(&mut self, amount: u128) {
        self.east += amount;
    }
    fn west(&mut self, amount: u128) {
        self.west += amount;
    }
    fn north(&mut self, amount: u128) {
        self.north += amount;
    }
    fn south(&mut self, amount: u128) {
        self.south += amount;
    }
}

fn modulus(a: i32, b: i32) -> i32 {
    return ((a % b) + b) % b;
}

fn sail_ship(directions: &Vec<(char, u128)>) -> SelfSailingShip {
    let mut ship = SelfSailingShip::new();

    for instruction in directions {
        ship.take_action(*instruction);
    }

    return ship;
}

fn sail_ship_with_waypoint(directions: &Vec<(char, u128)>) -> ShipWithWayPoint {
    let mut ship = ShipWithWayPoint::new();

    for instruction in directions {
        ship.take_action(*instruction);
    }

    return ship;
}

mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let input = "F10
N3
F7
R90
F11".lines().map(|a| (a.chars().next().unwrap(), a.split_at(1).1.parse::<u128>().unwrap())).collect();
    
        let ship = sail_ship(&input);
        assert_eq!(ship.manhattan_distance(), 25);

        let ship_2 = sail_ship_with_waypoint(&input);
        assert_eq!(ship_2.manhattan_distance(), 286);
    }
}