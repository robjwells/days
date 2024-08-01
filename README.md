```
$ days --help
Calculate the number of days between two dates.

A negative number (when not using verbose mode) means that DATE_TO is before DATE_FROM.

The dates are assumed to be in the same time zone.

Usage: days [OPTIONS] <DATE_TO> [DATE_FROM]

Arguments:
  <DATE_TO>
          Date to calculate to

  [DATE_FROM]
          Date to calculate from (defaults to today's date)

Options:
  -f, --format <format>
          strftime format string used to parse the date(s)

          [default: %Y-%m-%d]

  -v, --verbose
          Print natural language description

  -h, --help
          Print help (see a summary with '-h')
```

```
$ days 2023-01-01
-578
```

```
$ days -v -f "%B %-d %Y" "March 1 2023" "December 12 1995"
March 1 2023 is 9941 days after December 12 1995
```
