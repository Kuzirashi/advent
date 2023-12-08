use std::{
    cmp,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute_day_two_part_one() {
    let file_path = "./src/day_02/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut games_within_limit: Vec<u32> = vec![];
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let a: Vec<&str> = text.split(':').collect();
                let id: u32 =
                    u32::from_str_radix(a[0].split(' ').collect::<Vec<&str>>()[1], 10).unwrap();
                let split_by_semicolon: Vec<&str> = a[1].split(';').map(|i| i.trim()).collect();

                let mut max_cubes_map: HashMap<String, u32> =
                    HashMap::from([("red".into(), 0), ("green".into(), 0), ("blue".into(), 0)]);

                for round in split_by_semicolon.iter() {
                    let split_by_comma: Vec<&str> = round.split(',').map(|i| i.trim()).collect();

                    for cube_tuple in split_by_comma.iter() {
                        let cube_tuple_split: Vec<&str> =
                            cube_tuple.split(' ').map(|i| i.trim()).collect();
                        let cube_amount: u32 =
                            u32::from_str_radix(cube_tuple_split[0], 10).unwrap();
                        let cube_color = cube_tuple_split[1];

                        let existing_max_cube_amount_shown =
                            max_cubes_map.get(cube_color.into()).unwrap();
                        max_cubes_map.insert(
                            cube_color.into(),
                            cmp::max(*existing_max_cube_amount_shown, cube_amount),
                        );
                    }
                }

                if max_cubes_map.get("blue").unwrap() <= &14
                    && max_cubes_map.get("green").unwrap() <= &13
                    && max_cubes_map.get("red").unwrap() <= &12
                {
                    games_within_limit.push(id)
                }
            }
            _ => {}
        }
    }

    let result = games_within_limit.iter().fold(0, |acc, e| acc + e);
    println!("Day 2_1: {:?}", result);
}

pub fn execute_day_two_part_two() {
    let file_path = "./src/day_02/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut total_power: u32 = 0;
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let a: Vec<&str> = text.split(':').collect();
                let split_by_semicolon: Vec<&str> = a[1].split(';').map(|i| i.trim()).collect();

                // println!("{:?}, id: {:?}", a, id);
                // println!("split_by_semicolon: {:?}", split_by_semicolon);

                let mut max_cubes_map: HashMap<String, u32> =
                    HashMap::from([("red".into(), 0), ("green".into(), 0), ("blue".into(), 0)]);

                for round in split_by_semicolon.iter() {
                    let split_by_comma: Vec<&str> = round.split(',').map(|i| i.trim()).collect();
                    // println!("split_by_comma {:?}", split_by_comma);

                    for cube_tuple in split_by_comma.iter() {
                        let cube_tuple_split: Vec<&str> =
                            cube_tuple.split(' ').map(|i| i.trim()).collect();
                        let cube_amount: u32 =
                            u32::from_str_radix(cube_tuple_split[0], 10).unwrap();
                        let cube_color = cube_tuple_split[1];
                        // println!("cube_tuple_split {:?}", cube_amount);

                        let existing_max_cube_amount_shown =
                            max_cubes_map.get(cube_color.into()).unwrap();
                        max_cubes_map.insert(
                            cube_color.into(),
                            cmp::max(*existing_max_cube_amount_shown, cube_amount),
                        );
                    }
                }

                // println!("Game with id: {:?} cube_color_map {:?}", id, max_cubes_map);
                // println!("Cube power: {:?}", max_cubes_map.get("blue").unwrap() * max_cubes_map.get("green").unwrap() * max_cubes_map.get("red").unwrap());
                total_power += max_cubes_map.get("blue").unwrap()
                    * max_cubes_map.get("green").unwrap()
                    * max_cubes_map.get("red").unwrap();
            }
            _ => {}
        }
    }

    // println!("games_within_limit: {:?}", games_within_limit);
    // let result = games_within_limit.iter().fold(0, |acc, e| acc + e);
    println!("Day 2_2: {:?}", total_power);
}
