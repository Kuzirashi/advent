use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Debug, Clone)]
struct Card {
    duplicates: u32,
    won_card_ids: Vec<u32>,
}

pub fn execute_day_four_part_one() {
    let file_path = "./src/day_04/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut total_points = 0;
    for (_line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                let a: Vec<&str> = text.split(':').collect();

                let number_groups: Vec<&str> = a[1].split('|').map(|i| i.trim()).collect();
                let winning_numbers: Vec<u32> = number_groups[0]
                    .split(' ')
                    .filter(|i| i.len() != 0)
                    .map(|i| u32::from_str_radix(i, 10).unwrap())
                    .collect();
                let picked_numbers: Vec<u32> = number_groups[1]
                    .split(' ')
                    .filter(|i| i.len() != 0)
                    .map(|i| u32::from_str_radix(i.trim(), 10).unwrap())
                    .collect();

                let winning_numbers_count = winning_numbers
                    .iter()
                    .filter(|winning_number| picked_numbers.contains(&winning_number))
                    .count() as u32;

                let points = if winning_numbers_count > 0 {
                    2u32.pow(winning_numbers_count - 1)
                } else {
                    0
                };

                total_points += points;
            }
            _ => {}
        }
    }

    println!("Day 4_1: {:?}", total_points);
}

pub fn execute_day_four_part_two() {
    let file_path = "./src/day_04/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut cards: Vec<Card> = vec![];

    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                let a: Vec<&str> = text.split(':').collect();
                // println!("{:?}", a[0].split(' '));
                // let id: u32 =
                //     u32::from_str_radix(a[0].split(' ').collect::<Vec<&str>>()[1], 10).unwrap();
                let number_groups: Vec<&str> = a[1].split('|').map(|i| i.trim()).collect();
                let winning_numbers: Vec<u32> = number_groups[0]
                    .split(' ')
                    .filter(|i| i.len() != 0)
                    .map(|i| u32::from_str_radix(i, 10).unwrap())
                    .collect();
                let picked_numbers: Vec<u32> = number_groups[1]
                    .split(' ')
                    .filter(|i| i.len() != 0)
                    .map(|i| u32::from_str_radix(i.trim(), 10).unwrap())
                    .collect();

                let winning_numbers_count = winning_numbers
                    .iter()
                    .filter(|winning_number| picked_numbers.contains(&winning_number))
                    .count() as u32;

                let mut won_cards = vec![];
                let mut x = 0;
                while x < winning_numbers_count {
                    x += 1;
                    won_cards.push(line_index as u32 + x + 1);
                }

                cards.push(Card {
                    won_card_ids: won_cards,
                    duplicates: 0,
                })
            }
            _ => {}
        }
    }

    let mut cards_to_process = cards.clone();

    let mut total_cards = 0;
    let mut index = 0;
    while cards_to_process.len() > 0 {
        let card = cards_to_process[index].clone();
        let cards_won = card.won_card_ids.len();

        let total_instances = 1 + card.duplicates;

        for i in 0..cards_won {
            let inner_index = index + i + 1;
            cards_to_process[inner_index].duplicates += total_instances;
        }

        total_cards += total_instances;
        index += 1;

        if (index == cards_to_process.len()) {
            break;
        }
    }

    println!("Day 4_2: {:?}", total_cards);
}
