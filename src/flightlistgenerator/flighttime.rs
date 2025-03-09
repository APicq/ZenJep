use std::convert::From;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use time::Date;
use time::Duration;
use time::PrimitiveDateTime;
use time::Time;

/// This structure is used to express all flight hours.
#[derive(Copy, Clone, PartialEq)]
pub struct FlightTime(pub Duration);

impl FlightTime {
    pub const ZERO: Self = FlightTime(Duration::ZERO);
}

impl Deref for FlightTime {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for FlightTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Duration::ZERO {
            write!(f, "")
        } else {
            let full_minutes = self.0.whole_minutes();
            let hours = full_minutes / 60;
            let minutes = full_minutes % 60;
            write!(f, "{hours}:{minutes:02}")
        }
    }
}

//todo fancy
impl Debug for FlightTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_minutes = self.0.whole_minutes();
        let hours = full_minutes / 60;
        let minutes = full_minutes % 60;
        f.debug_struct("FlightTime")
            .field("hours:", &hours)
            .field("minutes:", &minutes)
            .finish()
    }
}

impl From<Duration> for FlightTime {
    fn from(d: Duration) -> Self {
        FlightTime(d)
    }
}

impl Add<PrimitiveDateTime> for FlightTime {
    type Output = PrimitiveDateTime;
    fn add(self, other: PrimitiveDateTime) -> Self::Output {
        other + self.0
    }
}

impl AddAssign for FlightTime {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Default for FlightTime {
    fn default() -> Self {
        Self(Duration::ZERO)
    }
}

#[derive(Clone, Debug, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct FlightDate(pub Date);

impl FlightDate {
    pub fn year(&self) -> i32 {
        self.0.year()
    }
    /// Month number
    /// 0 -> January
    pub fn month(&self) -> u8 {
        let month: u8 = self.0.month().into();
        month - 1
    }
}

impl Display for FlightDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}/{:02}/{}",
            self.0.day(),
            self.0.month() as u8,
            self.0.year()
        )
    }
}

#[derive(Clone, Debug)]
pub struct TimeOfDate(pub Time);

impl Display for TimeOfDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.0.hour(), self.0.minute())
    }
}
