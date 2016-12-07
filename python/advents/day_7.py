import os
import re

class IPv7(object):
    def __init__(self, address):
        super().__init__()

        self.address = address
        self.hyper_net_sequences = self.hyper_net_from_address()
        self.non_hyper_net_sequences = self.non_hyper_net_from_address()

    def hyper_net_from_address(self):
        hyper_nets = re.findall('\[\w+\]', self.address)

        return [hyper_net[1:-1] for hyper_net in hyper_nets]

    def non_hyper_net_from_address(self):
        return re.split('\[\w+\]', self.address)

    def supports_tls(self):
        for hyper_net in self.hyper_net_sequences:
            if self.contains_tls_pair(hyper_net):
                return False

        for non_hyper_net in self.non_hyper_net_sequences:
            if self.contains_tls_pair(non_hyper_net):
                return True

        return False

    @staticmethod
    def contains_tls_pair(word):
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

def main():
    file_name = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day7_input.txt')

    file_data = IPv7.load_data_from_file(file_name)

    ipv7_list = list()

    for data in file_data:
        ipv7_list.append(IPv7(data))

    number_of_ipv7_tls = IPv7.number_of_tls_supported(ipv7_list)

    print('Answer 1: {}'.format(number_of_ipv7_tls))

if __name__ == '__main__':
    main()
