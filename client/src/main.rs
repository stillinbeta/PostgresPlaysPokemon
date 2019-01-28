#[macro_use]
extern crate clap;
extern crate ppp_client;

use clap::{SubCommand,Arg};
use ppp_client::Client;

fn main() {

    let matches = app_from_crate!()
        .subcommand(SubCommand::with_name("list-party"))
        .subcommand(SubCommand::with_name("set-memory")
                    .arg(Arg::with_name("DESTINATION")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("VALUE")
                         .required(true)
                         .index(2))
        )
        .get_matches();

    let client = Client::new();

    match matches.subcommand_name() {
        Some("list-party") => {
            for pokemon in client.get_pokemon().expect("Failed to retrieve pokemon") {
                println!("Found pokemon: {:?}", pokemon)
            }
        }
        Some("set-memory") => {
            let matches = matches.subcommand_matches("set-memory").unwrap();
            let dest = u32::from_str_radix(
                matches.value_of("DESTINATION").unwrap()
            , 16).expect("Failed to parse DESTINATION");
            let val = u32::from_str_radix(
                matches.value_of("VALUE").unwrap()
            , 16).expect("Failed to parse VALUE");
            let resp = client.set_memory(dest, val).expect("Failed to set memory");
            println!("{:04x} was {:08x}, now {:08x}", dest, resp, val)
        }
        Some(cmd) => panic!(format!("unknown subcommand {}", cmd)),
        None => {
            eprintln!("You must specify a subcommand");
            std::process::exit(1);
        }
    }
}
