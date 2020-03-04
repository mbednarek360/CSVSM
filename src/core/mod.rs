pub mod log;
use csv::Reader;
use std::collections::HashMap;

fn read_csv(file_name: &String) -> HashMap<String, u64> {
    let mut outp = HashMap::new();
    let mut records = Reader::from_path(file_name).unwrap();
    for service in records.records() {
        outp.insert(
            (*((&service).as_ref().unwrap().get(0).unwrap())).to_string(),
            service.unwrap().get(1).unwrap().to_string().parse::<u64>().unwrap());
    }
    outp
}

pub fn add_service(service: &String, file_name: &String) {

    // read file
    println!("{:?}", read_csv(file_name));

    // error if duplicate

    // check ids and create unique

    // write to file

}

pub fn del_service(service: &String, file_name: &String) {

}

pub fn mut_service(service: &String, file_name: &String) {

}
