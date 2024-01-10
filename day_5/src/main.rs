use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const IS_SECOND_PART: bool = false;
fn main() {
    let file_name = "./src/input.txt";

    match read_lines(file_name) {
        Ok(lines) => {
            let mut n_line: u64 = 0;
            
            let mut seeds: Vec<u64> = vec![];
            let mut categories: Vec<CategoryMap> = vec![];

            let mut source = String::from("");
            let mut destination = String::from("");
            let mut descriptions = vec![];

            for line in lines {
                match line {
                    Ok(line) => {
                        if n_line == 0 {
                            seeds = (line[7..].to_string())
                                        .split(" ")
                                        .collect::<Vec<&str>>()
                                        .iter()
                                        .filter_map(|&x| x.parse().ok())
                                        .collect();
                        } else if n_line == 1 {
                            
                        } else if line.contains("-to-") {
                            let first_part = &line[0..(line.len() - 5)];
                            source = first_part.split("-to-").collect::<Vec<&str>>()[0].to_string();
                            destination = first_part.split("-to-").collect::<Vec<&str>>()[1].to_string();
                        } else if line.is_empty() {
                            categories.push(CategoryMap::new(source, destination, descriptions));
                            source = String::from("");
                            destination = String::from("");
                            descriptions = vec![];
                        } else {
                            descriptions.push(line);
                        }
                    }
                    Err(error) => {
                        println!("Error in line: {}", error);
                    }
                }
                n_line += 1;
            }
            categories.push(CategoryMap::new(source, destination, descriptions));

            if IS_SECOND_PART {
                let mut ranges: Vec<(u64, u64)> = vec![];
                for n_and_start in seeds.iter().step_by(2).enumerate() {
                    ranges.push((n_and_start.1.to_owned(), n_and_start.1.to_owned() + seeds[(n_and_start.0 * 2) + 1]));
                }
                
                
                let lowest_location: u64 = 0;
                println!("The lowest location number that corresponds to any of the initial seeds is: {}", lowest_location);
            } else {
                let mut locations: Vec<u64> = vec![];
                for seed in seeds.iter() {
                    let mut location = *seed;
                    print!("Seed -> Location: {}", location);
                    for category in categories.iter() {
                        location = category.result(location);
                        print!(" -> {}", location);
                    }
                    println!("");
                    locations.push(location);
                }

                let lowest_location: u64 = *locations.iter().min().unwrap();
                println!("The lowest location number that corresponds to any of the initial seeds is: {}", lowest_location);
            }

        }
        Err(error) => {
            println!("Error in file: {}", error);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[warn(dead_code)]
#[warn(unused_variables)]
#[derive(Debug)]
struct CategoryMap {
    source: String,
    destination: String,
    maps: Vec<InternalMap>
}

impl CategoryMap {
    fn new(source: String, destination: String, descriptions: Vec<String>) -> Self {
        let mut maps: Vec<InternalMap> = vec![];
        for description in descriptions.iter() {
            maps.push(InternalMap::new(description.to_string()));
        }

        CategoryMap {
            source,
            destination,
            maps
        }
    }

    fn result(&self, source: u64) -> u64 {
        for map in self.maps.iter() {
            if map.is_on_range(source) {
                return map.result(source)
            }
        }
        return source
    }

}

#[cfg(test)]
mod test_category_map {
    use super::*;

    fn initialize_seed_to_soil_map() -> CategoryMap {
        let source = String::from("seed");
        let destination = String::from("soil");
        let description = vec![
            String::from("50 98 2"),
            String::from("52 50 48")
        ];
        return CategoryMap::new(source, destination, description);
    }

    #[test]
    fn test_no_mapped() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(0), 0);
    }

    #[test]
    fn test_mapped_fisrt_map_a() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(98), 50);
    }

    #[test]
    fn test_mapped_fisrt_map_b() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(99), 51);
    }

    #[test]
    fn test_mapped_second_map_a() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(50), 52);
    }

    #[test]
    fn test_mapped_second_map_b() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(51), 53);
    }
}

#[derive(Debug)]
struct InternalMap {
    source_range_start: u64,
    source_range_end: u64,
    destination_range_start: u64,
    destination_range_end: u64
}

impl InternalMap {
    fn new(description: String) -> Self {
        let splited_description: Vec<u64> = description.splitn(3, ' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|&x| x.parse().ok())
        .collect();

        if splited_description.len() < 3 {
            print!("{:?}", splited_description);
            panic!("invalid description");
        }

        let range_length = splited_description[2];
        InternalMap {
            source_range_start: splited_description[1],
            source_range_end: splited_description[1] + range_length - 1,
            destination_range_start: splited_description[0],
            destination_range_end: splited_description[0] + range_length - 1
        }
    }

    fn is_on_range(&self, source: u64) -> bool {
        return (self.source_range_start <= source) && (source <= self.source_range_end);
    }

    fn result(&self, source: u64) -> u64 {
        if self.is_on_range(source) {
            return self.destination_range_start + (source - self.source_range_start); 
        } else {
            panic!("out of range");
        }
    }
}

#[cfg(test)]
mod test_internal_map {
    use super::*;

    fn initialize_seed_to_soil_internal_map() -> InternalMap {
        return InternalMap::new(String::from("50 98 2"));
    }

    #[test]
    fn test_mapped_first() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.result(98), 50);
    }

    #[test]
    fn test_mapped_second() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.result(99), 51);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn test_mapped_out_of_range() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        my_seed_to_soil_map.result(1);
    }

    #[test]
    fn test_is_not_on_range_low() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.is_on_range(97), false);
    }

    #[test]
    fn test_is_on_range() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.is_on_range(98), true);
    }

    #[test]
    fn test_is_not_on_range_greater() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.is_on_range(100), false);
    }

    #[test]
    #[should_panic(expected = "invalid description")]
    fn test_invalid_description_empty_string() {
        InternalMap::new(String::from(""));
    }

    #[test]
    #[should_panic(expected = "invalid description")]
    fn test_invalid_description() {
        InternalMap::new(String::from("01 01"));
    }

}