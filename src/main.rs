extern crate pvoutput;
extern crate ini;
extern crate csv;
extern crate rustc_serialize;
extern crate chrono;

mod parsing;

use ini::Ini;
use parsing::*;
use pvoutput::*;

impl From<PvOutputRecordParsed> for Status {

    fn from(rec: PvOutputRecordParsed) -> Status {
        let arg1 = format!("{}", rec.datetime.format("%Y%m%d"));
        let arg2 = format!("{}", rec.datetime.format("%H:%M"));
        let arg3 = format!("{}", (rec.current_status * 1000.0).floor());
        Status::simple_for_v2(
            arg1,
            arg2,
            arg3)
    }
}

impl From<PvOutputRecord> for Status {

    fn from(rec: PvOutputRecord) -> Status {
        let parsed_rec: PvOutputRecordParsed = rec.into();
        parsed_rec.into()
    }
}

fn main() {
    let ini_config = Ini::load_from_file("pvoutput.ini").unwrap();
    let api_key = ini_config.get_from(Some("api"), "api_key").unwrap();
    let system_id = ini_config.get_from(Some("api"), "system_id").unwrap();
    let file_dir = ini_config.get_from(Some("api"), "file_dir").unwrap();

    println!("api_key: {}, system_id: {}, file_dir: {}", api_key, system_id, file_dir);
    println!("api_key: {}, system_id: {}", api_key, system_id);

    let pvoutput = PvOutput::new(api_key, system_id);

    let mut rdr = csv::Reader::from_file("/vagrant/Downloads/Archief/2100323955/test.csv").
        unwrap().
        delimiter(b';');
    for record in rdr.decode() {
        let record: PvOutputRecord = record.unwrap();
        let status: Status = record.into();
        println!("{:?}", status);

    }

}
