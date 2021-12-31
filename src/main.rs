#![feature(iter_intersperse)]

mod structs;
mod file;

use rayon::prelude::*;
use std::error::Error;
use std::time::Instant;

use structs::{Overlap,Clue};
use file::read_lines;

#[allow(dead_code)]
enum ScoreType {
    MinimiseVariance,
    MinimiseLargestBucketSize
}

static SCORE_METHOD: ScoreType = ScoreType::MinimiseLargestBucketSize;

fn calculate_word_overlap(actual: &[char], guess: &[char]) -> Overlap {
    let mut actual_clone = actual.to_vec();
    let mut right_place = 0;
    let mut wrong_place = 0;

    for j in 0..guess.len() {
        if actual[j] == guess[j] {
            right_place += 1;
            actual_clone[j] = '\0';
            continue;
        }

        for i in 0..actual_clone.len() {
            if guess[j] == actual_clone[i] && actual_clone[i] != guess[i] {
                wrong_place += 1;
                actual_clone[i] = '\0';
                break;
            }
        }
    }

    Overlap::new(wrong_place, right_place)
}

fn best_clue(words_list: &Vec<Vec<char>>) -> Clue {
    words_list
        .par_iter()
        .map(|word| {
            let mut bucket_counts = [0; 26];

            for other_word in words_list {
                let overlap = calculate_word_overlap(&word, &other_word);

                bucket_counts[(overlap.right_place * 5 + overlap.wrong_place) as usize] += 1;
            }

            let value = match &SCORE_METHOD {
                ScoreType::MinimiseLargestBucketSize => 
                    *bucket_counts.iter().max().unwrap(),
                ScoreType::MinimiseVariance => {
                    let average = bucket_counts.iter().sum::<usize>() / bucket_counts.len();
                    bucket_counts
                        .iter()
                        .map(|v| (v - average).pow(2))
                        .sum::<usize>()
                }
            };

            Clue {
                value: value,
                word: word.to_vec(),
            }
        })
        .max_by(|a, b| b.value.cmp(&a.value))
        .unwrap()
}

fn read_u8_from_stdin() -> u8 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<u8>().unwrap()
}

fn solve(initial_words_list: &Vec<Vec<char>>) {
    let mut current_words_list = initial_words_list.clone();

    while current_words_list.len() > 1 {
        let next_clue = best_clue(&current_words_list);
        println!("Best clue is: {:?}", next_clue.word);

        println!("Wrong place? :");
        let wrong_place = read_u8_from_stdin();

        println!("Right place? :");
        let right_place = read_u8_from_stdin();

        let overlap = Overlap::new(wrong_place, right_place);

        current_words_list.retain(|v| calculate_word_overlap(&*v, &next_clue.word) == overlap);
    }

    println!("Answer is: {:?}", current_words_list);
}

fn solve_auto(initial_words_list: &Vec<Vec<char>>, word: &[char]) -> Vec<Clue> {
    let mut current_words_list = initial_words_list.clone();
    let mut sequence = Vec::new();

    while current_words_list.len() > 1 {
        let next_clue = best_clue(&current_words_list);
        let overlap = calculate_word_overlap(&next_clue.word, word);

        sequence.push(next_clue.clone());

        current_words_list.retain(|v| calculate_word_overlap(&*v, &next_clue.word) == overlap);
    }

    sequence
}

fn c(a: &str) -> Vec<char> {
    a.chars().collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("./data/words5.txt")?;
    let now = Instant::now();

    // dbg!(calculate_word_overlap(&c("plant"), &c("areas")));
    // dbg!(calculate_word_overlap(&c("plant"), &c("donee")));
    // dbg!(calculate_word_overlap(&c("plant"), &c("sloth")));
    // dbg!(calculate_word_overlap(&c("plant"), &c("skint")));

    // solve(&lines);
    // solve_auto(&lines, &c("plant"));

    for line in lines[100..101].iter() {
        let result = solve_auto(&lines, line);

        println!(
            "{}",
            result
                .iter()
                .map(|c| format!("{} ({})", c.word.iter().collect::<String>(), c.value))
                .intersperse(", ".to_string())
                .collect::<String>()
        );
    }

    println!("Elapsed after time: {:.2?}", now.elapsed());

    Ok(())
}
