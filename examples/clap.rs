extern crate clap;
use clap::{Arg, App, SubCommand};
 
fn main() {
    let app = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use")
             .required(true)
             .index(1))
        .subcommand(SubCommand::with_name("test")
                    .about("controls testing features")
                    .arg(Arg::with_name("debug")
                         .short("d")
                         .help("print debug information verbosely")));

    // Parse the command line arguments
    let matches = app.get_matches();
 
    let config = matches.value_of("config").unwrap_or("default.conf");
    let input = matches.value_of("INPUT").unwrap();

    // Handle subcommands
    match matches.subcommand() {
        ("clone",  Some(sub_matches)) => {
            if matches.is_present("d") {
                // ...
            }
        },
        ("push",   Some(sub_matches)) => {},
        ("commit", Some(sub_matches)) => {},
        _ => {},
    } 
}
