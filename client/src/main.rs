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
                         .index(2)))
        .subcommand(SubCommand::with_name("update-pokemon")
                    .arg(Arg::with_name("SLOT")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("id")
                         .long("id")
                         .help("Set pokemon id (kind)")
                         .takes_value(true))
                    .arg(Arg::with_name("hp")
                         .long("hp")
                         .help("Set pokemon HP")
                         .takes_value(true)))
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
        Some("update-pokemon") => {
            let matches = matches.subcommand_matches("update-pokemon").unwrap();
            let slot = u32::from_str_radix(
                matches.value_of("SLOT").unwrap()
            , 10).expect("Failed to parse SLOT");
            let id = matches.value_of("id").map(|v| u32::from_str_radix(v, 10).expect("failed to parse ID"));
            let hp = matches.value_of("hp").map(|v| u32::from_str_radix(v, 10).expect("failed to parse ID"));
            let resp = client.set_pokemon(slot, id, hp);
            println!("Pokemon: {:?}", resp)
        }
        Some(cmd) => panic!(format!("unknown subcommand {}", cmd)),
        None => {
            eprintln!("You must specify a subcommand");
            std::process::exit(1);
        }
    }
}
