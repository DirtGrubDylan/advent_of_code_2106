from collections import deque


class Screen(object):
    def __init__(self, width, height):
        super().__init__()

        self._width = width
        self._height = height
        self._pixels = self.build_pixels()

    def build_pixels(self):
        pixels = []

        for row in range(self._height):
            row = deque()

            for column in range(self._width):
                row.append(False)

            pixels.append(row)

        return pixels

    def turn_on_pixels_in_rectangle(self, start_row, start_column, width,
                                    height):
        for row in range(start_row, height):
            for column in range(start_column, width):
                self._pixels[row][column] = True

    def shift_pixel_row(self, row_number, offset):
        self._pixels.rotate(offset)

    def shift_pixel_column(self, column_number, offset):
        for row in range(len(self._pixels)):
            pass

    def number_of_active_pixels(self):
        pass

    def interpret_instruction(self, instruction):
        pass

    @staticmethod
    def interpret_instructions(instructions):
        pass


def main():
    broken_screen = Screen(7, 3)

    print(broken_screen._pixels)

if __name__ == "__main__":
    main()
