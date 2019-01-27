extern crate pg_extend;
extern crate pg_extern_attr;
extern crate ppp_client;


use ppp_client::Client;
use pg_extend::pg_fdw::{ForeignRow, ForeignData, OptionMap};
use pg_extend::{pg_datum, pg_magic, pg_type, pg_error};
use pg_extern_attr::pg_foreignwrapper;

pg_magic!(version: pg_sys::PG_VERSION_NUM);


struct Pokemon(ppp_client::server::Pokemon);

impl From<ppp_client::server::Pokemon> for Pokemon {
    fn from(p: ppp_client::server::Pokemon) -> Pokemon {
        Pokemon(p)
    }
}

#[pg_foreignwrapper]
struct PokemonFDW{
    retrieved: bool,
    pokemon: Vec<Pokemon>,
}



impl ForeignRow for Pokemon {
    fn get_field(
        &self,
        name: &str,
        _typ: pg_type::PgType,
        _opts: OptionMap,
    ) -> Result<Option<pg_datum::PgDatum>, &str> {
        let val = match name {
            "id" => self.0.id,
            "hp" => self.0.hp,
            "level" => self.0.level,
            "max_hp" => self.0.max_hp,
            "attack" => self.0.attack,
            "defense" => self.0.defense,
            "speed" => self.0.speed,
            "special" => self.0.special,
            _ => return Err("No such field'"),
        };
        Ok(Some((val as i32).into()))
    }
}

impl Iterator for PokemonFDW {
    type Item = Box<ForeignRow>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.retrieved {
            let client = Client::new();
            match client.get_pokemon() {
                Ok(vec) => {
                    self.pokemon = vec.into_iter().map(|p| p.into()).collect();
                    self.retrieved = true;
                }
                Err(err) => {
                    pg_error::log(pg_error::Level::Error, file!(), line!(), module_path!(), format!("{:?}", err));
                    return None;
                }
            }

        }

        match self.pokemon.pop() {
            Some(p) => Some(Box::new(p)),
            None => None,
        }
    }
}

impl ForeignData for PokemonFDW {
    fn begin(_sopts: OptionMap, _topts: OptionMap) -> Self {
        PokemonFDW{
            retrieved: false,
            pokemon: Vec::new(),
        }
    }
}
