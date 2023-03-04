use std::{cmp::Ordering::*, process};

use chrono::prelude::{Local, NaiveDate};
use chrono::ParseResult;
use clap::Parser;

#[derive(Clone, Debug)]
struct DateFormat(String);

impl DateFormat {
    fn parse(&self, date: &UnparsedDate) -> ParseResult<NaiveDate> {
        NaiveDate::parse_from_str(&date.0, &self.0)
    }

    fn format(&self, date: &NaiveDate) -> String {
        date.format(&self.0).to_string()
    }
}

impl From<String> for DateFormat {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
struct UnparsedDate(String);

impl From<String> for UnparsedDate {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Calculate the number of days between two dates.
///
/// A negative number (when not using verbose mode) means that FIRST is before SECOND.
///
/// The dates are assumed to be in the same time zone.
#[derive(Parser, Debug)]
struct Args {
    /// strftime format string used to parse the date(s)
    #[arg(name = "format", short, long, default_value = "%Y-%m-%d")]
    fmt: DateFormat,

    /// Print natural language description.
    #[arg(short, long)]
    verbose: bool,

    /// Date to calculate to.
    first: UnparsedDate,

    /// Date to calculate from (defaults to today's date).
    second: Option<UnparsedDate>,
}

fn quit_on_parse_failure(date: UnparsedDate, fmt: DateFormat) -> ! {
    eprintln!("Could not parse date {:?} with format {:?}.", date.0, fmt.0);
    process::exit(1);
}

fn make_message(first: String, second: String, adjective: &'static str, difference: u64) -> String {
    let plural = if difference == 1 { "" } else { "s" };
    format!("{first} is {difference} day{plural} {adjective} {second}.")
}

fn main() {
    let Args {
        first,
        second,
        verbose,
        fmt,
    } = Args::parse();

    let Ok(first) = fmt.parse(&first) else { quit_on_parse_failure(first, fmt) };

    let second = if let Some(second) = second {
        match fmt.parse(&second) {
            Ok(date) => date,
            Err(_) => quit_on_parse_failure(second, fmt),
        }
    } else {
        Local::now().date_naive()
    };

    let day_delta = (first - second).num_days();
    if !verbose {
        println!("{}", day_delta);
    } else {
        let first = fmt.format(&first);
        let second = fmt.format(&second);
        let abs = day_delta.unsigned_abs();
        let message = match day_delta.cmp(&0) {
            Less => make_message(first, second, "before", abs),
            Equal => format!("Both dates are the same ({first})."),
            Greater => make_message(first, second, "after", abs),
        };
        println!("{message}");
    }
}
