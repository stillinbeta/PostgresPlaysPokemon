# PostgresPlaysPokemon

Have you ever wanted to to play pokemon, but also cheat?
Is the GameGenie interface a little bit too user friendly for you? 
Do you want to do inner joins on lists of Pokemon?

PostgresPlaysPokemon is for you!

## Installation

### Server

`cd ./server`

The "server" in this case is a visual emulator that exposes a GRPC interface.
It is written in Python, and requires [pypy][pypy] to run at a reasonable speed.
You'll need the Python2 PyPy version pending resolution of [this issue][numpypy].

1. [Install pypy and SDL][pyboy].
2. Set up a virtualenv: `virtualenv -p $(which pypy) venv`
3. Activate the virtualenv: `. venv/bin/activate`
4. Install the requirements: `pip install -r requirements.txt`
5. Launch the emulator: `python main.py`

[pyboy]: https://github.com/Baekalfen/PyBoy#starting-the-emulator
[numpypy]: https://bitbucket.org/pypy/pypy/issues/2930/memoryview-ctypesstructure-does-not
[pypy]: https://pypy.org/download.html

### Client

`cd ./client/extension`

The "client" is a postgres extension. 
You first need to compile it, then make Postgres aware of it, and then you can use it!

1. Install [rust and cargo][rustup]
2. Install [Postgres][pg]. I test with Postgres 10; Other versions may or may not work.
3. run `make` to build the extension
4. run `sudo make install` to install all the required files where Postgres expects them.
5. If you don't have one already, set up a [postgres admin][pgadmin]. On Linux at least, if you make a superuser matching your username and a database matching your username, you can just type `psql`.
6. Get a `psql` shell. It doesn't matter what database you use, where we're going we don't need databases!
7. `CREATE EXTENSION pokemon`;
8. `IMPORT FOREIGN SCHEMA red FROM SERVER red INTO public`;
9. All done! 

| Table     | Cooresponds To                | Operations Supported   |
|:----------|:------------------------------|:-----------------------|
| party     | Your (up to) six pokemon      | SELECT, UPDATE         |
| inventory | Items you're carryin          | SELECT, INSERT, UPDATE |
| story     | Critical story event triggers | SELECT, UPDATE         |

Pokemon ID are [not the same as pokemon numbers][pid]. 
Items are listed [here][items]. 
Keep in mind that by default, Postgres will insert decimal numbers, not hexidecimal.

[rustup]: https://rustup.rs/
[pg]: https://www.postgresql.org/download/
[pgadmin]: https://medium.com/coding-blocks/creating-user-database-and-adding-access-on-postgresql-8bfcd2f4a91e
[pid]: https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_index_number_(Generation_I)
[items]: https://bulbapedia.bulbagarden.net/wiki/List_of_items_by_index_number_(Generation_I)

#### CLI

`cd ./client`

There's also a CLI that performs much of the same actions as the Postgres API.
`cargo run -- --help` should list all available operations. 
You'll need [rust and cargo][rustup] but not Postgres.
But where's the fun in that?
