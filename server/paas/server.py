class Server:
    def __init__(self, pyboy):
        self._pyboy = pyboy

    def tick(self):
        self._pyboy.tick()

    def stop(self):
        self._pyboy.stop()
