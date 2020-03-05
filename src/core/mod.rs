pub mod log;
use csv::Reader;
use std::collections::HashMap;
use rand::{thread_rng, Rng};

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
    let input: HashMap<String, u64> = read_csv(file_name);

    if input.contains_key(service) {
        log::dup_error(service);
    }

    let rand_id: u64 = thread_rng().gen();
    println!("{}", rand_id);

    // check ids and create unique

    // write to file

}

pub fn del_service(service: &String, file_name: &String) {

}

pub fn mut_service(service: &String, file_name: &String) {

}
