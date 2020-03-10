pub mod log;
use csv::Reader;
use std::collections::HashMap;
use rand::{thread_rng, Rng};

fn contains_id(records: HashMap<String, u64>, id: u64) -> bool {
    return records.values().collect::<Vec<&u64>>().contains(&&id);
}

fn read_csv(file_name: &String) -> HashMap<String, u64> {
    let mut outp = HashMap::new();
    let records = Reader::from_path(file_name);

    // check if error reading
    if records.is_err() {
        log::read_error(file_name);
    }

    // loop through records
    for service in records.unwrap().records() {

        // error if not csv
        if (&service).as_ref().is_err() {
            log::csv_error(file_name);
        }
        if (&service).as_ref().unwrap().len() != 2 {
            log::csv_error(file_name);
        }

        // insert pair
        outp.insert(
            (*((&service).as_ref().unwrap().get(0).unwrap())).to_string(),
            service.unwrap().get(1).unwrap().to_string().parse::<u64>().unwrap());
    }
    outp
}

pub fn add_service(service: &String, file_name: &String) {

    // read file
    let mut input: HashMap<String, u64> = read_csv(file_name);

    // ensure service not already in file
    if input.contains_key(service) {
        log::dup_error(service);
    }

    // create unique id
    let mut new_id: u64 = thread_rng().gen();
    while contains_id(input.clone(), new_id) {
        new_id = thread_rng().gen();
    }

    // check ids and create unique
    input.insert(service.clone(), new_id);

    // write to file
    println!("{:?}", input);
}

pub fn del_service(service: &String, file_name: &String) {

}

pub fn mut_service(service: &String, file_name: &String) {

}
