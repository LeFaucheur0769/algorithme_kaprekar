use clap::{CommandFactory, Parser, error::Result};

/// A simple program to calculate the kaprekar algorythm
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///The number to process
    #[arg(short, long)]
    number: u64,

    ///The number of attemps
    #[arg(short, long, default_value = "10")]
    attemps_nbr: u64,
}

fn main() {
    let mut number = Args::parse().number;
    let nbr_attemps = Args::parse().attemps_nbr;

    for _ in 1..nbr_attemps {
        number = kaprepar_algo(number);
    }

    // vec_number = number
    // println!("{}", format!("{}:{}", number, nbr_attemps));
}

fn kaprepar_algo(number: u64) -> u64 {
    let split_vec = split_number(number);
    // println!("{:?}", split_vec.iter().collect::<Vec<_>>());

    let split_vec_descending = descending_number_split_number(split_vec.clone());
    let split_vec_ascending = ascending_number_split_number(split_vec.clone());
    // println!("{:?}", split_vec_descending.iter().collect::<Vec<_>>());

    let ascending_number = vec_u64_to_u64(split_vec_ascending);
    let descending_number = vec_u64_to_u64(split_vec_descending);
    // let final_number: u64 = 0;
    let final_number = descending_number - ascending_number;
    //println!("ascending_number : {}", ascending_number);
    //println!("descending_number : {}", descending_number);
    println!("final_number : {}", final_number);
    final_number
}

fn split_number(mut num: u64) -> Vec<u64> {
    let mut digit = Vec::new();
    while num > 0 {
        // Push the modulo 10 of num, then devide it by 10
        digit.push(num % 10);
        num /= 10;
    }
    //reverse the vector to have it in the right order
    digit.reverse();
    digit
}

fn ascending_number_split_number(mut split_vec: Vec<u64>) -> Vec<u64> {
    split_vec.sort();
    split_vec
}

fn descending_number_split_number(mut split_vec: Vec<u64>) -> Vec<u64> {
    split_vec.sort();
    split_vec.reverse();
    split_vec
}

fn vec_u64_to_u64(vec_u64: Vec<u64>) -> u64 {
    let mut u_to_return: u64 = 0;
    for i in vec_u64 {
        u_to_return = u_to_return * 10 + i;
    }
    u_to_return
}
