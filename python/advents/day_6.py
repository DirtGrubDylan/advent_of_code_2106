from collections import Counter

class RepCode(object):
    def __init__(self, data_file):
        super().__init__()

        self.data = list()
        self.message = ''

        self.load_data_from_file(data_file)
        self.decode_data()

    def decode_data(self):
        counters = [Counter() for i in range(6)]

        for message in self.data:
            for counter, symbol in zip(counters, message):
                counter[symbol] += 1

        message = [counter.most_common()[0][0] for counter in counters]

        self.message = ''.join(message)

    def load_data_from_file(self, file_name):
        with open(file_name, 'r') as data_file:
            for line in data_file:
                self.data.append(line.strip())


def main():
    pass


if __name__ == '__main__':
    main()
