"""This is a test for day 4."""

import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_4 import RoomKey

class TestDay4(unittest.TestCase):
    """This is the unit tests for Day 4.
    """
    def setUp(self):
        self.test_data_1 = 'not-a-real-room-404[oarel]'
        self.test_room_1 = RoomKey(self.test_data_1)

    def tearDown(self):
        pass

    def test_room_name(self):
        self.assertEqual(self.test_room_1.name, 'notarealroom')

    def test_room_sector_id(self):
        self.assertEqual(self.test_room_1.sector_id, 404)

    def test_room_check_sum(self):
        self.assertEqual(self.test_room_1.check_sum, 'oarel')

    def test_unique_symbol_count(self):
        unique_symbol_count = {
            'n': 1,
            'o': 3,
            't': 1,
            'a': 2,
            'r': 2,
            'e': 1,
            'l': 1,
            'm': 1
        }

        self.assertEqual(
            self.test_room_1.unique_symbols_in_name(), unique_symbol_count)


if __name__ == "__main__":
    unittest.main()
