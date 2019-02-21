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

#[derive(Debug)]
pub struct UpdatePokemon {
    pub slot: u32,
    pub id: Option<u32>,
    pub hp: Option<u32>,
    pub level: Option<u32>,
    pub max_hp: Option<u32>,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    pub speed: Option<u32>,
    pub special: Option<u32>,
}

impl From<&UpdatePokemon> for  server::UpdatePokemonRequest {
    fn from(p: &UpdatePokemon) -> server::UpdatePokemonRequest {
        let mut req = server::UpdatePokemonRequest::new();
        req.set_position(p.slot.into());
        if let Some(id) = p.id {
            req.set_id(id.into())
        }
        if let Some(hp) = p.hp {
            req.set_hp(hp.into())
        }
        if let Some(level) = p.level {
            req.set_level(level.into())
        }
        if let Some(max_hp) = p.max_hp {
            req.set_max_hp(max_hp.into())
        }
        if let Some(attack) = p.attack {
            req.set_attack(attack.into())
        }
        if let Some(defense) = p.defense {
            req.set_defense(defense.into())
        }
        if let Some(speed) = p.speed {
            req.set_speed(speed.into())
        }
        if let Some(special) = p.special {
            req.set_special(special.into())
        }
        req
    }

}

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

    pub fn get_inventory(&self) -> Result<Vec<server::InventoryItem>, grpc::Error> {
        let req = server::GetInventoryRequest::new();
        let resp = self.client.get_inventory(grpc::RequestOptions::new(), req);
        let (_, items, _) = resp.wait()?;

        Ok(items.get_items().to_vec())
    }

    pub fn add_item(&self, id: u32, quantity: u32) -> Result<server::InventoryItem, grpc::Error>{
        let mut req = server::AddItemRequest::new();
        req.set_item(server::Item{id, quantity, ..Default::default()});
        let resp = self.client.add_item(grpc::RequestOptions::new(), req);

        let (_, mut resp, _) = resp.wait()?;
        Ok(resp.take_item())
    }

    pub fn update_item(&self, slot: u32, id: u32, quantity: u32) -> Result<server::InventoryItem, grpc::Error> {
        let invitem = server::InventoryItem::new();
        invitem.set_item(server::Item{id, quantity, ..Default::default()});
        invitem.set_position(slot);

        let mut req = server::UpdateInventoryRequest::new();
        req.set_item(invitem);
        let resp = self.client.update_inventory(grpc::RequestOptions::new(), req);

        let (_, mut resp, _) = resp.wait()?;
        Ok(resp.take_item())
     }

    pub fn set_memory(&self, dest: u32, val: u32) -> Result<u32, grpc::Error> {
        let mut req = server::MemoryUpdate::new();
        req.set_location(dest.into());
        req.set_value(val.into());
        let resp = self.client.set_memory(grpc::RequestOptions::new(), req);
        let (_, old, _) = resp.wait()?;
        Ok(old.original_value.into())
    }

    pub fn set_pokemon(&self, p: &UpdatePokemon) -> Result<server::Pokemon, grpc::Error> {
        let resp = self.client.update_pokemon(grpc::RequestOptions::new(), p.into());
        let (_, mut poke, _) = resp.wait()?;
        Ok(poke.take_pokemon())
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
