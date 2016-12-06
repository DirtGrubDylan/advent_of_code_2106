"""This is Day 4 of Advent of Code."""

import os

class RoomKey(object):
    """RoomKey class
    """

    def __init__(self, data):
        super().__init__()

        end_of_name_in_data = data.rfind('-')
        end_of_sector_id_in_data = data.rfind('[')

        self.name = data[:end_of_name_in_data]
        self.sector_id = int(
            data[end_of_name_in_data + 1 : end_of_sector_id_in_data])
        self.check_sum = data[end_of_sector_id_in_data + 1 : -1]

    def unique_symbol_counts(self):
        """Returns a dictionary who's key's are characters, in the RoomKey.name,
           with an associated value of their occurance count.

        Args:
            N/A

        Returns:
            (dict): A dictionary with key value pairs of characters in the
                    RoomKey.name and the number of their occurance in the
                    string.
        """
        unique_symbols = {}

        for symbol in self.name:
            if symbol != '-':
                try:
                    unique_symbols[symbol] += 1
                except KeyError:
                    unique_symbols[symbol] = 1

        return unique_symbols

    def is_valid(self):
        """Returns a boolean indicating whether a RoomKey is valid, according to
           it's check_sum value.

        Args:
            N/A

        Returns:
            (bool): Indicator to whether a RoomKey.name is compatable with it's
                    check_sum.
        """
        sorted_unique_symbols = sorted(
            self.unique_symbol_counts().items(),
            key=lambda item: (-item[1], item[0]))

        first_five_most_used_letters = (
            ''.join([item[0] for item in sorted_unique_symbols])[:5])

        return first_five_most_used_letters == self.check_sum

    def decrypt_name(self):
        """Returns the decryption of the RoomKey.name.

        Args:
            N/A

        Retuns:
            (str): The decrypted name.
        """
        decrypted = ''

        for symbol in self.name:
            if symbol == '-':
                decrypted += ' '
            else:
                new_ascii_value = (
                    (((ord(symbol) - 97) + (self.sector_id % 26)) % 26) + 97)
                decrypted += chr(new_ascii_value)

        return decrypted

    @staticmethod
    def sum_of_valid_room_sector_ids(roomkey_list):
        """Returns the sum of sector id's within the given list of RoomKeys. If
           a RoomKey is valid, of course.

        Args:
            roomkey_list (list): A list of RoomKey objects.

        Returns:
            (int): The sum of sector id's from valid RoomKeys in the list.
        """
        sum_of_valid_sector_ids = 0

        for roomkey in roomkey_list:
            if roomkey.is_valid():
                sum_of_valid_sector_ids += roomkey.sector_id

        return sum_of_valid_sector_ids

    @staticmethod
    def data_from_file(file_name):
        """Returns a list of RoomKey data strings from a file.

        Args:
            file_name (str): The file name, including path, to the data file.

        Returns:
            (list): A list of RoomKey data strings.
        """
        list_of_roomkey_data = []

        with open(file_name, 'r') as data_file:
            for line in data_file:
                list_of_roomkey_data.append(line.rstrip('\r\n'))

        return list_of_roomkey_data

    @staticmethod
    def sector_id_room_holding_north_pole_objects(roomkey_list):
        """Returns sector_id of a room, in the given list, holding North Pole
           Objects.

        Args:
            roomkey_list (list): A list of RoomKey objects.

        Returns:
            (int): The sector id of the room holding North Pole Objects.
        """
        key_words = ['north', 'pole', 'objects']

        for roomkey in roomkey_list:
            if roomkey.is_valid():
                decrypted_name = roomkey.decrypt_name()

                for key_word in key_words:
                    if key_word in decrypted_name:
                        return roomkey.sector_id


def main():
    """This is the main of the module."""
    file_name = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day4_input.txt')

    list_of_roomkey_data = RoomKey.data_from_file(file_name)

    roomkeys_from_data = [RoomKey(data) for data in list_of_roomkey_data]

    sum_of_valid_ids = RoomKey.sum_of_valid_room_sector_ids(roomkeys_from_data)

    bad_room = RoomKey.sector_id_room_holding_north_pole_objects(
        roomkeys_from_data)

    print(
        'Sum of the sector IDs of the real rooms: {}'.format(sum_of_valid_ids))
    print(
        'Sector ID of room with stolen objects: {}'.format(bad_room))


if __name__ == '__main__':
    main()
