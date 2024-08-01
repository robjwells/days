use anyhow::Result;
use clap::Parser;
use jiff::civil::Date;
use jiff::Zoned;

#[derive(Clone, Debug)]
struct DateFormat(String);

impl DateFormat {
    fn parse(&self, date: &UnparsedDate) -> Result<Date> {
        Date::strptime(&self.0, &date.0).map_err(anyhow::Error::from)
    }

    fn format(&self, date: &Date) -> String {
        date.strftime(&self.0).to_string()
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
/// A negative number (when not using verbose mode) means that DATE_TO is before DATE_FROM.
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
    date_to: UnparsedDate,

    /// Date to calculate from (defaults to today's date).
    date_from: Option<UnparsedDate>,
}

fn main() -> Result<()> {
    let Args {
        date_to,
        date_from,
        verbose,
        fmt,
    } = Args::parse();

    let date_to = fmt.parse(&date_to)?;
    let date_from = date_from
        .map(|d| fmt.parse(&d))
        .unwrap_or(Ok(Zoned::now().into()))?;
    let duration = date_from.until(date_to)?;

    if !verbose {
        println!("{}", duration.get_days());
    } else {
        let dt = fmt.format(&date_to);
        let df = fmt.format(&date_from);
        let days = duration.get_days();
        let abs = days.unsigned_abs();
        let message: String = match days {
            ..=-2 => format!("{dt} is {abs} days before {df}"),
            -1 => format!("{dt} is 1 day before {df}"),
            0 => format!("Both dates are the same ({df})"),
            1 => format!("{dt} is 1 day after {df}"),
            2.. => format!("{dt} is {abs} days after {df}"),
        };
        println!("{message}");
    }
    Ok(())
}
