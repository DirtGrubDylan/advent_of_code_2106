"""This is a test for day 3."""

import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_3 import is_valid_triangle
from advents.day_3 import num_of_valid_triangles
from advents.day_3 import triangles_rows_from_file
from advents.day_3 import triangles_cols_from_file

class TestDay3(unittest.TestCase):
    """Testing day 3 module.
    """
    def setUp(self):
        self.test_input_1 = [5, 10, 25]
        self.test_file_name_1 = os.path.join(
            os.path.join(os.path.dirname(__file__), 'test_data'),
            'test_day3_data.txt')

    def tearDown(self):
        pass

    def test_triangles_rows_from_file(self):
        """Tests the triangles rows from file function in the day_3 module.

        Args:
            N/A

        Returns:
            N/A
        """
        expected_answer = [
            [775, 785, 361], [622, 375, 125], [297, 839, 375], [245, 38, 891],
            [503, 463, 849], [731, 482, 759]]

        self.assertEqual(
            triangles_rows_from_file(self.test_file_name_1),
            expected_answer)


    def test_triangles_cols_from_file(self):
        """Tests the triangles cols from file function in the day_3 module.

        Args:
            N/A

        Returns:
            N/A
        """
        expected_answer = [
            [775, 622, 297], [785, 375, 839], [361, 125, 375], [245, 503, 731],
            [38, 463, 482], [891, 849, 759]]

        self.assertEqual(
            triangles_cols_from_file(self.test_file_name_1),
            expected_answer)


    def test_num_of_valid_triangles(self):
        """Tests the num_of_valid_triangles function in the day_3 module.

        Args:
            N/A

        Returns:
            N/A
        """
        second_input_list = triangles_rows_from_file(self.test_file_name_1)

        self.assertEqual(num_of_valid_triangles([self.test_input_1]), 0)
        self.assertEqual(num_of_valid_triangles(second_input_list), 3)

    def test_is_valid_triangle(self):
        """Tests the is_valid_triangle function in the day_3 module.

        Args:
            N/A

        Returns:
            N/A
        """
        self.assertFalse(is_valid_triangle(self.test_input_1))


if __name__ == "__main__":
    unittest.main()
