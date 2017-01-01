extern crate csv;
extern crate rustc_serialize;
extern crate chrono;


use rustc_serialize::{Decoder, Decodable};
use std::fmt;
use std::str::FromStr;
use chrono::{DateTime, Local, TimeZone, FixedOffset};


#[derive(Debug)]
struct FloatWithPoint(f64);

#[derive(Debug)]
struct MyDateTime(DateTime<Local>);

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
struct PvOutputRecord {
    datetime: MyDateTime,
    value: FloatWithPoint,
    othervalue: FloatWithPoint,
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


fn main() {
    let mut rdr = csv::Reader::from_file("/vagrant/Downloads/Archief/2100323955/test.csv").unwrap().
        delimiter(b';');
    for record in rdr.decode() {
        let record: PvOutputRecord = record.unwrap();
        println!("({},{},{})", record.datetime, record.value, record.othervalue);
    }

}
