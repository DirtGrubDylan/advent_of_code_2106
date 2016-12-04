"""Test day 1"""
import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_1 import longest_block_distance
from advents.day_1 import distance_to_first_repeated_block

class TestDay1(unittest.TestCase):
    """This is the unit tests for Day 1.
    """
    def setUp(self):
        self.test_1 = ['R2', 'L3']
        self.test_2 = ['R2', 'R2', 'R2']
        self.test_3 = ['R5', 'L5', 'R5', 'R3']
        self.test_4 = ['R8', 'R4', 'R4', 'R8']
        self.test_5 = ['R1000', 'R1000', 'R1000', 'R1000']


    def test_longest_block_distances(self):
        """Tests to make sure the longest block distance function works.
        """
        self.assertEqual(longest_block_distance(self.test_1), 5)
        self.assertEqual(longest_block_distance(self.test_2), 2)
        self.assertEqual(longest_block_distance(self.test_3), 12)
        self.assertEqual(longest_block_distance(self.test_4[:2]), 12)
        self.assertEqual(longest_block_distance(self.test_4), 8)

    def test_distance_to_first_repeated_block(self):
        """Tests to make sure the longest block distance function works.
        """
        self.assertEqual(distance_to_first_repeated_block(self.test_3), -1)
        self.assertEqual(distance_to_first_repeated_block(self.test_4), 4)
        self.assertEqual(distance_to_first_repeated_block(self.test_5), 0)


if __name__ == '__main__':
    unittest.main()
