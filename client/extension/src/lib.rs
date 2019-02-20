extern crate pg_extend;
extern crate pg_extern_attr;
extern crate ppp_client;


use ppp_client::{Client,UpdatePokemon};
use pg_extend::pg_fdw::{ForeignRow, ForeignData, OptionMap, Tuple};
use pg_extend::{pg_datum, pg_magic, pg_type, pg_error};
use pg_extern_attr::pg_foreignwrapper;

// Bring try_from into scope
use pg_extend::pg_datum::TryFromPgDatum;

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

fn get_column (row: &Tuple, name: &str) -> Option<u32> {
    match row.get(name.into()) {
        Some(v) => match i32::try_from((*v).clone()) {
            Ok(i) => Some(i as u32),
            Err(err) => {
                pg_error::log(pg_error::Level::Error, file!(), line!(), module_path!(),
                              format!("couldn't convert {}: {}", name, err));
                None
            }
        },
        None => None,
    }
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
            "position" => self.0.position,
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
                    return None
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

    fn index_columns(_server_opts: OptionMap, _table_opts: OptionMap) -> Option<Vec<String>> {
       Some(vec!("position".into()))
    }

    fn update(&self, row: &Tuple, indices: &Tuple) -> Option<Box<ForeignRow>> {
        pg_error::log(pg_error::Level::Warning, file!(), line!(), module_path!(), format!("row: {:?}, indices: {:?}", row, indices));
        let client = Client::new();
        let slot = match row.get("position") {
            Some(slot) => match i32::try_from((*slot).clone()) {
                Ok(i)  => {
                    i as u32
                },
                Err(err) => {
                    pg_error::log(pg_error::Level::Error, file!(), line!(), module_path!(),
                                  format!("couldn't convert position: {}", err));
                    return None
                }
            }
            None => {
                pg_error::log(pg_error::Level::Error, file!(), line!(), module_path!(),
                              "tried to update pokemon without a position!");
                return None
            }
        };

        if slot > 5 {
            pg_error::log(pg_error::Level::Error, file!(), line!(), module_path!(),
                          format!("Invalid slot {}", slot));
            return None
        }

        let update = UpdatePokemon{
            slot: slot,
            id: get_column(row, "id"),
            hp: get_column(row, "hp"),
            level: get_column(row, "level"),
            max_hp: get_column(row, "max_hp"),
            attack: get_column(row, "attack"),
            defense: get_column(row, "defense"),
            speed: get_column(row, "speed"),
            special: get_column(row, "special"),
        };

        match client.set_pokemon(&update) {
            Ok(pokemon) => Some(Box::new(Pokemon::from(pokemon))),
            Err(err) => {
                pg_error::log(pg_error::Level::Error, file!(), line!(), module_path!(),
                              format!("Failed to update pokemon: {:?}", err));
                None
            }
        }
    }
}
