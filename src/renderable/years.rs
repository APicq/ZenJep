use crate::flightlistgenerator::{flighttime::FlightTime, FlightList};
use anyhow::{Context, Result};
use askama::Template;

#[derive(Debug)]
struct MonthHtml {
    total_month: FlightTime, // todo change
    total_this_year: FlightTime,
    total_overall: FlightTime,
}

#[derive(Debug)]
pub struct YearHtml {
    year: i32,
    months: Vec<MonthHtml>,
}

#[derive(Template, Debug)]
#[template(path = "years.html")]
pub struct YearsRangeHtml {
    years: Vec<YearHtml>,
}

impl YearsRangeHtml {
    pub fn from_flight_list(flight_list: &FlightList) -> Result<Self> {
        // Find the maximum and minimum
        // year in all flights
        let all_flight_year: Vec<i32> = flight_list.flights.iter().map(|f| f.date.year()).collect();
        let year_min = *all_flight_year.iter().min().unwrap();
        let year_max = *all_flight_year.iter().max().unwrap();

        // Create an empty yearsrangehtml :
        let mut years = Vec::new();
        for year in year_min..=year_max {
            let mut months = Vec::new();
            for _month_number in 0..12 {
                months.push(MonthHtml {
                    total_month: FlightTime::ZERO,
                    total_this_year: FlightTime::ZERO,
                    total_overall: FlightTime::ZERO,
                });
            } // for month
            years.push(YearHtml { year, months });
        } // for year
        let mut years_range_html = YearsRangeHtml { years };

        // Fill
        for flight in flight_list.flights.iter() {
            let flight_year = flight.date.year();
            let flight_month = flight.date.month();
            let total_flight_time = flight.total_flight_time;
            let index_year = years_range_html.index_from_year(flight_year)?;
            let month = &mut years_range_html.years[index_year].months[flight_month as usize];
            month.total_month += total_flight_time;
        }

        // Fill total years and total overall
        let mut accumulator_year;
        let mut accumulator_overall = FlightTime::ZERO;
        for year in years_range_html.years.iter_mut() {
            accumulator_year = FlightTime::ZERO;
            for month in year.months.iter_mut() {
                accumulator_overall += month.total_month;
                accumulator_year += month.total_month;
                month.total_this_year = accumulator_year;
                month.total_overall = accumulator_overall;
            }
        }
        Ok(years_range_html)
    }

    /// Find an index from a year;
    fn index_from_year(&self, target_year: i32) -> Result<usize> {
        let year_min = self
            .years
            .get(0)
            .context("Error: YearsRangeHtml is empty. This should not happen")?
            .year;
        Ok((target_year - year_min) as usize)
    }
}
