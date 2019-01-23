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

class PokemonRedService(server_pb2_grpc.PokemonRedServicer):
    def GetPokemon(self, request, context):
        return Party(pokemon=pokemon)


