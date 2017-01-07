extern crate pvoutput;
extern crate ini;
extern crate csv;
extern crate rustc_serialize;
extern crate chrono;
extern crate clap;

mod parsing;

use ini::Ini;
use parsing::*;
use pvoutput::*;
use std::path::Path;

use clap::App;

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

fn do_import<'a>(file: &Path, pvoutput: &'a PvOutput, test_run: bool) {

    let mut rdr = csv::Reader::from_file(file).unwrap().delimiter(b';');
    for record in rdr.decode() {
        let record: PvOutputRecord = record.unwrap();
        let status: Status = record.into();
        if test_run {
            println!("{:?}", status)
        } else {
            println!("{:?}", pvoutput.send_request(status));
        }
    }

}

fn create_pv_output_from_ini<'a>(ini_config: &'a Ini) -> PvOutput<'a> {
    let api_key = ini_config.get_from(Some("api"), "api_key").unwrap();
    let system_id = ini_config.get_from(Some("api"), "system_id").unwrap();
    PvOutput::new(api_key, system_id)
}

fn main() {
    let ini_config = Ini::load_from_file(Path::new("pvoutput.ini")).unwrap();
    let pvoutput = create_pv_output_from_ini(&ini_config);
    let matches = App::new("csv-pvoutput")
                    .version("1.0")
                    .author("Toon Willems")
                    .about("upload csv status data to pvoutput.org")
                    .args_from_usage(
                        "-f, --file=[FILE]  'file to import'
                        -p, --path=[FILE]   'path to import all files from'
                        -d, --dry-run       'do not actually send'")
                    .get_matches();
    let test_run = matches.is_present("dry-run");
    let file = Path::new(matches.value_of("file").unwrap());
    do_import(file, &pvoutput, test_run);
}
