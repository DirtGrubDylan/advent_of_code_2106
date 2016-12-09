import os
import re

class ExperimentalSequence(object):
    def __init__(self, sequence):
        super().__init__()

        self.sequence = sequence
        self.original_sequence = sequence

    def decode(self):
        decoded_sequence = list()

        starting_index = 0

        substr = self.sequence

        mark_search = re.search('\(\d+x\d+\)', substr)

        while mark_search is not None:
            marker, marker_start_index, marker_end_index = (
                self.decode_re_mark_search(mark_search))

            decoded_sequence.append(
                len(substr[starting_index : marker_start_index]))

            starting_index = (marker_end_index) + marker[0]

            s_substr = substr[marker_end_index : starting_index]

            decoded_sequence.append(marker[1] * len(s_substr))

            substr = substr[starting_index:]

            mark_search = re.search('\(\d+x\d+\)', substr)


        decoded_sequence.append(len(substr))

        return sum(decoded_sequence)

    def decode_v2(self):
        # decoded_sequence = list()

        return self.rec_marker_interp(self.sequence, 1)

    @staticmethod
    def rec_marker_interp(substr, multiplier):
        decoded_sequence = list()

        starting_index = 0

        while marker is not None:
            decoded_sequence.append(
                len(self.sequence[starting_index : marker_start_index]))
            starting_index = (marker_end_index + 1) + marker[0]
            decoded_sequence.append(
                marker[1] *
                len(self.sequence[marker_end_index + 1 : starting_index]))

        decoded_sequence.append(len(self.sequence[starting_index:]))

        return sum(decoded_sequence)

    @staticmethod
    def decode_re_mark_search(re_mark_search):
        marker_substr = re_mark_search.group(0)
        marker = eval(marker_substr.replace('x', ', '))
        marker_starting_index, marker_ending_index = re_mark_search.span()

        return (marker, marker_starting_index, marker_ending_index)

    @staticmethod
    def experimental_sequences_from_file(file_name):
        experimental_sequences = list()

        with open(file_name) as data_file:
            for line in data_file:
                experimental_sequences.append(
                    ExperimentalSequence(line.strip()))

        return experimental_sequences


def main():
    data_file = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day9_input.txt')

    given_sequence = (
        ExperimentalSequence.experimental_sequences_from_file(data_file)[0])

    print('Answer 1: {}'.format(given_sequence.decode()))
    # print('Answer 2: {}'.format(len(given_sequence.decode_v2())))

if __name__ == '__main__':
    main()
