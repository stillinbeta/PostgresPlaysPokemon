import sys
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

    if not os.path.exists(ROM_path) and len(sys.argv) < 2:
        print ("ROM not found. Please copy the Game-ROM to '%s'" % ROM_path)
        exit()

    try:
        # Start PyBoy and run loop
        pyboy = PyBoy(Window(scale=scale), ROM_path, bootROM)
        server = paas.Server(pyboy)
        server.run()
    except KeyboardInterrupt:
        print ("Interrupted by keyboard")
        traceback.print_exc()
