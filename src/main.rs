#![feature(custom_derive, plugin)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

#[macro_use]
mod util;

use clap::{Arg, App};

fn main() {
    let matches = App::new("kuebico")
        .version(crate_version!())
        .author("Lee Olayvar <http://leeo.la>")
        .about("A data friendly wiki")
        .arg(Arg::with_name("fs-storage")
            .short("f")
            .long("fs-storage")
            .help("Use fs storage (default is true)")
            .conflicts_with("sqlite-storage"))
        .arg(Arg::with_name("sqlite-storage")
            .short("s")
            .long("sqlite-storage")
            .help("Use fs storage (default: true)")
            .conflicts_with("fs-storage"))
        .arg(Arg::with_name("fs-dir")
            .long("fs-dir")
            .help("Directory for fs-storage (default: ./storage)")
            .value_name("DIR")
            .takes_value(true)
            .conflicts_with("sqlite-storage"))
        .arg(Arg::with_name("static-dir")
            .long("static-dir")
            .help("Directory which holds static files")
            .value_name("DIR")
            .takes_value(true))
        .arg(Arg::with_name("templates")
            .long("templates")
            .help("Directory which holds template files (default: ./templates)")
            .value_name("DIR")
            .takes_value(true))
        .arg(Arg::with_name("export")
            .long("export")
            .help("Export *all* wiki pages to the given dir")
            .value_name("DIR")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    println!("WIP");
}
