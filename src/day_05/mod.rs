use std::{
    cmp,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    vec,
};

fn create_predicate(
    source_id: &u64,
) -> impl FnMut((usize, (&Range<u64>, &Range<u64>))) -> Option<u64> + '_ {
    move |(_index, (source, destination))| {
        if source.contains(source_id) {
            let index_in_source = source_id - source.start;
            return Some(destination.start + index_in_source);
        }

        None
    }
}

fn create_destination_predicate(
    destination_id: &u64,
) -> impl FnMut((usize, (&Range<u64>, &Range<u64>))) -> Option<u64> + '_ {
    move |(_index, (source, destination))| {
        if destination.contains(destination_id) {
            let index_in_destination = destination_id - destination.start;
            return Some(source.start + index_in_destination);
        }

        None
    }
}

fn get_location_from_seed_id(
    maps: &HashMap<(String, String), HashMap<Range<u64>, Range<u64>>>,
    seed_id: u64,
) -> u64 {
    let soil = maps
        .get(&("seed".to_string(), "soil".to_string()))
        .unwrap()
        .iter()
        .enumerate()
        .find_map(create_predicate(&seed_id))
        .unwrap_or(seed_id.clone());
    let fertilizer = maps
        .get(&("soil".to_string(), "fertilizer".to_string()))
        .unwrap()
        .iter()
        .enumerate()
        .find_map(create_predicate(&soil))
        .unwrap_or(soil);
    let water = maps
        .get(&("fertilizer".to_string(), "water".to_string()))
        .unwrap()
        .iter()
        .enumerate()
        .find_map(create_predicate(&fertilizer))
        .unwrap_or(fertilizer);
    let light = maps
        .get(&("water".to_string(), "light".to_string()))
        .unwrap()
        .iter()
        .enumerate()
        .find_map(create_predicate(&water))
        .unwrap_or(water);
    let temperature = maps
        .get(&("light".to_string(), "temperature".to_string()))
        .unwrap()
        .iter()
        .enumerate()
        .find_map(create_predicate(&light))
        .unwrap_or(light);
    let humidity = maps
        .get(&("temperature".to_string(), "humidity".to_string()))
        .unwrap()
        .iter()
        .enumerate()
        .find_map(create_predicate(&temperature))
        .unwrap_or(temperature);
    let location = maps
        .get(&("humidity".to_string(), "location".to_string()))
        .unwrap()
        .iter()
        .enumerate()
        .find_map(create_predicate(&humidity))
        .unwrap_or(humidity);

    // println!("seed_id: {:?}", seed_id);
    // println!("soil: {:?}", soil);
    // println!("fertilizer: {:?}", fertilizer);
    // println!("water: {:?}", water);
    // println!("light: {:?}", light);
    // println!("temperature: {:?}", temperature);
    // println!("humidity: {:?}", humidity);
    // println!("location: {:?}", location);

    location
}

pub fn process_all_from_start(
    seeds_ids_ranges: Vec<Range<u64>>,
    maps: HashMap<(String, String), HashMap<Range<u64>, Range<u64>>>,
) -> u64 {
    let mut lowest_location = u64::MAX;
    for range in seeds_ids_ranges {
        let range_start = range.start;
        let range_end = range.end;
        let range_len = range_end - range_start;
        let mut percentage = 0;
        for seed_id in range {
            let new_percentage = (seed_id - range_start) * 100 / range_len as u64;
            if new_percentage != percentage {
                println!(
                    "processing range: {:?}..{:?}, percentage: {:?}",
                    range_start, range_end, new_percentage
                );
                percentage = new_percentage;
            }

            let location = get_location_from_seed_id(&maps, seed_id);
            let location = location.clone();
            if lowest_location > location {
                println!(
                    "new lowest_location found: {:?} for seed_id: {:?}",
                    location, seed_id
                );
                lowest_location = location;
            }
        }
    }

    lowest_location
}

pub fn process_all_from_end(
    seeds_ids_ranges: Vec<Range<u64>>,
    maps: HashMap<(String, String), HashMap<Range<u64>, Range<u64>>>,
) -> u64 {
    'location_loop: for location in 0..1166153960 {
        // println!("location: {:?}", location);

        let humidity = maps
            .get(&("humidity".to_string(), "location".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_destination_predicate(&location));
        if humidity.is_none() {
            continue;
        }

        let humidity = humidity.unwrap();

        let temperature = maps
            .get(&("temperature".to_string(), "humidity".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_destination_predicate(&humidity));

        if temperature.is_none() {
            continue;
        }

        let temperature = temperature.unwrap();

        let light = maps
            .get(&("light".to_string(), "temperature".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_destination_predicate(&temperature));
        if light.is_none() {
            continue;
        }

        let light = light.unwrap();

        let water = maps
            .get(&("water".to_string(), "light".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_destination_predicate(&light))
            .unwrap_or(light);

        let fertilizer = maps
            .get(&("fertilizer".to_string(), "water".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_destination_predicate(&water))
            .unwrap_or(water);

        let soil = maps
            .get(&("soil".to_string(), "fertilizer".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_destination_predicate(&fertilizer))
            .unwrap_or(fertilizer);

        let seed_id = maps
            .get(&("seed".to_string(), "soil".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_destination_predicate(&soil))
            .unwrap_or(soil);

        println!("location: {:?} <-> seed: {:?} | humidity {:?} temperature: {:?} light: {:?} water: {:?} fertilizer: {:?} soil: {:?}", location, seed_id, humidity, temperature, light, water, fertilizer, soil);

        let inverse_location = get_location_from_seed_id(&maps, seed_id);

        for range in seeds_ids_ranges.clone() {
            if range.contains(&seed_id) {
                println!("inverse_location {:?}", inverse_location);
                println!("contained in seeds_ids_ranges");
                break 'location_loop;
            }
        }

        return inverse_location;
    }

    return u64::MAX;
}

pub fn execute_day_five_part_one() {
    let file_path = "./src/day_05/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut map_declaration: Option<(String, String)> = None;
    let mut seeds_ids: Vec<u64> = vec![];
    let mut maps: HashMap<(String, String), HashMap<Range<u64>, Range<u64>>> = HashMap::new();
    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                if line_index == 0 {
                    let parts: Vec<&str> = text.split(':').map(|i| i.trim()).collect();
                    seeds_ids = parts[1]
                        .split_whitespace()
                        .map(|i| u64::from_str_radix(i, 10).unwrap())
                        .collect();
                } else {
                    let split_by_whitespace: Vec<&str> = text.split_whitespace().collect();

                    match split_by_whitespace.len() {
                        0 => map_declaration = None,
                        2 => {
                            let map_name_parts: Vec<&str> =
                                split_by_whitespace[0].split('-').collect();
                            map_declaration =
                                Some((map_name_parts[0].into(), map_name_parts[2].into()));
                            // println!("parts {:?}", map_name_parts);
                        }
                        3 => {
                            let mut map: HashMap<Range<u64>, Range<u64>> = HashMap::new();

                            let destination_range_start =
                                u64::from_str_radix(split_by_whitespace[0], 10).unwrap();
                            let source_range_start =
                                u64::from_str_radix(split_by_whitespace[1], 10).unwrap();
                            let range_length =
                                u64::from_str_radix(split_by_whitespace[2], 10).unwrap();

                            map.insert(
                                source_range_start..source_range_start + range_length,
                                destination_range_start..destination_range_start + range_length,
                            );

                            let global_map_key = map_declaration.clone().unwrap();
                            match maps.get(&global_map_key) {
                                Some(existing_map) => {
                                    let mut combined_map = existing_map.clone();
                                    combined_map.extend(map);
                                    maps.insert(global_map_key, combined_map);
                                }
                                None => {
                                    maps.insert(global_map_key, map);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let mut lowest_location = u64::MAX;
    for seed_id in seeds_ids {
        let soil = maps
            .get(&("seed".to_string(), "soil".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_predicate(&seed_id))
            .unwrap_or(seed_id.clone());
        let fertilizer = maps
            .get(&("soil".to_string(), "fertilizer".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_predicate(&soil))
            .unwrap_or(soil);
        let water = maps
            .get(&("fertilizer".to_string(), "water".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_predicate(&fertilizer))
            .unwrap_or(fertilizer);
        let light = maps
            .get(&("water".to_string(), "light".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_predicate(&water))
            .unwrap_or(water);
        let temperature = maps
            .get(&("light".to_string(), "temperature".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_predicate(&light))
            .unwrap_or(light);
        let humidity = maps
            .get(&("temperature".to_string(), "humidity".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_predicate(&temperature))
            .unwrap_or(temperature);
        let location = maps
            .get(&("humidity".to_string(), "location".to_string()))
            .unwrap()
            .iter()
            .enumerate()
            .find_map(create_predicate(&humidity))
            .unwrap_or(humidity);

        // println!("seed_id: {:?}", seed_id);
        // println!("soil: {:?}", soil);
        // println!("fertilizer: {:?}", fertilizer);
        // println!("water: {:?}", water);
        // println!("light: {:?}", light);
        // println!("temperature: {:?}", temperature);
        // println!("humidity: {:?}", humidity);
        // println!("location: {:?}", location);

        lowest_location = cmp::min(lowest_location, location.clone());
    }

    println!("Day 5_1: {:?}", lowest_location);
}

pub fn execute_day_five_part_two() {
    let file_path = "./src/day_05/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut map_declaration: Option<(String, String)> = None;
    let mut seeds_ids: Vec<u64> = vec![];
    let mut seeds_ids_ranges: Vec<Range<u64>> = vec![];
    let mut maps: HashMap<(String, String), HashMap<Range<u64>, Range<u64>>> = HashMap::new();
    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                if line_index == 0 {
                    let parts: Vec<&str> = text.split(':').map(|i| i.trim()).collect();
                    seeds_ids = parts[1]
                        .split_whitespace()
                        .map(|i| u64::from_str_radix(i, 10).unwrap())
                        .collect();

                    let mut i = 0;
                    while i < seeds_ids.len() {
                        seeds_ids_ranges.push(seeds_ids[i]..seeds_ids[i] + seeds_ids[i + 1] - 1);
                        i += 2;
                    }
                } else {
                    let split_by_whitespace: Vec<&str> = text.split_whitespace().collect();

                    match split_by_whitespace.len() {
                        0 => map_declaration = None,
                        2 => {
                            let map_name_parts: Vec<&str> =
                                split_by_whitespace[0].split('-').collect();
                            map_declaration =
                                Some((map_name_parts[0].into(), map_name_parts[2].into()));
                            // println!("parts {:?}", map_name_parts);
                        }
                        3 => {
                            let mut map: HashMap<Range<u64>, Range<u64>> = HashMap::new();

                            let destination_range_start =
                                u64::from_str_radix(split_by_whitespace[0], 10).unwrap();
                            let source_range_start =
                                u64::from_str_radix(split_by_whitespace[1], 10).unwrap();
                            let range_length =
                                u64::from_str_radix(split_by_whitespace[2], 10).unwrap();

                            map.insert(
                                source_range_start..source_range_start + range_length,
                                destination_range_start..destination_range_start + range_length,
                            );

                            // for i in 0..range_length {
                            //     map.insert(source_range_start + i, destination_range_start + i);
                            // }

                            let global_map_key = map_declaration.clone().unwrap();
                            match maps.get(&global_map_key) {
                                Some(existing_map) => {
                                    let mut combined_map = existing_map.clone();
                                    combined_map.extend(map);
                                    maps.insert(global_map_key, combined_map);
                                }
                                None => {
                                    maps.insert(global_map_key, map);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let lowest_location = process_all_from_start(seeds_ids_ranges, maps);
    // let lowest_location = process_all_from_end(seeds_ids_ranges, maps);
    println!("Day 5_2: {:?}", lowest_location);
}
