"""This is the Day 4 test."""

import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_4 import RoomKey

class TestDay4(unittest.TestCase):
    """This is the unit tests for Day 4 module.
    """
    def setUp(self):
        self.test_data = [
            'aaaaa-bbb-z-y-x-123[abxyz]', 'a-b-c-d-e-f-g-h-987[abcde]',
            'not-a-real-room-404[oarel]', 'totally-real-room-200[decoy]',
            'qzmt-zixmtkozy-ivhz-343[zimtb]']
        self.test_roomkeys = [RoomKey(data) for data in self.test_data]
        self.test_file_name = os.path.join(
            os.path.join(os.path.dirname(__file__), 'test_data'),
            'test_day4_data.txt')

    def tearDown(self):
        pass

    def test_roomkey_name(self):
        """This is the unit test for the RoomKey names.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(self.test_roomkeys[0].name, 'aaaaa-bbb-z-y-x')
        self.assertEqual(self.test_roomkeys[1].name, 'a-b-c-d-e-f-g-h')
        self.assertEqual(self.test_roomkeys[2].name, 'not-a-real-room')
        self.assertEqual(self.test_roomkeys[3].name, 'totally-real-room')

    def test_roomkey_sector_id(self):
        """This is the unit test for the RoomKey sector ids.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(self.test_roomkeys[0].sector_id, 123)
        self.assertEqual(self.test_roomkeys[1].sector_id, 987)
        self.assertEqual(self.test_roomkeys[2].sector_id, 404)
        self.assertEqual(self.test_roomkeys[3].sector_id, 200)

    def test_roomkey_check_sum(self):
        """This is the unit test for the RoomKey check sums.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(self.test_roomkeys[0].check_sum, 'abxyz')
        self.assertEqual(self.test_roomkeys[1].check_sum, 'abcde')
        self.assertEqual(self.test_roomkeys[2].check_sum, 'oarel')
        self.assertEqual(self.test_roomkeys[3].check_sum, 'decoy')

    def test_roomkey_name_unique_symbol_counts(self):
        """This is the unit test for the RoomKey unique symbol count
           dictionaries.

        Args:
            N/A

        Returns:
            N/A
        """
        unique_symbol_room_0 = {'a': 5, 'b': 3, 'z': 1, 'y': 1, 'x': 1}
        unique_symbol_room_1 = {
            'a': 1, 'b': 1, 'c': 1, 'd': 1, 'e': 1, 'f': 1, 'g': 1, 'h': 1}
        unique_symbol_room_2 = {
            'n': 1, 'o': 3, 't': 1, 'a': 2, 'r': 2, 'e': 1, 'l': 1, 'm': 1}
        unique_symbol_room_3 = {
            't': 2, 'o': 3, 'a': 2, 'l': 3, 'y': 1, 'r': 2, 'e': 1, 'm': 1}

        self.assertEqual(
            self.test_roomkeys[0].unique_symbol_counts(), unique_symbol_room_0)
        self.assertEqual(
            self.test_roomkeys[1].unique_symbol_counts(), unique_symbol_room_1)
        self.assertEqual(
            self.test_roomkeys[2].unique_symbol_counts(), unique_symbol_room_2)
        self.assertEqual(
            self.test_roomkeys[3].unique_symbol_counts(), unique_symbol_room_3)

    def test_roomkey_is_valid(self):
        """This is the unit test to check if a RoomKey is valid.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertTrue(self.test_roomkeys[0].is_valid())
        self.assertTrue(self.test_roomkeys[1].is_valid())
        self.assertTrue(self.test_roomkeys[2].is_valid())
        self.assertFalse(self.test_roomkeys[3].is_valid())

    def test_sum_of_valid_room_sector_ids(self):
        """This is the unit test for the summation of valid room sector id's,
           from a list of RoomKeys.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(
            RoomKey.sum_of_valid_room_sector_ids(self.test_roomkeys), 1514)

    def test_data_from_file(self):
        """This is the unit test for getting RoomKey data from a file.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(
            RoomKey.data_from_file(self.test_file_name), self.test_data)

    def test_decrypt_name(self):
        """Unit test for decrypting RoomKey name.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(
            self.test_roomkeys[4].decrypt_name(), 'very encrypted name')


if __name__ == "__main__":
    unittest.main()
