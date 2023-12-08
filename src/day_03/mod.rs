use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Rectangle {
    start: Point,
    end: Point,
    value: Option<u32>,
}

impl Rectangle {
    pub fn intersects(&self, rectangle: &Rectangle) -> bool {
        if self.start.x > rectangle.end.x || rectangle.start.x > self.end.x {
            return false;
        }

        if self.end.y < rectangle.start.y || rectangle.end.y < self.start.y {
            return false;
        }

        true
    }
}

pub fn execute_day_three_part_one() {
    let file_path = "./src/day_03/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut number_rectangles: Vec<Rectangle> = vec![];
    let mut symbol_impact_area_rectangles: Vec<Rectangle> = vec![];

    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                let mut number_vector = vec![];
                for (index, char) in text.chars().enumerate() {
                    if char.is_digit(10) {
                        number_vector.push(char);
                    }

                    if number_vector.len() != 0 && (!char.is_digit(10) || index == text.len() - 1) {
                        let value: Option<u32> = Some(
                            u32::from_str_radix(&number_vector.iter().collect::<String>(), 10)
                                .unwrap(),
                        );
                        number_rectangles.push(Rectangle {
                            start: Point {
                                x: index as u32 - number_vector.len() as u32,
                                y: line_index as u32,
                            },
                            end: Point {
                                x: index as u32 - 1 as u32,
                                y: line_index as u32,
                            },
                            value,
                        });
                        number_vector = vec![];
                    }

                    if char != '.' && !char.is_digit(10) {
                        symbol_impact_area_rectangles.push(Rectangle {
                            start: Point {
                                x: cmp::max(0, index as u32 - 1),
                                y: cmp::max(0, line_index as u32 - 1),
                            },
                            end: Point {
                                x: index as u32 + 1,
                                y: line_index as u32 + 1,
                            },
                            value: None,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    let mut intersecting_number_rectangles: Vec<&Rectangle> = vec![];
    for number_rect in number_rectangles.iter() {
        for symbol_rect in symbol_impact_area_rectangles.iter() {
            if number_rect.intersects(symbol_rect) {
                intersecting_number_rectangles.push(number_rect);
                break;
            }
        }
    }

    let result = intersecting_number_rectangles
        .iter()
        .fold(0, |acc, e| acc + e.value.unwrap());
    println!("Day 3_1: {:?}", result);
}

pub fn execute_day_three_part_two() {
    let file_path = "./src/day_03/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut number_rectangles: Vec<Rectangle> = vec![];
    let mut symbol_impact_area_rectangles: Vec<Rectangle> = vec![];

    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                let mut number_vector = vec![];
                for (index, char) in text.chars().enumerate() {
                    if char.is_digit(10) {
                        number_vector.push(char);
                    }

                    if number_vector.len() != 0 && (!char.is_digit(10) || index == text.len() - 1) {
                        let value: Option<u32> = Some(
                            u32::from_str_radix(&number_vector.iter().collect::<String>(), 10)
                                .unwrap(),
                        );
                        number_rectangles.push(Rectangle {
                            start: Point {
                                x: index as u32 - number_vector.len() as u32,
                                y: line_index as u32,
                            },
                            end: Point {
                                x: index as u32 - 1 as u32,
                                y: line_index as u32,
                            },
                            value,
                        });
                        number_vector = vec![];
                    }

                    if char != '.' && !char.is_digit(10) {
                        let mut value = None;
                        if char == '*' {
                            value = Some(777777777)
                        }

                        symbol_impact_area_rectangles.push(Rectangle {
                            start: Point {
                                x: cmp::max(0, index as u32 - 1),
                                y: cmp::max(0, line_index as u32 - 1),
                            },
                            end: Point {
                                x: index as u32 + 1,
                                y: line_index as u32 + 1,
                            },
                            value,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    let mut total_gear_ratio = 0;
    for symbol_rect in symbol_impact_area_rectangles
        .iter()
        .filter(|i| i.value == Some(777777777))
    {
        let mut intersections: Vec<u32> = vec![];
        for number_rect in number_rectangles.iter() {
            if number_rect.intersects(symbol_rect) {
                intersections.push(number_rect.value.unwrap());
            }
        }

        if intersections.len() == 2 {
            let gear_ratio = intersections.iter().fold(1, |acc, e| acc * e);
            total_gear_ratio += gear_ratio;
        }
    }

    println!("Day 3_2: {:?}", total_gear_ratio);
}
