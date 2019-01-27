import logging

from . import server_pb2_grpc
from .server_pb2 import Party, Pokemon

pokemon = [
    Pokemon(
        id=0x54, # pikachu
        hp=35,
        level=50,
        max_hp=35,
        attack=55,
        defense=30,
        speed=90,
        special=50,
   )
]

# http://datacrystal.romhacking.net/wiki/Pok%C3%A9mon_Red/Blue:RAM_map#Player
POKEMON_PARTY_COUNT = 0xD163
POKEMON_PARTY_START = 0xD16B
POKEMON_STRUCT_SIZE = 44

def _get_two_bytes(pyboy, address):
    return (pyboy.getMemoryValue(address) << 8) + pyboy.getMemoryValue(address + 0x1)



class PokemonRedService(server_pb2_grpc.PokemonRedServicer):
    def __init__(self, pyboy):
        server_pb2_grpc.PokemonRedServicer.__init__(self)
        self._pyboy = pyboy

    def GetPokemon(self, request, context):
        party_size = self._pyboy.getMemoryValue(POKEMON_PARTY_COUNT)

        return Party(party=map(self._get_pokemon_in_party, range(party_size)))

    def _get_pokemon_in_party(self, i):
        return self.get_pokemon(POKEMON_PARTY_START + POKEMON_STRUCT_SIZE * i)

    def get_pokemon(self, offset):
        return Pokemon(
            id=self._pyboy.getMemoryValue(offset),
            hp=_get_two_bytes(self._pyboy, offset + 0x1),
            level=self._pyboy.getMemoryValue(offset + 0x3),
            max_hp=_get_two_bytes(self._pyboy, offset + 0x22),
            attack=_get_two_bytes(self._pyboy, offset + 0x24),
            defense=_get_two_bytes(self._pyboy, offset + 0x26),
            speed=_get_two_bytes(self._pyboy, offset + 0x28),
            special=_get_two_bytes(self._pyboy, offset + 0x2A),
        )


