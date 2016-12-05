"""This is Day 5 of Advent of Code."""

import hashlib


class Door(object):
    """Door class.
    """

    def __init__(self, door_id):
        super().__init__()

        self.door_id = door_id
        self.code_index = 0

    def next_id_hash_character(self):
        """Returns the sixth character from the next md5 hashed binary of the
           door_id combined with the code_index, if the hashed binary starts
           with five leading zeroes.

        Args:
            N/A

        Returns:
            (str): The sixth character of the next md5 hash value of the door_id
                   combined with the code_index.
        """
        first_six_hex_hash_characters = ''

        while first_six_hex_hash_characters[:5] != '00000':
            combined_hash_input = (self.door_id + str(self.code_index)).encode()

            md5_hasher = hashlib.md5()
            md5_hasher.update(combined_hash_input)
            first_six_hex_hash_characters = md5_hasher.hexdigest()[:6]

            self.code_index += 1

        return first_six_hex_hash_characters[-1]

    def code_from_id(self):
        """Returns the door code from the door_id.

        Args:
            N/A

        Returns:
            (str): The door code.
        """
        code = ''

        for _ in range(8):
            code += self.next_id_hash_character()

        return code


def main():
    """Day 5 module main.
    """
    door = Door('wtnhxymk')

    print('The door code is: {}'.format(door.code_from_id()))

if __name__ == '__main__':
    main()
