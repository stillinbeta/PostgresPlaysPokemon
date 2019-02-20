#[macro_use]
extern crate clap;
extern crate ppp_client;

use clap::{SubCommand,Arg};
use ppp_client::Client;

fn get_arg(matches: &clap::ArgMatches, key: &str) -> Option<u32> {
    matches.value_of(key) .map(|v| u32::from_str_radix(v, 10).expect(&format!("failed to parse \"{}\"", key)))
}

macro_rules! make_arg {
    ( $key:expr ) => {
        {
            Arg::with_name($key)
                .long($key)
                .help(&format!("Set pokemon '{}'", $key).clone())
                .takes_value(true)
        }
    };
}

fn main() {

    let matches = app_from_crate!()
        .subcommand(SubCommand::with_name("list-party"))
        .subcommand(SubCommand::with_name("list-inventory"))
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
                    .arg(make_arg!("id"))
                    .arg(make_arg!("hp"))
                    .arg(make_arg!("level"))
                    .arg(make_arg!("max-hp"))
                    .arg(make_arg!("attack"))
                    .arg(make_arg!("defense"))
                    .arg(make_arg!("speed"))
                    .arg(make_arg!("special"))
        )
        .get_matches();

    let client = Client::new();

    match matches.subcommand_name() {
        Some("list-party") => {
            for pokemon in client.get_pokemon().expect("Failed to retrieve pokemon") {
                println!("Found pokemon: {:?}", pokemon)
            }
        }
        Some("list-inventory") => {
            for item in client.get_inventory().expect("Failed to retrieve items") {
                println!("Found {} of {:x}", item.get_quantity(), item.get_id())
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
            let resp = client.set_pokemon(
                &ppp_client::UpdatePokemon{
                    slot: get_arg(matches, "SLOT").unwrap(),
                    id: get_arg(matches, "id"),
                    hp: get_arg(matches, "hp"),
                    level: get_arg(matches, "level"),
                    max_hp: get_arg(matches, "max-hp"),
                    attack: get_arg(matches, "attack"),
                    defense: get_arg(matches, "defense"),
                    speed: get_arg(matches, "speed"),
                    special: get_arg(matches, "special"),
                }
            );
            println!("Pokemon: {:?}", resp)
        }
        Some(cmd) => panic!(format!("unknown subcommand {}", cmd)),
        None => {
            eprintln!("You must specify a subcommand");
            std::process::exit(1);
        }
    }
}
