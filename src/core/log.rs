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

pub fn cli_error() {
    throw_error(format!("{}{}{}", "Invalid command! See ", "csvsm help".bold().to_string(), " for a list of commands."));
}

pub fn dup_error(service: &String) {
    throw_error(format!("{}{}{}", "Service ", service.bold(), " already exists in file."))
}

pub fn help() {

}
