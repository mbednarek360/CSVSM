use std::process;
use colored::*;

// format and print
fn show_msg(msg_str: String, tag_str: String, tag_color: String, fatal: bool) {
    println!("{}{}{} {}", "[".bright_black().bold(), tag_str.color(tag_color).bold(), "]".bright_black().bold(), msg_str.white());
    if fatal { process::exit(1); }
}

fn throw_error(err_str: String) {
    show_msg(err_str, "ERROR".to_string(), "red".to_string(), true);
}

fn print_info(info_str: String) {
    show_msg(info_str, "INFO".to_string(), "green".to_string(), false);
}

// error implementations
pub fn cli_error() {
    throw_error(format!("{}{}{}", "Invalid command! See ", "csvsm help".cyan().bold().to_string(), " for a list of commands."));
}

pub fn dup_error(service: &String) {
    throw_error(format!("{}{}{}", "Service ", service.cyan().bold(), " already exists in file."));
}

pub fn read_error(file_name: &String) {
    throw_error(format!("{}{}{}", "File ", file_name.cyan().bold(), " cannot be read."));
}

pub fn csv_error(file_name: &String) {
    throw_error(format!("{}{}{}", "File ", file_name.cyan().bold(), " contains invalid CSV."));
}

pub fn del_error(service: &String) {
    throw_error(format!("{}{}{}", "Service ", service.cyan().bold(), " does not exist in file."));
}

// info implementations
pub fn add_success(service: &String) {
    print_info(format!("{}{}{}", "Service ", service.cyan().bold(), " successfully added."))
}

pub fn del_success(service: &String) {
    print_info(format!("{}{}{}", "Service ", service.cyan().bold(), " successfully removed."))
}

pub fn mut_success(service: &String) {
    print_info(format!("{}{}{}", "Service ", service.cyan().bold(), " successfully mutated."))
}


pub fn help() {

}
