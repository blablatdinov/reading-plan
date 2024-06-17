// The MIT License (MIT).
//
// Copyright (c) 2024 Almaz Ilaletdinov <a.ilaletdinov@yandex.ru>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
// OR OTHER DEALINGS IN THE SOFTWARE.

use std::{io, process};
use chrono::*;
use std::cmp;

fn read_numbers() -> (u32, u32) {
    (
        read_user_input("Enter start page").parse().expect("Error parsing start page"),
        read_user_input("Enter end page").parse().expect("Error parsing end page")
    )
}

fn read_user_input(guess: &str) -> String {
    println!("{}: ", guess); // add `?` if you care about errors here
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.trim().to_string();
        },
        Err(error) => {
            println!("Reading error. {}", error);
            process::exit(1)
        }
    }
}

fn read_dates() -> (NaiveDate, NaiveDate) {
    let start_date_input = read_user_input("Enter start date");
    let end_date_input = read_user_input("Enter end date");
    let start_date;
    let end_date;
    match NaiveDate::parse_from_str(&start_date_input[..], "%Y-%m-%d") {
        Ok(date) => start_date = date,
        Err(error) => {
            println!("Error on parsing start date. {}", error);
            process::exit(1)
        }
    }
    match NaiveDate::parse_from_str(&end_date_input[..], "%Y-%m-%d") {
        Ok(date) => end_date = date,
        Err(error) => {
            println!("Error on parsing end date. {}", error);
            process::exit(1)
        }
    }
    return (start_date, end_date)
}

fn render_plan(start_page: u32, end_page: u32, start_date: NaiveDate, finish_date: NaiveDate) -> String {
    let days = (finish_date - start_date).num_days() as u32 + 1;
    let page_diff = end_page - start_page;
    let page_per_day = (page_diff as f64 / days as f64).ceil() as u32;
    let mut strings: Vec<String> = vec![];
    for x in 0..days {
        let last_page_for_day = cmp::min(end_page, start_page + (page_per_day * x) + page_per_day - 1);
        strings.push(format!(
            "Date: {}. Pages: {}-{}",
            start_date + Duration::days(x.into()),
            start_page + (page_per_day * x),
            last_page_for_day,
        ))
    }
    strings.join("\n")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_plan() {
        let got = render_plan(
            1,
            12,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 12).unwrap(),
        );
        assert_eq!(got, vec![
            "Date: 2020-01-01. Pages: 1-1",
            "Date: 2020-01-02. Pages: 2-2",
            "Date: 2020-01-03. Pages: 3-3",
            "Date: 2020-01-04. Pages: 4-4",
            "Date: 2020-01-05. Pages: 5-5",
            "Date: 2020-01-06. Pages: 6-6",
            "Date: 2020-01-07. Pages: 7-7",
            "Date: 2020-01-08. Pages: 8-8",
            "Date: 2020-01-09. Pages: 9-9",
            "Date: 2020-01-10. Pages: 10-10",
            "Date: 2020-01-11. Pages: 11-11",
            "Date: 2020-01-12. Pages: 12-12",
        ].join("\n"));
    }

    #[test]
    fn test_render_plan_2() {
        let got = render_plan(
            691,
            767,
            NaiveDate::from_ymd_opt(2023, 3, 27).unwrap(),
            NaiveDate::from_ymd_opt(2023, 4, 2).unwrap(),
        );
        assert_eq!(got, vec![
            "Date: 2023-03-27. Pages: 691-701",
            "Date: 2023-03-28. Pages: 702-712",
            "Date: 2023-03-29. Pages: 713-723",
            "Date: 2023-03-30. Pages: 724-734",
            "Date: 2023-03-31. Pages: 735-745",
            "Date: 2023-04-01. Pages: 746-756",
            "Date: 2023-04-02. Pages: 757-767",
        ].join("\n"));
    }
}

fn main() {
    let (start_page, end_page) = read_numbers();
    let (start_date, finish_date) = read_dates();
    println!("{}", render_plan(start_page, end_page, start_date, finish_date))
}
