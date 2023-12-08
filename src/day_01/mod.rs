use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute_day_one_part_one() {
    let file_path = "./src/day_01/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut total_calibration_value = 0u32;
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let mut first_digit: Option<u8> = None;
                let mut last_digit: Option<u8> = None;
                for char in text.chars() {
                    if char.is_digit(10) {
                        if first_digit.is_none() {
                            first_digit = Some(u8::from_str_radix(&char.to_string(), 10).unwrap())
                        }

                        last_digit = Some(u8::from_str_radix(&char.to_string(), 10).unwrap());
                    }
                }

                let mut line_calibration_value_string = first_digit.unwrap().to_string();
                line_calibration_value_string.push_str(&last_digit.unwrap().to_string());
                total_calibration_value +=
                    u32::from_str_radix(&line_calibration_value_string, 10).unwrap();
            }
            _ => {}
        }
    }

    println!("Day 1_1: {:?}", total_calibration_value);
}

pub fn execute_day_one_part_two() {
    let file_path = "./src/day_01/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));

    let mut total_calibration_value = 0u32;
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let mut digits_map: HashMap<String, String> = HashMap::new();
                digits_map.insert("one".into(), "o1e".into());
                digits_map.insert("two".into(), "t2o".into());
                digits_map.insert("three".into(), "t3e".into());
                digits_map.insert("four".into(), "f4r".into());
                digits_map.insert("five".into(), "f5e".into());
                digits_map.insert("six".into(), "s6x".into());
                digits_map.insert("seven".into(), "s7n".into());
                digits_map.insert("eight".into(), "e8t".into());
                digits_map.insert("nine".into(), "n9e".into());

                let mut replaced_text = text.clone();
                for (key, value) in digits_map.iter() {
                    replaced_text = replaced_text.replacen(key, value, 99);
                }

                let mut first_digit: Option<u8> = None;
                let mut last_digit: Option<u8> = None;
                for char in replaced_text.chars() {
                    if char.is_digit(10) {
                        if first_digit.is_none() {
                            first_digit = Some(u8::from_str_radix(&char.to_string(), 10).unwrap())
                        }

                        last_digit = Some(u8::from_str_radix(&char.to_string(), 10).unwrap());
                    }
                }

                let mut line_calibration_value_string = first_digit.unwrap().to_string();
                line_calibration_value_string.push_str(&last_digit.unwrap().to_string());
                total_calibration_value +=
                    u32::from_str_radix(&line_calibration_value_string, 10).unwrap();
            }
            _ => {}
        }
    }

    println!("Day 1_2: {:?}", total_calibration_value);
}
