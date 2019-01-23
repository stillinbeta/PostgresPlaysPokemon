from concurrent import futures

import grpc

from . import server_pb2_grpc
from .servicer import PokemonRedService

class Server:
    def __init__(self, pyboy, port=50051):
        self._pyboy = pyboy
        self._server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
        server_pb2_grpc.add_PokemonRedServicer_to_server(PokemonRedService(), self._server)
        self._server.add_insecure_port('unix:/tmp/ppp')

    def start(self):
        self._server.start()

    def tick(self):
        self._pyboy.tick()

    def stop(self):
        self._pyboy.stop()
        self._server.stop()

    def run(self):
        self.start()
        while not self.tick():
            pass
        self.stop()
