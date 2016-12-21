"""This is a dummy test."""
import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_10 import Warehouse
from advents.day_10 import Bot

class TestDummy(unittest.TestCase):
    def setUp(self):
        self.test_warehouse = Warehouse(2, 5)
        self.test_file_name = os.path.join(
            os.path.join(os.path.dirname(__file__), 'test_data'),
            'test_day10_data.txt')

        self.test_warehouse.load_bots_with_instructions_from_file(
            self.test_file_name)

    def tearDown(self):
        pass

    def test_load_bots(self):
        test_dict = {'0': Bot(), '1': Bot(), '2': Bot()}

        test_dict['2'].instructions.append(
            'bot 2 gives low to bot 1 and high to bot 0')
        test_dict['1'].instructions.append(
            'bot 1 gives low to output 1 and high to bot 0')
        test_dict['0'].instructions.append(
            'bot 0 gives low to output 2 and high to output 0')

        test_from = [
            'value 5 goes to bot 2',
            'value 3 goes to bot 1',
            'value 2 goes to bot 2']

        for bot_id in test_dict:
            self.assertEqual(
                self.test_warehouse.bots[bot_id].instructions,
                test_dict[bot_id].instructions)

        self.assertEqual(self.test_warehouse.from_input_instructions, test_from)

    def test_execute_0(self):
        self.test_warehouse.bots['2'].values.append(5)
        self.test_warehouse.bots['2'].values.append(2)
        self.test_warehouse.bots['1'].values.append(3)

        self.test_warehouse.execute('0')

        self.assertEqual(self.test_warehouse.bots['0'].values, [])
        self.assertEqual(self.test_warehouse.bots['1'].values, [3])
        self.assertEqual(self.test_warehouse.bots['2'].values, [5, 2])

    def test_execute_1(self):
        self.test_warehouse.bots['2'].values.append(5)
        self.test_warehouse.bots['2'].values.append(2)
        self.test_warehouse.bots['1'].values.append(3)

        self.test_warehouse.execute('1')

        self.assertEqual(self.test_warehouse.bots['0'].values, [])
        self.assertEqual(self.test_warehouse.bots['1'].values, [])
        self.assertEqual(self.test_warehouse.bots['2'].values, [5, 2])

        self.assertEqual(self.test_warehouse.output_bins['1'], [3])

    def test_execute_2(self):
        self.test_warehouse.bots['2'].values.append(5)
        self.test_warehouse.bots['2'].values.append(2)
        self.test_warehouse.bots['1'].values.append(3)

        self.test_warehouse.execute('2')

        self.assertEqual(self.test_warehouse.bots['0'].values, [])
        self.assertEqual(self.test_warehouse.bots['1'].values, [])
        self.assertEqual(self.test_warehouse.bots['2'].values, [])

        self.assertEqual(self.test_warehouse.output_bins['0'], [5])
        self.assertEqual(self.test_warehouse.output_bins['1'], [2])
        self.assertEqual(self.test_warehouse.output_bins['2'], [3])

    def test_run_bots(self):
        self.test_warehouse.run_bots()

        self.assertEqual(self.test_warehouse.bots['0'].values, [])
        self.assertEqual(self.test_warehouse.bots['1'].values, [])
        self.assertEqual(self.test_warehouse.bots['2'].values, [])

        self.assertEqual(self.test_warehouse.output_bins['0'], [5])
        self.assertEqual(self.test_warehouse.output_bins['1'], [2])
        self.assertEqual(self.test_warehouse.output_bins['2'], [3])

        self.assertEqual(self.test_warehouse.desired_bot_id, '2')


if __name__ == "__main__":
    unittest.main()