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

def _get_two_bytes(pyboy, address):
    return (pyboy.getMemoryValue(address) << 8) + pyboy.getMemoryValue(address + 0x1)



class PokemonRedService(server_pb2_grpc.PokemonRedServicer):
    def __init__(self, pyboy):
        server_pb2_grpc.PokemonRedServicer.__init__(self)
        self._pyboy = pyboy

    def GetPokemon(self, request, context):
        return Party(party=[self.get_first_pokemon()])

    def get_first_pokemon(self):
        # red offset + 1st pokemon
        return self.get_pokemon(0xd163 + 0x08)

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


