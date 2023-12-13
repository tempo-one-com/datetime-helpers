use chrono::{NaiveDate, NaiveDateTime};

use crate::{Result, Error};

/// value: yyyy-mm-dd
pub fn parse_from_iso_date(value: &str) -> Result<NaiveDate> {
   NaiveDate::parse_from_str(value, "%Y-%m-%d")
      .map_err(|e| Error::ParamsError(e.to_string()))
}

/// value: yyyy-mm-ddThh:mm:ss
pub fn parse_from_iso_date_time(value: &str) -> Result<NaiveDateTime> {
   NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
      .map_err(|e| Error::ParamsError(e.to_string()))
}