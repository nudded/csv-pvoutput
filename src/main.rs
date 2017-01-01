extern crate pvoutput;
extern crate ini;
extern crate csv;
extern crate rustc_serialize;
extern crate chrono;

mod parsing;

use ini::Ini;
use parsing::*;

fn main() {
    let ini_config = Ini::load_from_file("pvoutput.ini").unwrap();
    let apikey = ini_config.get_from(Some("api"), "api_key").unwrap();
    let systemid = ini_config.get_from(Some("api"), "system_id").unwrap();
    println!("api_key: {}, system_id: {}", apikey, systemid);
    let mut rdr = csv::Reader::from_file("/vagrant/Downloads/Archief/2100323955/test.csv").unwrap().
        delimiter(b';');
    for record in rdr.decode() {
        let record: PvOutputRecord = record.unwrap();

    }

}
