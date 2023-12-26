use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Strength {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

pub fn execute_day_07_part_one() {
    let file_path = "./src/day_07/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut hands: Vec<(Vec<char>, u16, Strength)> = vec![];
    let mut hands_types: Vec<Strength> = vec![];
    let mut cards_ordered = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    cards_ordered.reverse();

    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                let mut split_by_whitespace: Vec<&str> = text.split_whitespace().collect();
                // println!("{:?}", split_by_whitespace);

                let bet: u16 = split_by_whitespace[1].parse().unwrap();

                let mut occurences: HashMap<char, u8> = HashMap::new();
                for card in split_by_whitespace[0].chars() {
                    match occurences.get(&card) {
                        Some(occurence) => {
                            occurences.insert(card, occurence + 1);
                        }
                        _ => {
                            occurences.insert(card, 1);
                        }
                    }
                }

                let mut hand_type = Strength::HighCard;

                let contains_five_of_a_kind = occurences.iter().find(|i| *i.1 == 5).is_some();
                if contains_five_of_a_kind {
                    hand_type = Strength::FiveOfAKind;
                } else {
                    let contains_four_of_a_kind = occurences.iter().find(|i| *i.1 == 4).is_some();
                    if contains_four_of_a_kind {
                        hand_type = Strength::FourOfAKind;
                    } else {
                        let contains_three_of_a_kind =
                            occurences.iter().find(|i| *i.1 == 3).is_some();
                        let contains_two_of_a_kind =
                            occurences.iter().find(|i| *i.1 == 2).is_some();

                        if contains_three_of_a_kind {
                            if contains_two_of_a_kind {
                                hand_type = Strength::FullHouse;
                            } else {
                                hand_type = Strength::ThreeOfAKind;
                            }
                        } else {
                            let pairs_count = occurences.iter().filter(|i| *i.1 == 2).count();
                            if pairs_count == 2 {
                                hand_type = Strength::TwoPair;
                            } else if pairs_count == 1 {
                                hand_type = Strength::OnePair;
                            }
                        }
                    }
                }

                hands.push((split_by_whitespace[0].chars().collect(), bet, hand_type));
                hands_types.push(hand_type);

                // println!("occurences {:?}", occurences);
            }
            _ => {}
        }
    }

    hands.sort_by(|a, b| {
        if a.2 as i32 == b.2 as i32 {
            for (card_index, card_a) in a.0.clone().iter().enumerate() {
                let card_in_b = b.0.clone()[card_index];
                let card_in_b_strength =
                    cards_ordered.iter().position(|&i| i == card_in_b).unwrap();
                let card_in_a_strength = cards_ordered.iter().position(|&i| i == *card_a).unwrap();

                if card_in_a_strength > card_in_b_strength {
                    return Ordering::Greater;
                } else if card_in_a_strength < card_in_b_strength {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        } else if a.2 as i32 > b.2 as i32 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    // println!("hands_types {:?}", hands_types);
    // println!("hands_sorted {:?}", hands);

    let mut total_winnings = 0;

    for (index, hand) in hands.iter().enumerate() {
        total_winnings += (index as i32 + 1) * hand.1 as i32;
    }

    println!("Day 7_1: {:?}", total_winnings);
}

pub fn execute_day_07_part_2() {
    let file_path = "./src/day_07/input.txt";
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));

    let mut hands: Vec<(Vec<char>, u16, Strength)> = vec![];
    let mut hands_types: Vec<Strength> = vec![];
    let mut cards_ordered = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    cards_ordered.reverse();

    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                let mut split_by_whitespace: Vec<&str> = text.split_whitespace().collect();
                // println!("{:?}", split_by_whitespace);

                let bet: u16 = split_by_whitespace[1].parse().unwrap();

                let mut jokers_count = 0;

                let mut occurences: HashMap<char, u8> = HashMap::new();
                for card in split_by_whitespace[0].chars() {
                    if card == 'J' {
                        jokers_count += 1;
                    }

                    match occurences.get(&card) {
                        Some(occurence) => {
                            occurences.insert(card, occurence + 1);
                        }
                        _ => {
                            occurences.insert(card, 1);
                        }
                    }
                }

                let mut hand_type = Strength::HighCard;

                let contains_five_of_a_kind = occurences.iter().find(|i| *i.1 == 5).is_some();
                if contains_five_of_a_kind {
                    hand_type = Strength::FiveOfAKind;
                } else {
                    let contains_four_of_a_kind = occurences.iter().find(|i| *i.1 == 4).is_some();

                    if contains_four_of_a_kind {
                        hand_type = Strength::FourOfAKind;
                    } else {
                        let contains_three_of_a_kind =
                            occurences.iter().find(|i| *i.1 == 3).is_some();

                        if contains_three_of_a_kind {
                            let contains_two_of_a_kind =
                                occurences.iter().find(|i| *i.1 == 2).is_some();

                            if contains_two_of_a_kind {
                                hand_type = Strength::FullHouse;
                            } else {
                                hand_type = Strength::ThreeOfAKind;
                            }
                        } else {
                            let pairs_count = occurences.iter().filter(|i| *i.1 == 2).count();
                            if pairs_count == 2 {
                                hand_type = Strength::TwoPair;
                            } else if pairs_count == 1 {
                                hand_type = Strength::OnePair;
                            }
                        }
                    }
                }

                if jokers_count == 4 {
                    hand_type = Strength::FiveOfAKind;
                } else if jokers_count == 3 {
                    if hand_type == Strength::FullHouse {
                        hand_type = Strength::FiveOfAKind;
                    } else if hand_type == Strength::ThreeOfAKind {
                        hand_type = Strength::FourOfAKind;
                    }
                } else if jokers_count == 2 {
                    if hand_type == Strength::FullHouse {
                        hand_type = Strength::FiveOfAKind;
                    } else if hand_type == Strength::TwoPair {
                        hand_type = Strength::FourOfAKind;
                    } else if hand_type == Strength::OnePair {
                        hand_type = Strength::ThreeOfAKind;
                    }
                } else if jokers_count == 1 {
                    if hand_type == Strength::FourOfAKind {
                        hand_type = Strength::FiveOfAKind;
                    } else if hand_type == Strength::ThreeOfAKind {
                        hand_type = Strength::FourOfAKind;
                    } else if hand_type == Strength::TwoPair {
                        hand_type = Strength::FullHouse;
                    } else if hand_type == Strength::OnePair {
                        hand_type = Strength::ThreeOfAKind;
                    } else if hand_type == Strength::HighCard {
                        hand_type = Strength::OnePair;
                    }
                }

                hands.push((split_by_whitespace[0].chars().collect(), bet, hand_type));
                hands_types.push(hand_type);

                // println!("occurences {:?}", occurences);
            }
            _ => {}
        }
    }

    hands.sort_by(|a, b| {
        if a.2 as i32 == b.2 as i32 {
            for (card_index, card_a) in a.0.clone().iter().enumerate() {
                let card_in_b = b.0.clone()[card_index];
                let card_in_b_strength =
                    cards_ordered.iter().position(|&i| i == card_in_b).unwrap();
                let card_in_a_strength = cards_ordered.iter().position(|&i| i == *card_a).unwrap();

                if card_in_a_strength > card_in_b_strength {
                    return Ordering::Greater;
                } else if card_in_a_strength < card_in_b_strength {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        } else if a.2 as i32 > b.2 as i32 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    // println!("hands_types {:?}", hands_types);
    // println!("hands_sorted {:?}", hands);

    let mut total_winnings = 0;

    for (index, hand) in hands.iter().enumerate() {
        total_winnings += (index as i32 + 1) * hand.1 as i32;
    }

    println!("Day 7_2: {:?}", total_winnings);
}
