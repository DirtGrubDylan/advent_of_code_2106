import os
import re

class IPv7(object):
    def __init__(self, address):
        super().__init__()

        self._address = address
        self._hyper_net_sequences = self.hyper_net_from_address()
        self._super_net_sequences = self.super_net_from_address()

    def hyper_net_from_address(self):
        return re.findall('\[\w+\]', self._address)

    def super_net_from_address(self):
        return re.split('\[\w+\]', self._address)

    def supports_tls(self):
        for hyper_net in self._hyper_net_sequences:
            if self.contains_tls_pair(hyper_net):
                return False

        for non_hyper_net in self._super_net_sequences:
            if self.contains_tls_pair(non_hyper_net):
                return True

        return False

    def supports_ssl(self):
        for super_net in self._super_net_sequences:
            for aba in self.abas_in(super_net):
                bab = self.aba_to_bab(aba)

                for hyper_net in self._hyper_net_sequences:
                    if bab in hyper_net:
                        return True

        return False

    @staticmethod
    def abas_in(word):
        abas = list()

        if len(word) < 3:
            return abas

        for index in range(len(word)):
            if word[index : index + 2] == word[index + 1 : index + 3][::-1]:
                if word[index] != word[index + 1]:
                    abas.append(word[index : index + 3])

        return abas

    @staticmethod
    def aba_to_bab(aba_word):
        return aba_word[1] + aba_word[0] + aba_word[1]

    @staticmethod
    def contains_tls_pair(word):
        if word == '':
            return False

        for index in range(len(word)):
            if word[index : index + 2] == word[index + 2 : index + 4][::-1]:
                if word[index] != word[index + 1]:
                    return True

        return False

    @staticmethod
    def load_data_from_file(file_name):
        data = list()

        with open(file_name, 'r') as data_file:
            for line in data_file:
                data.append(line.strip())

        return data

    @staticmethod
    def number_of_tls_supported(ipv7_list):
        num_supported = 0

        for ipv7_address in ipv7_list:
            if ipv7_address.supports_tls():
                num_supported += 1

        return num_supported

    @staticmethod
    def number_of_ssls_supported(ipv7_list):
        num_supported = 0

        for ipv7_address in ipv7_list:
            if ipv7_address.supports_ssl():
                num_supported += 1

        return num_supported


def main():
    file_name = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day7_input.txt')

    file_data = IPv7.load_data_from_file(file_name)

    ipv7_list = list()

    for data in file_data:
        ipv7_list.append(IPv7(data))

    number_of_ipv7_tls = IPv7.number_of_tls_supported(ipv7_list)

    print('Answer 1: {}'.format(number_of_ipv7_tls))

    number_of_ipv7_ssls = IPv7.number_of_ssls_supported(ipv7_list)

    print('Answer 2: {}'.format(number_of_ipv7_ssls))


if __name__ == '__main__':
    main()
