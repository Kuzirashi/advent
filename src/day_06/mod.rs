use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

pub fn execute_day_six_part_one() {
    let file_path = "./src/day_06/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut races: Vec<(u16, u16)> = vec![];

    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                if line_index == 0 {
                    // time
                    let mut split_by_whitespace: Vec<&str> = text.split_whitespace().collect();

                    split_by_whitespace.remove(0);
                    for (index, time ) in split_by_whitespace.iter().enumerate() {
                        println!("time {:?}", time);
                        races.push((time.parse().unwrap(), 0));
                    }
                } else {
                    // distance
                    let mut split_by_whitespace: Vec<&str> = text.split_whitespace().collect();

                    split_by_whitespace.remove(0);

                    for (index, distance ) in split_by_whitespace.iter().enumerate() {
                        println!("distance {:?}", distance);
                        races[index] = (races[index].0, distance.parse().unwrap());
                    }
                    }
            }
            _ => {}
        }
    }

    let mut number_of_ways_to_win = vec![];
    for (time, distance) in races.clone() {
        let mut ways_to_win = 0;
        for i in 0..time {
            let distance_current = (time - i) * i;
            // println!("distance_current {:?}", distance_current);

            if distance_current > distance {
                ways_to_win += 1;
            }
        }

        number_of_ways_to_win.push(ways_to_win)
    }

    println!("races {:?}", races);
    println!("number_of_ways_to_win {:?}", number_of_ways_to_win);

    println!("Day 6_1: {:?}", number_of_ways_to_win.iter().fold(1, |acc, i| acc * i));
}

pub fn execute_day_six_part_two() {
    let file_path = "./src/day_06/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                if line_index == 0 {
                    let mut split_by_whitespace: Vec<&str> = text.split_whitespace().collect();
                    split_by_whitespace.remove(0);

                    let mut time_string = "".to_string();
                    for (index, time_part) in split_by_whitespace.iter().enumerate() {
                        // println!("time {:?}", time);
                        // races.push((time.parse().unwrap(), 0));
                        time_string.push_str(time_part);
                    }

                    time = time_string.parse().unwrap();
                } else {
                    // distance
                    let mut split_by_whitespace: Vec<&str> = text.split_whitespace().collect();
                    split_by_whitespace.remove(0);

                    let mut distance_string = "".to_string();
                    for (index, distance_part ) in split_by_whitespace.iter().enumerate() {
                        distance_string.push_str(distance_part);
                    }

                    distance = distance_string.parse().unwrap();
                }
            }
            _ => {}
        }
    }

    let mut number_of_ways_to_win = vec![];

    let mut ways_to_win = 0;
    for i in 0..time {
        let distance_current = (time - i) * i;
        // println!("distance_current {:?}", distance_current);

        if distance_current > distance {
            ways_to_win += 1;
        }
    }

    number_of_ways_to_win.push(ways_to_win);
    

    println!("time {:?}", time);
    println!("distance {:?}", distance);
    println!("number_of_ways_to_win {:?}", number_of_ways_to_win);

    println!("Day 6_2: {:?}", number_of_ways_to_win.iter().fold(1, |acc, i| acc * i));
}

