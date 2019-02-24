extern crate grpc;
extern crate pg_extend;
extern crate pg_extern_attr;
extern crate ppp_client;

use pg_extend::pg_fdw::{ForeignData, ForeignRow, OptionMap, Tuple};
use pg_extend::{pg_datum, pg_error, pg_magic, pg_type};
use pg_extern_attr::pg_foreignwrapper;
use ppp_client::{Client, UpdatePokemon};

// Bring try_from into scope
use pg_extend::pg_datum::TryFromPgDatum;

pg_magic!(version: pg_sys::PG_VERSION_NUM);

struct Pokemon(ppp_client::server::Pokemon);
struct Event(ppp_client::server::Event);
struct Inventory(ppp_client::server::InventoryItem);

impl From<ppp_client::server::Pokemon> for Pokemon {
    fn from(p: ppp_client::server::Pokemon) -> Pokemon {
        Pokemon(p)
    }
}

enum Table {
    PARTY,
    INVENTORY,
    STORY,
}

macro_rules! create_party {
    ($s:expr, $n:expr) => {
        format!(
            "
CREATE FOREIGN TABLE {}.party (
  id Integer
, position Integer
, hp Integer
, level Integer
, max_hp Integer
, attack Integer
, defense Integer
, speed Integer
, special Integer
) SERVER {};
",
            $s, $n
        )
    };
}

macro_rules! create_inventory {
    ($s:expr, $n:expr) => {
        format!(
            "
CREATE FOREIGN TABLE {}.inventory (
  id Integer
, position Integer
, quantity Integer
) SERVER {};
",
            $s, $n
        )
    };
}

macro_rules! create_story {
    ($s:expr, $n:expr) => {
        format!(
            "
CREATE FOREIGN TABLE {}.story (
  event text
, setting integer
) SERVER {};
",
            $s, $n
        )
    };
}

#[pg_foreignwrapper]
struct PokemonFDW {
    table: Table,
    retrieved: bool,
    items: Vec<Box<ForeignRow>>,
}

fn get_column(row: &Tuple, name: &str) -> Result<Option<u32>, String> {
    Ok(match row.get(name.into()) {
        Some(v) => Some(i32::try_from((v).clone())? as u32),
        None => None,
    })
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

impl ForeignRow for Inventory {
    fn get_field(
        &self,
        name: &str,
        _typ: pg_type::PgType,
        _opts: OptionMap,
    ) -> Result<Option<pg_datum::PgDatum>, &str> {
        let val = match name {
            "id" => self.0.get_item().id,
            "quantity" => self.0.get_item().quantity,
            "position" => self.0.position,
            _ => return Err("No such field'"),
        };
        Ok(Some((val as i32).into()))
    }
}

impl ForeignRow for Event {
    fn get_field(
        &self,
        name: &str,
        _typ: pg_type::PgType,
        _opts: OptionMap,
    ) -> Result<Option<pg_datum::PgDatum>, &str> {
        match name {
            "event" => Ok(Some(format!("{:?}", self.0.event).into())),
            "setting" => Ok(Some((self.0.setting as i32).into())),
            _ => Err("No such field'"),
        }
    }
}

impl Iterator for PokemonFDW {
    type Item = Box<ForeignRow>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.retrieved {
            let client = Client::new();
            let res = match self.table {
                // TODO: clean this mess up
                Table::PARTY => client.get_pokemon().map(|r| {
                    r.iter()
                        .map(|v| Box::new(Pokemon(v.clone())) as Box<ForeignRow>)
                        .collect()
                }),
                Table::INVENTORY => client.get_inventory().map(|r| {
                    r.iter()
                        .map(|v| Box::new(Inventory(v.clone())) as Box<ForeignRow>)
                        .collect()
                }),
                Table::STORY => client.get_events().map(|r| {
                    r.iter()
                        .map(|v| Box::new(Event(v.clone())) as Box<ForeignRow>)
                        .collect()
                }),
            };
            match res {
                Ok(vec) => {
                    self.items = vec;
                    self.retrieved = true;
                }
                Err(err) => {
                    pg_error::log(
                        pg_error::Level::Error,
                        file!(),
                        line!(),
                        module_path!(),
                        format!("{:?}", err),
                    );
                    return None;
                }
            }
        }

        self.items.pop()
    }
}

impl PokemonFDW {
    fn update_party(&self, row: &Tuple, _indices: &Tuple) -> Result<Box<ForeignRow>, String> {
        let client = Client::new();
        let slot = match row.get("position") {
            Some(slot) => match i32::try_from((*slot).clone()) {
                Ok(i) => i as u32,
                Err(err) => return Err(format!("couldn't convert position: {}", err)),
            },
            None => return Err("tried to update pokemon without a position!".into()),
        };

        if slot > 5 {
            return Err(format!("Invalid slot {}", slot));
        }

        let update = UpdatePokemon {
            slot: slot,
            id: get_column(row, "id")?,
            hp: get_column(row, "hp")?,
            level: get_column(row, "level")?,
            max_hp: get_column(row, "max_hp")?,
            attack: get_column(row, "attack")?,
            defense: get_column(row, "defense")?,
            speed: get_column(row, "speed")?,
            special: get_column(row, "special")?,
        };

        match client.set_pokemon(&update) {
            Ok(pokemon) => Ok(Box::new(Pokemon::from(pokemon))),
            Err(err) => Err(format!("Failed to update pokemon: {:?}", err)),
        }
    }

    fn get_number_column(row: &Tuple, column: &str) -> Result<u32, String> {
        match row.get(column) {
            Some(slot) => match i32::try_from((*slot).clone()) {
                Ok(i) => Ok(i as u32),
                Err(err) => Err(format!("couldn't convert {}: {}", column, err)),
            },
            None => Err("tried to update inventory without a position!".into()),
        }
    }

    fn update_inventory(&self, row: &Tuple, _indices: &Tuple) -> Result<Box<ForeignRow>, String> {
        let client = Client::new();
        let position = Self::get_number_column(row, "position")?;
        let id = Self::get_number_column(row, "id")?;
        let quantity = Self::get_number_column(row, "quantity")?;

        match client.update_item(position, id, quantity) {
            Ok(i) => Ok(Box::new(Inventory(i))),
            Err(err) => Err(format!("Update failed: {:?}", err)),
        }
    }
    fn update_event(&self, row: &Tuple, _indices: &Tuple) -> Result<Box<ForeignRow>, String> {
        let client = Client::new();
        let evt_string = match row.get("event") {
            Some(evt) => String::try_from((*evt).clone())?,
            None => return Err("Missing event".into()),
        };

        let evt = match evt_string.as_str() {
            "GOT_POKEDEX" => ppp_client::server::Event_EventType::GOT_POKEDEX,
            "DELIVERED_PARCEL" => ppp_client::server::Event_EventType::DELIVERED_PARCEL,
            "GOT_PARCEL" => ppp_client::server::Event_EventType::GOT_PARCEL,
            evt => return Err(format!("Unknown event {}", evt)),
        };

        // TODO: should be a bool
        let setting = Self::get_number_column(row, "setting")?;

        match client.set_event(evt, setting != 0) {
            Ok(i) => Ok(Box::new(Event(i))),
            Err(err) => Err(format!("Update failed: {:?}", err)),
        }
    }

    fn insert_item(&self, row: &Tuple) -> Result<Box<ForeignRow>, String> {
        let client = Client::new();
        let id = Self::get_number_column(row, "id")?;
        let quantity = Self::get_number_column(row, "quantity")?;
        if let Some(v) = row.get("position") {
            if !v.is_null() {
                return Err("Can't provide position for insert".into());
            }
        }

        match client.add_item(id, quantity) {
            Ok(i) => Ok(Box::new(Inventory(i))),
            Err(err) => Err(format!("Insert failed: {:?}", err)),
        }
    }
}

impl ForeignData for PokemonFDW {
    fn begin(_sopts: OptionMap, _topts: OptionMap, table_name: String) -> Self {
        PokemonFDW {
            table: match table_name.as_str() {
                "party" => Table::PARTY,
                "inventory" => Table::INVENTORY,
                "story" => Table::STORY,
                table => {
                    pg_error::log(
                        pg_error::Level::Error,
                        file!(),
                        line!(),
                        module_path!(),
                        format!("Unknown table: {}", table),
                    );
                    Table::PARTY
                }
            },
            retrieved: false,
            items: Vec::new(),
        }
    }

    fn schema(
        _server_opts: OptionMap,
        server_name: String,
        _remote_schema: String,
        local_schema: String,
    ) -> Option<Vec<String>> {
        Some(vec![
            create_party!(local_schema, server_name),
            create_inventory!(local_schema, server_name),
            create_story!(local_schema, server_name),
        ])
    }

    fn index_columns(
        _server_opts: OptionMap,
        _table_opts: OptionMap,
        table_name: String,
    ) -> Option<Vec<String>> {
        match table_name.as_str() {
            "party" | "inventory" => Some(vec!["position".into()]),
            "story" => Some(vec!["event".into()]),
            table => {
                pg_error::log(
                    pg_error::Level::Error,
                    file!(),
                    line!(),
                    module_path!(),
                    format!("unknown table {}", table),
                );
                None
            }
        }
    }

    fn update(&self, row: &Tuple, indices: &Tuple) -> Option<Box<ForeignRow>> {
        let res = match self.table {
            Table::STORY => self.update_event(row, indices),
            Table::PARTY => self.update_party(row, indices),
            Table::INVENTORY => self.update_inventory(row, indices),
        };

        match res {
            Ok(res) => Some(res),
            Err(err) => {
                pg_error::log(
                    pg_error::Level::Error,
                    file!(),
                    line!(),
                    module_path!(),
                    err,
                );
                None
            }
        }
    }

    fn insert(&self, row: &Tuple) -> Option<Box<ForeignRow>> {
        let res = match self.table {
            Table::STORY => Err("Can't insert into Story".into()),
            Table::PARTY => Err("Can't insert into Party".into()),
            Table::INVENTORY => self.insert_item(row),
        };

        match res {
            Ok(res) => Some(res),
            Err(err) => {
                pg_error::log(
                    pg_error::Level::Error,
                    file!(),
                    line!(),
                    module_path!(),
                    err,
                );
                None
            }
        }
    }
}
