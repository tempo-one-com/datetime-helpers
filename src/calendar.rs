use std::num::TryFromIntError;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Weekday};

use crate::{Error, Result};

pub struct Calendar {
    is_saturday_off: bool,
    is_sunday_off: bool,
    holidays: Vec<NaiveDate>,
}

impl Calendar {
    pub fn new(year: i32) -> Result<Self> {
        let holidays = build_holidays(year)?;

        Ok(Calendar {
            is_saturday_off: true,
            is_sunday_off: true,
            holidays,
        })
    }

    pub fn new_with_days_off(
        year: i32,
        is_saturday_off: bool,
        is_sunday_off: bool,
    ) -> Result<Self> {
        let holidays = build_holidays(year)?;

        Ok(Calendar {
            is_saturday_off,
            is_sunday_off,
            holidays,
        })
    }

    pub fn is_day_off(&self, date: NaiveDate) -> bool {
        let is_holidays = self.holidays.iter().any(|&x| date == x);

        let saturday_off = (date.weekday() == Weekday::Sat) && self.is_saturday_off;
        let sunday_off = date.weekday() == Weekday::Sun && self.is_sunday_off;

        is_holidays || saturday_off || sunday_off
    }

    pub fn get_next_working_day(&self, date: NaiveDate, days_added: i32) -> Result<NaiveDate> {
        self.get_working_day_at_day(date, days_added, add_days)
    }

    pub fn get_previous_working_day(&self, date: NaiveDate, days_added: i32) -> Result<NaiveDate> {
        self.get_working_day_at_day(date, days_added, minus_days)
    }

    fn get_working_day_at_day(
        &self,
        date: NaiveDate,
        nb_days: i32,
        date_leaper: impl Fn(NaiveDate, i32) -> NaiveDate,
    ) -> Result<NaiveDate> {
        let other_date = date_leaper(date, nb_days);

        if self.is_day_off(other_date) {
            self.get_working_day_at_day(other_date, 1, date_leaper)
        } else {
            Ok(other_date)
        }
    }

    pub fn get_next_working_day_with_hours(&self, datetime: NaiveDateTime, hours_added: i32) -> Result<NaiveDateTime> {
        self.get_working_day_at_hour(datetime, hours_added, add_hours)
    }

    pub fn get_previous_working_day_with_hours(&self, datetime: NaiveDateTime, hours_added: i32) -> Result<NaiveDateTime> {
        self.get_working_day_at_hour(datetime, hours_added, minus_hours)
    }

    fn get_working_day_at_hour(
        &self,
        date: NaiveDateTime,
        nb_hours: i32,
        datetime_leaper: impl Fn(NaiveDateTime, i32) -> NaiveDateTime,
    ) -> Result<NaiveDateTime> {
        let other_date = datetime_leaper(date, nb_hours);

        if self.is_day_off(other_date.date()) {
            self.get_working_day_at_hour(other_date, 24, datetime_leaper)
        } else {
            Ok(other_date)
        }
    }

}

fn get_easter(year: i32) -> Result<NaiveDate> {
    let my_year: u32 = year
        .try_into()
        .map_err(|e: TryFromIntError| Error::CalendarError(e.to_string()))?;

    let (a, b) = divide(my_year, 100)?;
    let (c, d) = divide(3 * (a + 25), 4)?;
    let e = (8 * (a + 11)) / 25;
    let f = (5 * a + b) % 19;
    let g = (19 * f + c - e) % 30;
    let h = (f + 11 * g) / 319;
    let (j, k) = divide(60 * (5 - d) + b, 4)?;
    let m = (2 * j - k - g + h) % 7;
    let (n, p) = divide(g - h + m + 114, 31)?;

    let day = p + 1;
    let month = n;

    get_date(year, month, day)
}

fn build_holidays(year: i32) -> Result<Vec<NaiveDate>> {
    let easter = get_easter(year)?;
    let mut holidays: Vec<Vec<NaiveDate>> = vec![];

    //generation de 3 ans pour les cas ou passage d'année (A-1, A, A+1)
    for y in [year - 1, year, year + 1] {
        let days = vec![
            get_date(y, chrono::Month::January.number_from_month(), 1)?,
            easter + Duration::days(1),  //lundi de Pâques
            easter + Duration::days(39), //jeudi Ascension
            easter + Duration::days(50), //Pentecôte
            get_date(y, chrono::Month::May.number_from_month(), 1)?,
            get_date(y, chrono::Month::May.number_from_month(), 8)?,
            get_date(y, chrono::Month::July.number_from_month(), 14)?,
            get_date(y, chrono::Month::August.number_from_month(), 15)?,
            get_date(y, chrono::Month::November.number_from_month(), 1)?,
            get_date(y, chrono::Month::November.number_from_month(), 11)?,
            get_date(y, chrono::Month::December.number_from_month(), 25)?,
        ];

        holidays.push(days);
    }

    let holidays = holidays.into_iter().flatten().collect::<Vec<_>>();

    Ok(holidays)
}

fn divide(dividende: u32, diviseur: u32) -> Result<(u32, u32)> {
    let result = dividende / diviseur;
    let rest = dividende % diviseur;

    Ok((result, rest))
}

fn add_days(date: NaiveDate, nb_days: i32) -> NaiveDate {
    date + Duration::days(nb_days.into())
}

fn minus_days(date: NaiveDate, nb_days: i32) -> NaiveDate {
    date - Duration::days(nb_days.into())
}

fn add_hours(datetime: NaiveDateTime, nb_hours: i32) -> NaiveDateTime {
    datetime + Duration::hours(nb_hours.into())
}

fn minus_hours(datetime: NaiveDateTime, nb_hours: i32) -> NaiveDateTime {
    datetime - Duration::hours(nb_hours.into())
}


fn get_date(year: i32, month: u32, day: u32) -> Result<NaiveDate> {
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(Error::CalendarError(format!("{year}-{month}-{day}")))
}

#[cfg(test)]
mod tests {
    use chrono::{Month, NaiveTime};

    use super::*;

    #[test]
    fn janvier_1() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::January.number_from_month(), 1).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn may_1() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::May.number_from_month(), 1).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn may_8() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::May.number_from_month(), 8).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn july_14() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 14).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn august_15() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::August.number_from_month(), 15).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn november_1() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::November.number_from_month(), 1).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn november_11() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::November.number_from_month(), 11).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn december_25() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::December.number_from_month(), 25).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn lundi_paques_2018() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::April.number_from_month(), 2).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn lundi_paques_2020() {
        let cal = Calendar::new(2020).unwrap();
        let day = NaiveDate::from_ymd_opt(2020, Month::April.number_from_month(), 13).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn jeudi_ascension() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::May.number_from_month(), 10).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn lundi_pentecote() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::May.number_from_month(), 21).unwrap();

        assert!(cal.is_day_off(day))
    }

    #[test]
    fn jour_ouvre_apres_jour_ferie() {
        let cal = Calendar::new(2020).unwrap();
        let day = NaiveDate::from_ymd_opt(2020, Month::July.number_from_month(), 13).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2020, Month::July.number_from_month(), 15).unwrap();

        assert_eq!(cal.get_next_working_day(day, 1).unwrap(), next_working_day)
    }

    #[test]
    fn jour_ouvre_apres_jour_ferie_hour() {
        let cal = Calendar::new(2020).unwrap();
        let date = new_date_time(2020, Month::July, 13, 14, 0, 0);
        let next_working_day = new_date_time(2020, Month::July, 15, 14, 0, 0);
                
        assert_eq!(cal.get_next_working_day_with_hours(date, 24).unwrap(), next_working_day)
    }


    #[test]
    fn jour_36h_avant_apres_midi() {
        let cal = Calendar::new(2024).unwrap();
        let date = new_date_time(2024, Month::November, 13, 17, 0, 0);
        let next_working_day = new_date_time(2024, Month::November, 12, 5, 0, 0);
                
        assert_eq!(cal.get_previous_working_day_with_hours(date, 36).unwrap(), next_working_day)
    }

    #[test]
    fn jour_36h_avant_matin() {
        let cal = Calendar::new(2024).unwrap();
        let date = new_date_time(2024, Month::November, 13, 8, 0, 0);
        let next_working_day = new_date_time(2024, Month::November, 8, 20, 0, 0);
                
        assert_eq!(cal.get_previous_working_day_with_hours(date, 36).unwrap(), next_working_day)
    }

    #[test]
    fn jour_ouvre_annee_suivante() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::December.number_from_month(), 31).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2019, Month::January.number_from_month(), 2).unwrap();

        assert_eq!(cal.get_next_working_day(day, 1).unwrap(), next_working_day)
    }

    #[test]
    fn jour_ouvre_annee_suivante_() {
        let cal = Calendar::new(2018).unwrap();
        let date = new_date_time(2018, Month::December, 31, 10, 0, 0);
        let next_working_day =
            new_date_time(2019, Month::January, 2, 10, 0, 0);

        assert_eq!(cal.get_next_working_day_with_hours(date, 24).unwrap(), next_working_day)
    }

    #[test]
    fn jour_ouvre_annee_precedente() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2019, Month::January.number_from_month(), 2).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2018, Month::December.number_from_month(), 31).unwrap();

        assert_eq!(
            cal.get_previous_working_day(day, 1).unwrap(),
            next_working_day
        )
    }

    #[test]
    fn jour_suivant_samedi_ouvre_mais_ferie() {
        let cal = Calendar::new_with_days_off(2018, false, true).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 13).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 16).unwrap();

        assert_eq!(cal.get_next_working_day(day, 1).unwrap(), next_working_day)
    }

    #[test]
    fn jour_suivant_ouvre_mais_ferie_ouvert_dimanche() {
        let cal = Calendar::new_with_days_off(2018, false, false).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 13).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 15).unwrap();

        assert_eq!(cal.get_next_working_day(day, 1).unwrap(), next_working_day)
    }

    #[test]
    fn jour_ouvre_n_jours_apres() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 9).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 16).unwrap();

        assert_eq!(cal.get_next_working_day(day, 5).unwrap(), next_working_day)
    }

    #[test]
    fn jour_ouvre_n_jours_avant() {
        let cal = Calendar::new(2018).unwrap();
        let day = NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 13).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2018, Month::July.number_from_month(), 11).unwrap();

        assert_eq!(
            cal.get_previous_working_day(day, 2).unwrap(),
            next_working_day
        )
    }

    #[test]
    fn passage_weekend() {
        let cal = Calendar::new(2024).unwrap();
        let day = NaiveDate::from_ymd_opt(2024, Month::November.number_from_month(), 6).unwrap();
        let next_working_day =
            NaiveDate::from_ymd_opt(2024, Month::October.number_from_month(), 31).unwrap();

        assert_eq!(
            cal.get_previous_working_day(day, 3).unwrap(),
            next_working_day
        )
    }

    fn new_date_time(year: i32, month: Month, day: u32, hour: u32, minute: u32, seconde: u32) -> NaiveDateTime {
        let date = NaiveDate::from_ymd_opt(year, month.number_from_month(), day).unwrap();
        let time = NaiveTime::from_hms_opt(hour, minute, seconde).unwrap();        

        NaiveDateTime::new(date, time)
    }
}
