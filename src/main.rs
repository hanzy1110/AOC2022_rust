// use std::env;
mod first;
// use crate::first::challenges;

mod second;
use crate::second::challenges_2;

mod second2;
use crate::second2::challenges_2v2;

mod third;
use crate::third::third_day;

fn main() {
    // let max_calories = challenges::first_challenge();
    // println!("Max calories found! {}", max_calories);

    // let top3_total_cals = challenges::second_challenge();
    // println!("top 3 total calories! {}", top3_total_cals);

    let (total_score, scores) = challenges_2::first_challenge();
    println!("{:?}", scores.len());
    println!("Total Score is: {}", total_score);

    let total_score_2 = challenges_2::second_challenge();
    println!("Total Score new is: {}", total_score_2);

    challenges_2v2::second_challenge();

    let total_priorities = third_day::first_challenge();
    println!("total priorities were: {}", total_priorities)
}
