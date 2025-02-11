use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

/// -> dd/mm/yyyy
pub fn format_to_date_fr<T>(date: T) -> String
where
    T: Datelike,
{
    format!("{:02}/{:02}/{}", date.day(), date.month(), date.year())
}

/// -> hh:mm
pub fn format_to_time_fr(date: NaiveDateTime) -> String {
    format!("{:02}:{:02}", date.hour(), date.minute())
}

/// 20230101 103000-> (yyyy-mm-dd, hh:mm:ss)
pub fn format_teliway_datetime_to_iso(date: &str, hour: &str) -> (String, String) {
    let y = &date[..=3];
    let m = &date[4..=5];
    let d = &date[6..];
    let date = format!("{y}-{m}-{d}");

    let h = &hour[..=1];
    let mm = &hour[2..=3];
    let s = &hour[4..];
    let time = format!("{h}:{mm}:{s}");

    (date, time)
}

pub fn iso_date_to_teliway_date(date: NaiveDate) -> String {
    date.to_string().replace('-', "")
}

pub fn iso_time_to_teliway_hour(date: NaiveTime) -> String {
    date.to_string()[..5].replace(':', "")
}

pub fn iso_time_to_hour(date: NaiveTime) -> String {
    date.to_string()[..5].to_string()
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use super::*;

    #[test]
    fn format_date_from_naive_date_time() {
        let date = Local
            .with_ymd_and_hms(2023, 12, 1, 10, 30, 0)
            .unwrap()
            .naive_local();

        assert_eq!("01/12/2023", format_to_date_fr(date));
        assert_eq!("10:30", format_to_time_fr(date));
        assert_eq!(
            ("2023-12-01".to_string(), "10:30:00".to_string()),
            format_teliway_datetime_to_iso("20231201", "103000")
        );
    }

    #[test]
    fn format_date_from_naive_date() {
        let date = NaiveDate::from_ymd_opt(2023, 12, 1).unwrap();

        assert_eq!("01/12/2023", format_to_date_fr(date));
    }
}
