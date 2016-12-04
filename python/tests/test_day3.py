"""This is a dummy test."""
import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_3 import is_valid_triangle
from advents.day_3 import triangles_from_file
from advents.day_3 import num_of_valid_triangles

class TestDay3(unittest.TestCase):
    """Testing day 3 module.
    """
    def setUp(self):
        self.test_input_1 = (5, 10, 25)
        self.test_file_name_1 = os.path.join(
            os.path.join(os.path.dirname(__file__), 'test_data'),
            'test_day3_data.txt')

    def tearDown(self):
        pass

    def test_triangles_from_file(self):
        """Tests the triangles from file function in the day_3 module.

        Args:
            N/A

        Returns:
            N/A
        """
        expected_answer = [
            (775, 785, 361), (622, 375, 125), (297, 839, 375), (245, 38, 891),
            (503, 463, 849)]

        self.assertEqual(
            triangles_from_file(self.test_file_name_1),
            expected_answer)


    def test_num_of_valid_triangles(self):
        """Tests the num_of_valid_triangles function in the day_3 module.

        Args:
            N/A

        Returns:
            N/A
        """
        second_input_list = triangles_from_file(self.test_file_name_1)

        self.assertEqual(num_of_valid_triangles([self.test_input_1]), 0)
        self.assertEqual(num_of_valid_triangles(second_input_list), 2)

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
