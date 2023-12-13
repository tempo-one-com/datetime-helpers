pub mod calendar;
pub mod formatter;
pub mod parser;

#[derive(Debug)]
pub enum Error {
    CalendarError(String),
    ParamsError(String),
}

type Result<T> = std::result::Result<T, Error>;