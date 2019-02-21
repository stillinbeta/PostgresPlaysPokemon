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


CREATE FOREIGN TABLE inventory (
  id Integer
, position Integer
, quantity Integer
) SERVER red;

CREATE FOREIGN TABLE story (
  event text
, setting integer
) SERVER red;


SELECT * FROM party;

SELECT * FROM inventory;

SELECT * FROM story;
