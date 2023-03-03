use std::{
    cmp::Ordering::{Equal, Greater, Less},
    process,
};

use chrono::prelude::{Local, NaiveDate};
use clap::Parser;

/// Calculate the number of days between two dates.
///
/// The arguments should be given in ISO format, eg 2023-03-03.
///
/// A negative number result means that FIRST_ISO is before SECOND_ISO
/// (when not using --verbose mode).
///
/// The dates are assumed to be in the same time zone.
#[derive(Parser, Debug)]
struct Args {
    /// Print natural language description.
    #[arg(short, long)]
    verbose: bool,
    /// Date to calculate from.
    #[arg(value_name = "FIRST_ISO")]
    first: String,
    /// Date to calculate to (defaults to today's date).
    #[arg(value_name = "SECOND_ISO")]
    second: Option<String>,
}

fn parse_iso_date(date: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Failed to parse date: {:?}", date);
            process::exit(1);
        }
    }
}

fn main() {
    let Args {
        first,
        second,
        verbose,
    } = Args::parse();
    let first = parse_iso_date(&first);
    let second = second.map_or_else(|| Local::now().date_naive(), |s| parse_iso_date(&s));

    let days_difference = (first - second).num_days();
    if verbose {
        match days_difference.cmp(&0) {
            Less => println!("{first} is {} days before {second}.", days_difference.abs()),
            Equal => println!("Both dates are the same ({first})."),
            Greater => println!("{first} is {} days after {second}.", days_difference.abs()),
        }
    } else {
        println!("{:?}", days_difference);
    }
}
