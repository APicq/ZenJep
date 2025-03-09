use anyhow::bail;
use anyhow::Result;
use time::Date;
use time::Duration;
use time::PrimitiveDateTime;
use time::Time;
///
/// Helper functions
/// The scanner in chronological order
/// FindYear: expects an integer
/// Finddayseparator: expects a -
/// etc.
#[derive(Debug, PartialEq)]
enum DateScanner {
    FindYear,
    FindYearSeparator,
    FindMonth,
    FindMonthSeparator,
    FindDay,
    FindDaySeparator,
    FindHour,
    FindHourSeparator,
    FindMinute,
    End,
}

/// Turn a string into a DateTime
pub fn extract_date(input: &str) -> Result<PrimitiveDateTime> {
    let mut scanner = DateScanner::FindYear;
    let mut year_number = 0;
    let mut year = 0;
    let mut month_number = 0;
    let mut month = 0;
    let mut day_number: u32 = 0;
    let mut day: u32 = 0;
    let mut hour_number = 0;
    let mut hour = 0;
    let mut minute_number = 0;
    let mut minute = 0;

    for (index, c) in input.char_indices() {
        // log::trace!("index= {} char={}", index, c);
        // log::trace!("year={}", year);
        // log::trace!("month={}", month);
        // log::trace!("day={}", day);
        // log::trace!("hour={}", hour);
        // log::trace!("minute={}", minute);
        match c {
            '0'..='9' => {
                if scanner == DateScanner::FindYear {
                    year += (c as u32 - '0' as u32) * 10_u32.pow(3 - year_number);
                    year_number += 1;
                    if year_number == 4 {
                        scanner = DateScanner::FindYearSeparator;
                    }
                } else if scanner == DateScanner::FindMonth {
                    month += (c as u32 - '0' as u32) * 10_u32.pow(1 - month_number);
                    month_number += 1;
                    if month_number == 2 {
                        scanner = DateScanner::FindMonthSeparator;
                    }
                } else if scanner == DateScanner::FindDay {
                    day += (c as u32 - '0' as u32) * 10_u32.pow(1 - day_number);
                    day_number += 1;
                    if day_number == 2 {
                        scanner = DateScanner::FindDaySeparator;
                    }
                } else if scanner == DateScanner::FindHour {
                    hour += (c as u32 - '0' as u32) * 10_u32.pow(1 - hour_number);
                    hour_number += 1;
                    if hour_number == 2 {
                        scanner = DateScanner::FindHourSeparator;
                    }
                } else if scanner == DateScanner::FindMinute {
                    minute += (c as u32 - '0' as u32) * 10_u32.pow(1 - minute_number);
                    minute_number += 1;
                    if minute_number == 2 {
                        scanner = DateScanner::End;
                    }
                } else {
                    bail!(
                        "Integer found at position {} with scanner={:?}",
                        index,
                        scanner
                    );
                }
            }
            '-' => {
                if scanner == DateScanner::FindYearSeparator {
                    scanner = DateScanner::FindMonth;
                } else if scanner == DateScanner::FindMonthSeparator {
                    scanner = DateScanner::FindDay;
                } else {
                    bail!("unexpected - at position {}", index);
                }
            }
            ':' => {
                if scanner == DateScanner::FindHourSeparator {
                    scanner = DateScanner::FindMinute;
                } else {
                    bail!("Unexpected : at position {}", index);
                }
            }
            ' ' => {
                if scanner == DateScanner::FindDaySeparator {
                    scanner = DateScanner::FindHour;
                } else {
                    bail!("unexpected space at position {}", index);
                }
            }
            c => {
                bail!("Unexpected character {} at position {}", c, index);
            }
        }
    }
    if scanner != DateScanner::End {
        bail!("scanner has not finished to parse the string. Extra characters suspected.");
    }
    let date =
        Date::from_calendar_date(year as i32, time::Month::try_from(month as u8)?, day as u8)?;
    let time = Time::from_hms(hour as u8, minute as u8, 0)?;
    Ok(PrimitiveDateTime::new(date, time))
}

/// todo change as test
#[test]
fn experiment_date() {
    let input = "2013-02-23 00:59";
    log::info!("input:{}", input);
    match extract_date(input) {
        Err(e) => log::error!("{}", e),
        Ok(d) => log::info!("{}", d),
    }
    let input = "2013-02-23 00:59";
    log::info!("input:{}", input);
    match extract_date(input) {
        Err(e) => log::error!("{}", e),
        Ok(d) => log::info!("{}", d),
    }
    let input = "2013-02-31 00:59";
    log::info!("input:{}", input);
    match extract_date(input) {
        Err(e) => log::error!("{}", e),
        Ok(d) => log::info!("{}", d),
    }
    let input = "201314-23 00:59";
    log::info!("input:{}", input);
    match extract_date(input) {
        Err(e) => log::error!("{}", e),
        Ok(d) => log::info!("{}", d),
    }
    let input = "2013-02-23 26:59";
    log::info!("input:{}", input);
    match extract_date(input) {
        Err(e) => log::error!("{}", e),
        Ok(d) => log::info!("{}", d),
    }
    let input = "2013-02-23 11:16 ";
    log::info!("input:{}", input);
    match extract_date(input) {
        Err(e) => log::error!("{}", e),
        Ok(d) => log::info!("{}", d),
    }
    let input = " 2013-02-23 11:16 ";
    log::info!("input:{}", input);
    match extract_date(input) {
        Err(e) => log::error!("{}", e),
        Ok(d) => log::info!("{}", d),
    }
}

#[derive(Debug, PartialEq)]
enum DurationScanner {
    FindHour,
    FindHourSeparator,
    FindMinute,
    End,
}

pub fn extract_duration(input: &str) -> Result<Duration> {
    let mut scanner = DurationScanner::FindHour;
    let mut hour_number = 0;
    let mut hour = 0;
    let mut minute_number = 0;
    let mut minute = 0;
    for (index, c) in input.char_indices() {
        log::trace!("index= {} char={}", index, c);
        log::trace!("hour={}", hour);
        log::trace!("minute={}", minute);
        match c {
            '0'..='9' => {
                if scanner == DurationScanner::FindHour {
                    hour += (c as u32 - '0' as u32) * 10_u32.pow(1 - hour_number);
                    hour_number += 1;
                    if hour_number == 2 {
                        scanner = DurationScanner::FindHourSeparator;
                    }
                } else if scanner == DurationScanner::FindMinute {
                    minute += (c as u32 - '0' as u32) * 10_u32.pow(1 - minute_number);
                    minute_number += 1;
                    if minute_number == 2 {
                        scanner = DurationScanner::End;
                    }
                } else {
                    bail!(
                        "Integer found at position {} with scanner={:?} in {}",
                        index,
                        scanner,
                        input
                    );
                }
            }
            ':' => {
                if scanner == DurationScanner::FindHourSeparator {
                    scanner = DurationScanner::FindMinute;
                } else {
                    bail!("Unexpected : at position {} in {}", index, input);
                }
            }
            c => {
                bail!(
                    "Unexpected character {} at position {} in {}",
                    c,
                    index,
                    input
                );
            }
        }
    }
    if scanner != DurationScanner::End {
        bail!(
            "scanner has not finished to parse the string. Extra characters suspected in {}",
            input
        );
    }
    let duration = Duration::hours(hour as i64) + Duration::minutes(minute as i64);
    Ok(duration)
}
