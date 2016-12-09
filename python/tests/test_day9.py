import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_9 import ExperimentalSequence

class TestDummy(unittest.TestCase):
    def setUp(self):
        self.test_data_file = os.path.join(
            os.path.join(os.path.dirname(__file__), 'test_data'),
            'test_day9_data.txt')
        self.test_sequence = (
            ExperimentalSequence.experimental_sequences_from_file(
                self.test_data_file))

    def tearDown(self):
        pass

    def test_experimental_sequences_from_file(self):
        answer = 'X(8x2)(3x3)ABCY'

        self.assertEqual(self.test_sequence[5].sequence, answer)

    # def test_next_marker_with_indicies_from(self):
    #     self.assertEqual(
    #         self.test_sequence[2].next_marker_with_indicies_from(0),
    #         ((3,3), 0, 4))
    #     self.assertEqual(
    #         self.test_sequence[3].next_marker_with_indicies_from(8),
    #         ((2, 2), 9, 13))
    #     self.assertEqual(
    #         self.test_sequence[0].next_marker_with_indicies_from(0),
    #         (None, -1, -1))

    def test_decode(self):
        self.assertEqual(self.test_sequence[0].decode(), 6)
    #     self.assertEqual(self.test_sequence[1].decode(), 'ABBBBBC')
        self.assertEqual(self.test_sequence[5].decode(), 18)

    # def test_decode_v2(self):
    #     self.assertEqual(self.test_sequence[0].decode_v2(), 'ADVENT')
    #     self.assertEqual(self.test_sequence[1].decode_v2(), 'ABBBBBC')
    #     self.assertEqual(
    #         self.test_sequence[5].decode_v2(), 'XABCABCABCABCABCABCY')

    #     self.assertEqual(len(self.test_sequence[6].decode_v2()), 241920)
    #     self.assertEqual(len(self.test_sequence[7].decode_v2()), 445)

if __name__ == "__main__":
    unittest.main()