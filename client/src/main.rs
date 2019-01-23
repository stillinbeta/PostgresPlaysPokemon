extern crate ppp_client;

use ppp_client::Client;

fn main() {
    let client = Client::new();

    for pokemon in client.get_pokemon().expect("Failed to retrieve pokemon") {
        println!("Found pokemon: {:?}", pokemon)
    }
}
