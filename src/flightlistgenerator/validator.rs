use anyhow::bail;
use anyhow::Result;
use time::{Duration, PrimitiveDateTime};

pub fn validate_duration(
    duration: Duration,
    date_start: &PrimitiveDateTime,
    date_end: &PrimitiveDateTime,
) -> Result<()> {
    //if let Some(total_duration) = duration {
    if duration != *date_end - *date_start {
        bail!(
            "total_duration={} different of {} - {}",
            duration,
            date_end,
            date_start
        );
        // }
    }
    Ok(())
}

pub fn validate_date(date_start: &PrimitiveDateTime, date_end: &PrimitiveDateTime) -> Result<()> {
    if *date_start >= *date_end {
        bail!(
            "take-off after landing : date_start={}, date_end={}",
            date_start,
            date_end
        );
    }
    if *date_end - *date_start > Duration::hours(20) {
        bail!(
            "flight more than 20 hours: seems impossible : date_start={}, date_end={}",
            date_start,
            date_end
        );
    }
    Ok(())
}

/// todo for flightlist
/// Add checks :
/// flight time check, chronology,..
pub fn perform_basic_check() {
    // chronological order
    // check airports
    // check
}
