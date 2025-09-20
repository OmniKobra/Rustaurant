pub use chrono::prelude::*;

pub struct Calendar {
    seconds_elapsed: u32,
    start_date: DateTime<Utc>,
}

impl Calendar {
    pub fn advance(&mut self, seconds: u32) {
        self.seconds_elapsed += seconds;
    }
    pub fn start_seconds(&self) -> i64 {
        self.start_date.timestamp()
    }
    pub fn time(&self) -> DateTime<Local> {
        let start = self.start_seconds();
        let rn = DateTime::from_timestamp(start + self.seconds_elapsed as i64, 0)
            .unwrap()
            .with_timezone(&Local);

        rn
    }
}
impl std::fmt::Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.time();
        let (hour, meridian) = meridian(date.hour());
        write!(
            f,
            "{:02}:{:02} {} {}/{}/{}",
            hour,
            date.minute(),
            meridian,
            date.day(),
            fmt_month(date.month()),
            date.year()
        )
    }
}

fn meridian(hour: u32) -> (u32, &'static str) {
    match hour {
        0..=11 => (hour, "AM"),
        12 => (hour, "PM"),
        13..=23 => (hour - 12, "PM"),
        _ => (0, "UNDEFINED"),
    }
}
fn fmt_month(m: u32) -> &'static str {
    let months: [&'static str; 12] = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    months[m as usize - 1]
}

impl Default for Calendar {
    fn default() -> Self {
        Self {
            seconds_elapsed: 0,
            start_date: Utc::now(),
        }
    }
}
