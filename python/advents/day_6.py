import os
from collections import Counter

class RepCode(object):
    def __init__(self, data_file):
        super().__init__()

        self.data = list()
        self.message = ''

        self.load_data_from_file(data_file)

    def decode_ml_data(self):
        num_of_slots = len(self.data[0])
        counters = [Counter() for i in range(num_of_slots)]

        for message in self.data:
            for counter, symbol in zip(counters, message):
                counter[symbol] += 1

        message = [counter.most_common()[0][0] for counter in counters]

        self.message = ''.join(message)

    def decode_ll_data(self):
        num_of_slots = len(self.data[0])
        counters = [Counter() for i in range(num_of_slots)]

        for message in self.data:
            for counter, symbol in zip(counters, message):
                counter[symbol] += 1

        message = [counter.most_common()[-1][0] for counter in counters]

        self.message = ''.join(message)

    def load_data_from_file(self, file_name):
        with open(file_name, 'r') as data_file:
            for line in data_file:
                self.data.append(line.strip())


def main():
    file_name = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day6_input.txt')

    rep_code = RepCode(file_name)

    rep_code.decode_ml_data()

    print('First answer: {}'.format(rep_code.message))

    rep_code.decode_ll_data()

    print('Second answer: {}'.format(rep_code.message))


if __name__ == '__main__':
    main()
