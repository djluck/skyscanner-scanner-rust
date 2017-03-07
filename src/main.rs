extern crate chrono;
use chrono::prelude::*;

use std::ops::Range;
use std::iter::*;

fn main(){
    let search = FlightSearch {
        departure: "SYD",
        destination: "LON",
        fly_between: FlightRange { 
            earliest_departure: NaiveDate::from_ymd(2017, 6 , 20),
            latest_return: NaiveDate::from_ymd(2017, 7, 30)
        },
        trip_time_days: Range{
            start: 16,
            end: 30
        },
        max_holiday_days: None

    };

    for date in search.fly_between.into_iter() {
        println!("{}", date)
    }
    
}

struct FlightRange {
    earliest_departure : NaiveDate,
    latest_return: NaiveDate
}

impl IntoIterator for FlightRange {
    type Item = NaiveDate;
    type IntoIter = std::vec::IntoIter<NaiveDate>;

    fn into_iter(self) -> Self::IntoIter {
        let oneDay = chrono::Duration::days(1);
        let range = 0..self.latest_return.signed_duration_since(self.earliest_departure).num_days();
        range
            .map(|day| self.earliest_departure + chrono::Duration::days(day))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

struct FlightSearch<'a>  {
    departure: &'a str, 
    destination: &'a str, 
    fly_between: FlightRange,
    trip_time_days: Range<i32>,
    max_holiday_days: Option<i32>
}

/*
How many flight searches will we do?
(num distinct days - min trip time) * (max trip time - min trip time)
*/
