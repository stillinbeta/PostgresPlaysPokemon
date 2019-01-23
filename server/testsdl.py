import sdl2
import sdl2.ext

if __name__ == "__main__":
    window = sdl2.ext.Window("PyBoy", size=(800,600))
    surface = window.get_surface()

    pixels2d = sdl2.ext.pixels2d(surface)
