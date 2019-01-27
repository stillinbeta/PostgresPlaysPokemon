import sys
sys.path.append("PyBoy/Source")

import os

from PyBoy import PyBoy
from PyBoy.GameWindow import SdlGameWindow as Window

import paas

if __name__ == "__main__":
    # Automatically bump to '-OO' optimizations
    if __debug__:
        os.execl(sys.executable, sys.executable, '-OO', *sys.argv)

    bootROM = None
    ROM_path = "ROMs/Pokemon_Red.gb"
    scale = 2
    debug = "debug" in sys.argv and platform.system() != "Windows"

    if not os.path.exists(ROM_path) and len(sys.argv) < 2:
        print ("ROM not found. Please copy the Game-ROM to '%s'" % ROM_path)
        exit()

    try:
        # Start PyBoy and run loop
        pyboy = PyBoy(Window(scale=scale), ROM_path, bootROM)

        # def handler(signum, frame):
        #     print('Saved Name is: %s' % ' '.join(list(hex(pyboy.getMemoryValue(i)) for i in range(0xa598, 0xa5a3))))
        #     print('Player Name is: %s' % ' '.join(list(hex(pyboy.getMemoryValue(i)) for i in range(0xd158, 0xd162))))
        # signal.signal(signal.SIGUSR1, handler)

        # while not pyboy.tick():
        #     pass
        # pyboy.stop()

        server = paas.Server(pyboy)
        server.run()


    except KeyboardInterrupt:
        print ("Interrupted by keyboard")
        traceback.print_exc()
