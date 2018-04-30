#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());

    let matches = app.get_matches();

}
