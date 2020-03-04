use std::env;
mod core;

fn main() {
    let args: Vec<String> = env::args().collect();
    //if args.len() != 4 { core::log::cli_error(); }
    match &args[1] as &str {
        "add" => core::add_service(&args[2], &args[3]),
        "del" => core::del_service(&args[2], &args[3]),
        "mut" => core::mut_service(&args[2], &args[3]),
        "help" => core::log::help(),
        _ => core::log::cli_error()
    }
}
