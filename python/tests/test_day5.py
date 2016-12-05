"""This is the unit test for Day 5."""
import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_5 import Door

class TestDay5(unittest.TestCase):
    """This is the unit tests for Day 5 module.
    """
    def setUp(self):
        self.test_door = Door('abc')

    def tearDown(self):
        pass

    def test_next_id_hash(self):
        """Unit test the next_id_hash function of the Door class.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(self.test_door.next_id_hash()[5], '1')
        self.assertEqual(self.test_door.next_id_hash()[5], '8')
        self.assertEqual(self.test_door.next_id_hash()[5], 'f')

    def test_code_from_id(self):
        """Unit test the code_from_id function of the Door class.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(self.test_door.code_from_id(), '18f47a30')

    def test_code_from_id_new(self):
        """Unit test the code_from_id_new function of the Door class.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertEqual(self.test_door.code_from_id_new(), '05ace8e3')


if __name__ == "__main__":
    unittest.main()
