"""This is a dummy test."""
import os
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

class TestDummy(unittest.TestCase):
    def setUp(self):
        pass
    
    def tearDown(self):
        pass

    def test_dummy(self):
        pass


if __name__ == "__main__":
    unittest.main()