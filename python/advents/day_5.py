"""This is Day 5 of Advent of Code."""

import hashlib
from datetime import datetime


class Door(object):
    """Door class.
    """

    def __init__(self, door_id):
        super().__init__()

        self.door_id = door_id
        self.code_index = 0

    def next_id_hash(self):
        """Returns the sixth character from the next md5 hashed binary of the
           door_id combined with the code_index, if the hashed binary starts
           with five leading zeroes.

        Args:
            N/A

        Returns:
            (str): The sixth character of the next md5 hash value of the door_id
                   combined with the code_index.
        """
        hex_hash = ''

        while hex_hash[:5] != '00000':
            combined_hash_input = (
                self.door_id + str(self.code_index)).encode(encoding='utf_8')

            hex_hash = hashlib.md5(combined_hash_input).hexdigest()

            self.code_index += 1

        return hex_hash

    def code_from_id(self):
        """Returns the door code from the door_id.

        Note:
            Solved left to right.

        Args:
            N/A

        Returns:
            (str): The door code.
        """
        code = ''

        for _ in range(8):
            code += self.next_id_hash()[5]

        return code

    def code_from_id_new(self, print_iterations=False):
        """Returns the door code from the door_id.

        Note:
            Solved in place.

        Args:
            print_iterations (bool): Indicates whether you want cinema printing.

        Returns:
            (str): The door code.
        """
        code = ['_' for _ in range(8)]
        num_of_symbols = 0
        possible_indicies = set([str(num) for num in range(8)])

        while num_of_symbols < 8:
            index, code_symbol = self.next_id_hash()[5:7]

            if index in possible_indicies:
                index = int(index)

                if code[index] == '_':
                    num_of_symbols += 1
                    code[int(index)] = code_symbol

            if print_iterations:
                print(
                    'Finding new door code: {}'.format(''.join(code)), end='\r')

        return ''.join(code)


def main():
    """Day 5 module main.
    """
    door_id = 'wtnhxymk'
    door = Door(door_id)

    start_time = datetime.now()

    print('The door code is: {}'.format(door.code_from_id()))

    door.code_index = 0

    print('\nThe new door code is: {}'.format(door.code_from_id_new(True)))

    print(
        'Time to decode door_id [{}]: {}s'.format(
            door_id, (datetime.now() - start_time).total_seconds()))

if __name__ == '__main__':
    main()
