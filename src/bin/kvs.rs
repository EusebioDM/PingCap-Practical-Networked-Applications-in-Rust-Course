extern crate clap;
extern crate structopt;

use clap::{App, Arg, ArgMatches, SubCommand};
use kvs::KvStore;
use std::io::Write;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("V").short("V").help("Print the version"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(
                    Arg::with_name("KEY")
                        .index(1)
                        .required(true)
                        .multiple(false),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .index(2)
                        .required(true)
                        .multiple(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(
                    Arg::with_name("KEY")
                        .index(1)
                        .required(true)
                        .multiple(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm").about("Remove a given key").arg(
                Arg::with_name("KEY")
                    .index(1)
                    .required(true)
                    .multiple(false),
            ),
        )
        .get_matches();
    let mut kvs = KvStore::new();

    if matches.is_present("V") {
        println!(env!("CARGO_PKG_VERSION"))
    }
    if let Some(args) = matches.subcommand_matches("set") {
        unimplemented_exit();
        let key = get_value(args, "KEY");
        let value = get_value(args, "VALUE");

        kvs.set(key, value);
    } else if let Some(args) = matches.subcommand_matches("get") {
        unimplemented_exit();
        let key = get_value(args, "KEY");

        kvs.get(key);
    } else if let Some(args) = matches.subcommand_matches("rm") {
        unimplemented_exit();
        let key = get_value(args, "KEY");

        kvs.remove(key);
    } else {
        unimplemented_exit()
    }
}

fn get_value(matches: &ArgMatches, name: &str) -> String {
    matches.values_of(name).unwrap().next().unwrap().to_string()
}

fn unimplemented_exit() {
    std::io::stderr().write_all(b"unimplemented").unwrap();
    std::process::exit(1)
}
