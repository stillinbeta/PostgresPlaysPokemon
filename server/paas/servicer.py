import logging

from . import server_pb2_grpc
from . import server_pb2 as pb

# http://datacrystal.romhacking.net/wiki/Pok%C3%A9mon_Red/Blue:RAM_map#Player
POKEMON_PARTY_COUNT = 0xD163
POKEMON_PARTY_ID_START = 0xD164
POKEMON_PARTY_START = 0xD16B
POKEMON_STRUCT_SIZE = 44

POKEMON_INVENTORY_START = 0xD31D


def _get_two_bytes(pyboy, address):
    return (pyboy.getMemoryValue(address) <<
            8) + pyboy.getMemoryValue(address + 0x1)


def _set_two_bytes(pyboy, address, value):
    pyboy.setMemoryValue(address, value >> 8)
    pyboy.setMemoryValue(address + 0x1, value & 0xFF)


class PokemonRedService(server_pb2_grpc.PokemonRedServicer):
    def __init__(self, pyboy):
        server_pb2_grpc.PokemonRedServicer.__init__(self)
        self._pyboy = pyboy

    def GetPokemon(self, request, context):
        party_size = self._pyboy.getMemoryValue(POKEMON_PARTY_COUNT)

        return pb.Party(
            party=map(self._get_pokemon_in_party, range(party_size)))

    def UpdatePokemon(self, request, context):
        start = POKEMON_PARTY_START + POKEMON_STRUCT_SIZE * request.position

        if request.WhichOneof("set_id") is not None:
            # ID is set in two places
            self._pyboy.setMemoryValue(start, request.id)
            self._pyboy.setMemoryValue(
                POKEMON_PARTY_ID_START + request.position, request.id)
        if request.WhichOneof("set_hp") is not None:
            _set_two_bytes(self._pyboy, start + 0x1, request.hp)
        if request.WhichOneof("set_level") is not None:
            self._pyboy.setMemoryValue(start + 0x3, request.level)
        if request.WhichOneof("set_max_hp") is not None:
            _set_two_bytes(self._pyboy, start + 0x22, request.max_hp)
        if request.WhichOneof("set_attack") is not None:
            _set_two_bytes(self._pyboy, start + 0x24, request.attack)
        if request.WhichOneof("set_attack") is not None:
            _set_two_bytes(self._pyboy, start + 0x26, request.attack)
        if request.WhichOneof("set_speed") is not None:
            _set_two_bytes(self._pyboy, start + 0x28, request.speed)
        if request.WhichOneof("set_special") is not None:
            _set_two_bytes(self._pyboy, start + 0x2A, request.special)

        return pb.UpdatePokemonResponse(
            pokemon=self._get_pokemon_in_party(request.position))

    def SetMemory(self, request, context):
        orig = self._pyboy.getMemoryValue(request.location)
        self._pyboy.setMemoryValue(request.location, request.value)
        return pb.MemoryResponse(original_value=orig)

    def _get_pokemon_in_party(self, i):
        pokemon = self.get_pokemon(
            POKEMON_PARTY_START + POKEMON_STRUCT_SIZE * i)
        pokemon.position = i
        return pokemon

    def get_pokemon(self, offset):
        return pb.Pokemon(
            id=self._pyboy.getMemoryValue(offset),
            hp=_get_two_bytes(self._pyboy, offset + 0x1),
            level=self._pyboy.getMemoryValue(offset + 0x3),
            max_hp=_get_two_bytes(self._pyboy, offset + 0x22),
            attack=_get_two_bytes(self._pyboy, offset + 0x24),
            defense=_get_two_bytes(self._pyboy, offset + 0x26),
            speed=_get_two_bytes(self._pyboy, offset + 0x28),
            special=_get_two_bytes(self._pyboy, offset + 0x2A),
        )

    def get_inventory_item(self, index):
        offset = (POKEMON_INVENTORY_START + 1) + (index * 2)


        return pb.InventoryItem(
            item=pb.Item(
                id=self._pyboy.getMemoryValue(offset),
                quantity=self._pyboy.getMemoryValue(offset + 1),
            ),
            position=index,
        )

    def GetInventory(self, request, context):
        total = self._pyboy.getMemoryValue(POKEMON_INVENTORY_START)
        return pb.GetInventoryResponse(
            items=map(self.get_inventory_item, range(total)))

    def _update_inventory_item(self, slot, item):
        offset = (POKEMON_INVENTORY_START + 1) + (slot * 2)
        print("setting item at {:x}".format(offset))
        self._pyboy.setMemoryValue(offset, item.id)
        self._pyboy.setMemoryValue(offset + 1, item.quantity)

    def UpdateInventory(self, request, context):
        total = self._pyboy.getMemoryValue(POKEMON_INVENTORY_START)
        item = request.item
        if item.position > total:
            raise IndexError("{} is out of range", item.position)

        self._update_inventory_item(item.position, item.item)
        return pb.UpdateInventoryResponse(
            item=self.get_inventory_item(item.position))

    def AddItem(self, request, context):
        total = self._pyboy.getMemoryValue(POKEMON_INVENTORY_START)
        if total >= 20:
            raise IndexError("We already have 20 items!")

        total += 1
        # update total
        self._pyboy.setMemoryValue(POKEMON_INVENTORY_START, total)
        # set items
        self._update_inventory_item(total - 1, request.item) # zero-indexed
        # set terminator
        terminator = POKEMON_INVENTORY_START + 1 + (2 * total)
        print("null terminator at {:x}".format(terminator))
        self._pyboy.setMemoryValue(terminator, 0xff)

        return pb.AddItemResponse(item=self.get_inventory_item(total - 1))
