use rustc_serialize::{Decoder, Decodable};
use std::fmt;
use std::str::FromStr;
use chrono::{DateTime, Local, TimeZone, FixedOffset};

#[derive(Debug)]
pub struct FloatWithPoint(f64);

#[derive(Debug)]
pub struct MyDateTime(DateTime<Local>);

impl fmt::Display for FloatWithPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for MyDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(RustcDecodable)]
pub struct PvOutputRecord {
    pub datetime: MyDateTime,
    pub cumulative: FloatWithPoint,
    pub current_status: FloatWithPoint,
}

impl Decodable for FloatWithPoint {
    fn decode<D: Decoder>(d: &mut D) -> Result<FloatWithPoint, D::Error> {
        let str_val = d.read_str().unwrap_or(String::new()).replace(",", ".");
        Ok(FloatWithPoint(f64::from_str(&*str_val).unwrap()))
    }
}

impl Decodable for MyDateTime {
    fn decode<D: Decoder>(d: &mut D) -> Result<MyDateTime, D::Error> {
        let str_val = d.read_str().unwrap_or(String::new());
        Ok(MyDateTime(Local::from_offset(&FixedOffset::east(7200)).datetime_from_str(&*str_val, "%d.%m.%Y %H:%M").unwrap()))
    }
}

