import re
import os
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

    def turn_on_pixels_in_rectangle(self, width, height):
        for row in range(height):
            for column in range(width):
                self._pixels[row][column] = True

    def rotate_pixel_row(self, row_number, offset):
        self._pixels[row_number].rotate(offset)

    def rotate_pixel_column(self, column_number, offset):
        column = deque([row[column_number] for row in self._pixels])
        column.rotate(offset)

        for row, new_column_value in zip(self._pixels, column):
            row[column_number] = new_column_value

    def number_of_active_pixels(self):
        return sum(row.count(True) for row in self._pixels)

    def interpret_instruction(self, instruction):
        instruction = re.split('\W+|x', instruction)

        if instruction[0] == 'rotate' and instruction[1] == 'column':
            self.rotate_pixel_column(
                int(instruction[-3]), int(instruction[-1]))
        if instruction[0] == 'rotate' and instruction[1] == 'row':
            self.rotate_pixel_row(
                int(instruction[-3]), int(instruction[-1]))
        elif instruction[0] == 'rect':
            self.turn_on_pixels_in_rectangle(
                int(instruction[1]), int(instruction[2]))


    def __str__(self):
        return_string = 'Display:\n'

        for row in self._pixels:
            print_row = ['#' if pixel else ' ' for pixel in row]
            return_string += ''.join(print_row) + '\n'

        return return_string

    def interpret_instructions_from_file(self, file_name):
        with open(file_name, 'r') as data_file:
            for line in data_file:
                instruction = line.strip()
                self.interpret_instruction(instruction)


def main():
    data_file_name = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day8_input.txt')

    broken_screen = Screen(50, 6)

    broken_screen.interpret_instructions_from_file(data_file_name)

    print('Answer 1: {}'.format(broken_screen.number_of_active_pixels()))

    print(broken_screen)

if __name__ == "__main__":
    main()
