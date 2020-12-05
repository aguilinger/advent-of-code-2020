use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let mut test_map = map_test();
    test_count_trees(&mut test_map);
    test_map.reset();

    test_multiples_tree_counts_varied_slopes(&mut test_map);

    let mut map = read_data("src/input.txt").unwrap();
    println!("slope -3, 1: {}", map.slide_with_slope_count_trees(3, 1));

    let slopes = vec![
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2],
    ];
    map.reset();
    println!("multiple {}", map.multiple_tree_counts_varied_slopes(slopes))
}

fn read_data(filename: &str) -> Result<Map, Error> {
    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    let matrix = reader.lines().map(|line| line.unwrap())
        .map(|line_to_array| line_to_array.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();
    let map = Map::new(matrix);

    return Ok(map);
}

struct Map {
    x_position: usize,
    y_position: usize,
    pattern_width: usize,
    map_height: usize,
    map: Vec<Vec<char>>,
}

impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Map {
        let pattern_width = map[0].len();
        let map_height = map.len();
        Map {
            x_position: 0,
            y_position: 0,
            pattern_width: pattern_width,
            map_height: map_height,
            map: map,
        }
    }

    pub fn reset(&mut self) {
        self.x_position = 0;
        self.y_position = 0;
    }

    pub fn slide_with_slope_count_trees(&mut self, x_slope: usize, y_slope: usize) -> i64 {
        let mut tree_count = 0;

        loop {
            // We've reached the bottom of the map
            if self.y_position > self.map_height - 1 {
                break;
            }

            if self.map[self.y_position][self.x_position % self.pattern_width] == '#' {
                tree_count+=1;
            }

            self.y_position += y_slope;
            self.x_position += x_slope;

        }

        return tree_count;
    }

    pub fn multiple_tree_counts_varied_slopes(&mut self, slopes: Vec<[usize; 2]>) -> i64 {
        let mut multiple = 1;
        for slope in slopes {
            self.reset();
            let trees = self.slide_with_slope_count_trees(slope[0], slope[1]);
            multiple *= trees;
        }
        return multiple;

    }
}

fn test_multiples_tree_counts_varied_slopes(test_map: &mut Map) {
    let test_slopes = vec![
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2],
    ];

    assert_eq!(test_map.multiple_tree_counts_varied_slopes(test_slopes), 336);
}

fn test_count_trees(test_map: &mut Map) {
    assert_eq!(test_map.slide_with_slope_count_trees(3, 1), 7);
}

fn map_test() -> Map {
    let map_pattern = 
    "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

    let map_matrix: Vec<Vec<char>> = map_pattern.lines()
        .map(|line| line)
        .map(|line_to_array| line_to_array.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();

    return Map::new(map_matrix);

}