"""This is Day 4."""

import os
from collections import deque


class RoomKey(object):
    def __init__(self, data):
        end_of_name_in_data = data.rfind('-')
        end_of_sector_id_in_data = data.rfind('[')

        self.name = data[:end_of_name_in_data].replace('-', '')
        self.sector_id = int(
            data[end_of_name_in_data + 1:end_of_sector_id_in_data])
        self.check_sum = data[end_of_sector_id_in_data + 1:-1]

    def unique_symbols_in_name(self):
        unique_symbol_counts = {}

        for symbol in self.name:
            try:
                unique_symbol_counts[symbol] += 1
            except KeyError:
                unique_symbol_counts[symbol] = 1

        return unique_symbol_counts

    def


def main():
    """This is day 4 module main.
    """
    file_name = (
        os.path.join(os.path.dirname(__file__), 'data'))

    print(file_name)


if __name__ == '__main__':
    main()
