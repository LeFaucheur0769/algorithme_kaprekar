use core::time;
use std::{
    fs::{OpenOptions, exists},
    ops::Not,
    path::PathBuf,
    process::{self, exit},
    thread::sleep,
};

use clap::{CommandFactory, Parser, builder::Str, error::Result};

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

    /// The file to which store the results
    #[arg(short, long, default_value = "")]
    file_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut number = args.number;
    let nbr_attemps = args.attemps_nbr;
    let file_path = args.file_path;
    let mut stringified: String;

    if file_path.is_empty().not() && check_if_file_exist(file_path).unwrap() {
        process::exit(0);
    }

    for _ in 0..nbr_attemps {
        number = kaprepar_algo(number);
        stringified = u64_to_spaced_string(number);
        println!("final_number : {}", stringified);
    }

    Ok(())
}

fn kaprepar_algo(number: u64) -> u64 {
    let split_vec = split_number(number);
    let split_vec_descending = descending_number_split_number(split_vec.clone());
    let split_vec_ascending = ascending_number_split_number(split_vec.clone());
    let ascending_number = vec_u64_to_u64(split_vec_ascending);
    let descending_number = vec_u64_to_u64(split_vec_descending);
    descending_number - ascending_number

    //println!("final_number : {}", final_number);
}

/// Function used to split u64 into a vec of u64
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
/// Function used to sort a vector of u64 in ascending order
fn ascending_number_split_number(mut split_vec: Vec<u64>) -> Vec<u64> {
    split_vec.sort();
    split_vec
}

/// Function used to sort a vector of u64 in descending order
fn descending_number_split_number(mut split_vec: Vec<u64>) -> Vec<u64> {
    split_vec.sort();
    split_vec.reverse();
    split_vec
}

/// Function used to converte a vec of u64 into u64
fn vec_u64_to_u64(vec_u64: Vec<u64>) -> u64 {
    let mut u_to_return: u64 = 0;
    for i in vec_u64 {
        u_to_return = u_to_return * 10 + i;
    }
    u_to_return
}

/// Function used to test if the file exists
fn check_if_file_exist(file_path: String) -> Result<bool, Box<dyn std::error::Error>> {
    let path = PathBuf::from(file_path);
    if exists(path)? {
        println!("File already exists, aborting ...");
        sleep(time::Duration::from_millis(500));
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Write the selected vec to a specified file
fn write_vec_to_file(vec: Vec<u64>, file_path: String) {
    let path = PathBuf::from(file_path);
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path);
}

/// Function used to turn u64 into String and add whitespace between 3 digits for easier reading
fn u64_to_spaced_string(num: u64) -> String {
    let chars_vec: Vec<char> = num.to_string().chars().collect();
    let mut stringified_u64 = "".to_string();
    for i in 1..chars_vec.len() {
        stringified_u64.push(chars_vec[i - 1]);
        if i % 3 == 0 {
            stringified_u64.push(' ');
        }
    }
    stringified_u64
}
