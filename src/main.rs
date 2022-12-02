// use std::env;
mod first;
use crate::first::challenges;

mod second;
use crate::second::challenges_2;

fn main() {
    let max_calories = challenges::first_challenge();
    println!("Max calories found! {}", max_calories);

    let top3_total_cals = challenges::second_challenge();
    println!("top 3 total calories! {}", top3_total_cals)
}
