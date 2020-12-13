use std::fs::File;
use std::io::Read;
use modinverse::egcd;

fn main() {
    let schedule = read_data("src/input.txt");

    let bus = find_earliest_bus(&schedule);
    println!("Earliest bus multiple {}", bus.1 * bus.0);

    let time = find_time_that_fits_pattern(&schedule);
    println!("Earliest time that fits {}", time);

}

fn read_data(filename: &str) -> String {
    let mut file = File::open(filename)
        .expect("Something went wrong reading the file");
    let mut schedule = String::new();
    file.read_to_string(&mut schedule).expect("Error reading data");

    return schedule;
}

fn parse_schedule_sparse(schedule: &String) -> Vec<i128> {
    let mut schedule_lines = schedule.lines();

    schedule_lines.next();

    let buses = schedule_lines.next().unwrap()
        .split(",").collect::<Vec<&str>>().iter()
        .map(|bus| if *bus == "x" { 0 } else { bus.parse().unwrap() })
        .collect();

    return buses;
}

fn parse_schedule_condensed(schedule: &String) -> (i128, Vec<i128>) {
    let mut schedule_lines = schedule.lines();

    let departure_time = schedule_lines.next().unwrap().parse().unwrap();

    let buses = schedule_lines.next().unwrap()
        .split(",").collect::<Vec<&str>>().iter()
        .filter(|bus| *bus != &"x")
        .map(|bus| bus.parse().unwrap() )
        .collect();

    return (departure_time, buses);
}

fn find_earliest_bus(schedule: &String) -> (i128, i128) {
    let time_and_schedule_condensed = parse_schedule_condensed(schedule);

    let target_time = time_and_schedule_condensed.0;

    let mut smallest_wait = 100000000000;
    let mut smallest_wait_bus = 0;
    for bus in time_and_schedule_condensed.1 {
        let wait_time = modulus(((target_time + bus) / bus) * bus, target_time);
        if wait_time < smallest_wait {
            smallest_wait = wait_time;
            smallest_wait_bus = bus;
        }
    }

    return (smallest_wait, smallest_wait_bus);
}

fn modulus(a: i128, b: i128) -> i128 {
    return ((a % b) + b) % b;
}
 
fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(buses: Vec<(i128, i128)>) -> i128 {
    let buses_multiple = buses.iter().fold(1, |acc, bus| acc * bus.1);
 
    let mut sum = 0;
    for (distance, bus) in buses {
        let buses_multiple_except_current = buses_multiple / bus;
        sum += distance * mod_inv(buses_multiple_except_current, bus).unwrap() * buses_multiple_except_current;
    }
 
    return sum % buses_multiple;
}

fn find_time_that_fits_pattern(schedule: &String) -> i128 {
    let schedule = parse_schedule_sparse(schedule);

    let mut buses = Vec::new();
    for (index, bus) in schedule.iter().enumerate() {
        if *bus != 0 {
            buses.push((modulus(-1 * index as i128, *bus), *bus));
        }
    }
    return chinese_remainder(buses);
}

mod tests {
    use super::*;

    #[test]
    fn test_earliest_bus() {

        let test_input = "939
17,13,x,x,59,x,31,19".to_string();
    
        let bus = find_earliest_bus(&test_input);
        assert_eq!(bus.1 * bus.0, 295);
    }

    #[test]
    fn test_bus_pattern() {

        let test_input = "939
7,13,x,x,59,x,31,19".to_string();
        let timestamp = find_time_that_fits_pattern(&test_input);
        assert_eq!(timestamp, 1068781);

        let test_2 = "1
17,x,13,19".to_string();
        let timestamp_2 = find_time_that_fits_pattern(&test_2);
        assert_eq!(timestamp_2, 3417);

        let test = "1
67,7,59,61".to_string();
        let timestamp = find_time_that_fits_pattern(&test);
        assert_eq!(timestamp, 754018);

        let test = "1
67,x,7,59,61".to_string();
        let timestamp = find_time_that_fits_pattern(&test);
        assert_eq!(timestamp, 779210);

        let test = "1
67,7,x,59,61".to_string();
        let timestamp = find_time_that_fits_pattern(&test);
        assert_eq!(timestamp, 1261476);

        let test = "1
1789,37,47,1889".to_string();
        let timestamp = find_time_that_fits_pattern(&test);
        assert_eq!(timestamp, 1202161486);
    }
}