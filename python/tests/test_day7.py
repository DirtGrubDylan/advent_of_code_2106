import os
import re
import sys
import unittest

sys.path.append(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), '..'))

from advents.day_7 import IPv7

class TestDay7(unittest.TestCase):
    def setUp(self):
        self.test_file = os.path.join(
            os.path.join(os.path.dirname(__file__), 'test_data'),
            'test_day7_data.txt')
        self.test_long_addr = (
            'itgslvpxoqqakli[arktzcssgkxktejbno]wsgkbwwtbmfnddt[zblrboqsvezcgf'\
            'mfvcz]iwyhyatqetsreeyhh')

    def tearDown(self):
        pass

    def test_load_data_from_file(self):
        answer = [
            'abba[mnop]qrst', 'abcd[bddb]xyyx', 'aaaa[qwer]tyui',
            'ioxxoj[asdfgh]zxcvbn', 'aba[bab]xyz', 'xyx[xyx]xyx', 'aaa[kek]eke',
            'zazbz[bzb]cdb']
        self.assertEqual(IPv7.load_data_from_file(self.test_file), answer)

    def test_supports_tls(self):
        data = IPv7.load_data_from_file(self.test_file)

        self.assertTrue(IPv7(data[0]).supports_tls())
        self.assertFalse(IPv7(data[1]).supports_tls())
        self.assertFalse(IPv7(data[2]).supports_tls())
        self.assertTrue(IPv7(data[3]).supports_tls())

    def test_contains_tls_pair(self):
        data = re.split('\W+', IPv7.load_data_from_file(self.test_file)[3])

        self.assertTrue(IPv7.contains_tls_pair(data[0]))
        self.assertFalse(IPv7.contains_tls_pair(data[1]))
        self.assertFalse(IPv7.contains_tls_pair(data[2]))

    def test_number_of_tls_supported(self):
        data = IPv7.load_data_from_file(self.test_file)

        ipv7_list = list()

        for d in data:
            ipv7_list.append(IPv7(d))

        self.assertEqual(IPv7.number_of_tls_supported(ipv7_list), 2)

    def test_hyper_net_from_address(self):
        test_ipv7 = IPv7(self.test_long_addr)
        answer = ['[arktzcssgkxktejbno]', '[zblrboqsvezcgfmfvcz]']

        self.assertEqual(test_ipv7.hyper_net_from_address(), answer)

    def test_non_hyper_net_from_address(self):
        test_ipv7 = IPv7(self.test_long_addr)
        answer = ['itgslvpxoqqakli', 'wsgkbwwtbmfnddt', 'iwyhyatqetsreeyhh']

        self.assertEqual(test_ipv7.super_net_from_address(), answer)

    def test_support_ssl(self):
        data = IPv7.load_data_from_file(self.test_file)

        self.assertTrue(IPv7(data[4]).supports_ssl())
        self.assertFalse(IPv7(data[5]).supports_ssl())
        self.assertTrue(IPv7(data[6]).supports_ssl())
        self.assertTrue(IPv7(data[7]).supports_ssl())

    def test_abas_in(self):
        self.assertEqual(IPv7.abas_in('zazbz'), ['zaz', 'zbz'])

    def test_aba_to_bab(self):
        self.assertEqual(IPv7.aba_to_bab('zbz'), 'bzb')

    def test_number_of_ssls_supported(self):
        data = IPv7.load_data_from_file(self.test_file)

        ipv7_list = list()

        for d in data:
            ipv7_list.append(IPv7(d))

        self.assertEqual(IPv7.number_of_ssls_supported(ipv7_list), 3)


if __name__ == "__main__":
    unittest.main()
