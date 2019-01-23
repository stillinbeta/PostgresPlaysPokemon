extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

mod server_grpc;
pub mod server;

// Add new_plain_unix method
use grpc::ClientStubExt;

use server_grpc::PokemonRed;

pub struct Client {
    client: server_grpc::PokemonRedClient,
}

const SOCKET_PATH: &str = "/tmp/ppp";

impl Client {
    pub fn new() -> Self {
        let conf = Default::default();

        Client {
            client: server_grpc::PokemonRedClient::new_plain_unix(SOCKET_PATH, conf).unwrap()
        }
    }

    pub fn get_pokemon(&self) -> Result<Vec<server::Pokemon>, grpc::Error>{
        let req = server::GetPokemonRequest::new();
        let resp = self.client.get_pokemon(grpc::RequestOptions::new(), req);
        let (_, party, _) = resp.wait()?;

        Ok(party.get_party().to_vec())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
