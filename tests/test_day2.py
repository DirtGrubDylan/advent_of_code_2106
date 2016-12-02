"""This is the day 2 test."""
import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_2 import code_from_instructions
from advents.day_2 import instructions_from_file

class TestDay2(unittest.TestCase):
    """Test the day 2 code.
    """

    def setUp(self):
        self.test_input_1 = ['ULL', 'RRDDD', 'LURDL', 'UUUUD']
        self.test_input_file_1 = (
            os.path.join(
                os.path.join(os.path.dirname(__file__), 'test_data'),
                'test_day2_data.txt'))

    def tearDown(self):
        pass

    def test_code_from_instuctions(self):
        """Test the code_from_instructions function in the day_2 module.
        """
        self.assertEqual(
            code_from_instructions(self.test_input_1), '1985')

    def test_instructions_from_file(self):
        """Test the instructions_from_file function in the day_2 module.
        """
        self.assertEqual(
            instructions_from_file(self.test_input_file_1), self.test_input_1)


if __name__ == "__main__":
    unittest.main()
