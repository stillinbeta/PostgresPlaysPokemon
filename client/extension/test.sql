DROP FUNCTION pfdw() CASCADE;

CREATE FUNCTION pfdw() RETURNS fdw_handler
AS '/home/ellie/Projects/ppp/client/extension/target/debug/libpg_extension.so' , 'fdw_PokemonFDW'
  LANGUAGE C STRICT;

CREATE FOREIGN DATA WRAPPER pfdw handler pfdw NO VALIDATOR;

CREATE SERVER red FOREIGN DATA WRAPPER pfdw;

CREATE FOREIGN TABLE party (
  id Integer
, position Integer
, hp Integer
, level Integer
, max_hp Integer
, attack Integer
, defense Integer
, speed Integer
, special Integer
) SERVER red;

UPDATE party SET hp = 500, max_hp = 500 WHERE position = 4;

SELECT * FROM party;
