import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_6 import RepCode

class TestDay6(unittest.TestCase):
    def setUp(self):
        test_file = os.path.join(
            os.path.join(os.path.dirname(__file__), 'test_data'),
            'test_day6_data.txt')
        self.test_rep_code = RepCode(test_file)

    def tearDown(self):
        pass

    def test_load_from_file(self):
        test_data = ( 
            'eedadn, drvtee, eandsr, raavrd, atevrs, tsrnev, sdttsa, rasrtv, '
            'nssdts, ntnada, svetve, tesnvt, vntsnd, vrdear, dvrsen, enarar')

        test_data = test_data.split(', ')
        self.assertEqual(self.test_rep_code.data, test_data)

    def test_ml_message(self):
        self.test_rep_code.decode_ml_data()

        self.assertEqual(self.test_rep_code.message, 'easter')

    def test_ll_message(self):
        self.test_rep_code.decode_ll_data()

        self.assertEqual(self.test_rep_code.message, 'advent')


if __name__ == "__main__":
    unittest.main()