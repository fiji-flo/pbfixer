#[macro_use]
extern crate clap;
extern crate protobuf;

use std::process::exit;
use clap::App;

mod protogen;

mod utils;
mod fix;

fn main() {
    let mut app = App::new("pbfixer");
    let matches = clap_app!(app =>
        (version: "0.1")
        (author: "Florian Merz <flomerz@gmail.com>")
        (about: ":/")
        (@arg OUT: -o --out +required +takes_value "output dir")
        (@arg DATASET: -d --dataset +required +takes_value "data set to fix")
        ).get_matches();
    let ret = match (matches.value_of("OUT"), matches.value_of("DATASET")) {
        (Some(out), Some(dataset)) => fix::do_fix(dataset, out),
        _ => {
            if let Err(e) = app.print_help() {
                println!("FATAL: {}", e);
            }
            exit(1);
        }
    };
    match ret {
        Ok(_) => println!("DONE"),
        Err(e) => println!("ERROR: {}", e),
    }
}
