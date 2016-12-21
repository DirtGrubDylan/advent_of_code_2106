import re
import os
from collections import deque

BOT_SEARCH = re.compile('bot\s(\d+)')
LOW_SEARCH = re.compile('low\sto\s(\w+)\s(\d+)')
HIGH_SEARCH = re.compile('high\sto\s(\w+)\s(\d+)')
VAL_SEARCH = re.compile('value\s(\d+)\sgoes\sto\sbot\s(\d+)')


class Warehouse(object):
    def __init__(self, number_1, number_2):
        super().__init__()

        self.bots = {}
        self.output_bins = {}
        self.number_1, self.number_2 = sorted([number_1, number_2])
        self.from_input_instructions = []
        self.desired_bot_id = None

    def load_bots_with_instructions_from_file(self, file_name):
        with open(file_name) as data_file:
            for line in data_file:
                line = line.strip()
                bot = BOT_SEARCH.search(line)
                main_bot_id = bot.groups()[0]

                while bot is not None:
                    if bot.groups()[0] not in self.bots:
                        self.bots[bot.groups()[0]] = Bot()

                    bot = BOT_SEARCH.search(line, bot.span()[1])

                main_bot = self.bots[main_bot_id]

                if line[0] == 'b':  # instruction
                    main_bot.instructions.append(line)
                else:
                    self.from_input_instructions.append(line)

    def run_bots(self):
        for from_input_instruction in self.from_input_instructions:
            value, bot_id = VAL_SEARCH.search(from_input_instruction).groups()
            value = int(value)

            self.bots[bot_id].values.append(value)

            if self.bots[bot_id].is_full():
                self.execute(bot_id)

    def execute(self, current_bot_id):
        current_bot = self.bots[current_bot_id]

        if current_bot.contains(self.number_1, self.number_2):
            self.desired_bot_id = current_bot_id

        low_destination, high_destination = current_bot.next_instruction()

        if low_destination[0] == 'bot':
            low_bot = self.bots[low_destination[1]]
            current_bot.give_low_value_to(low_bot)

            if low_bot.is_full():
                self.execute(low_destination[1])
        elif low_destination[0] == 'output':
            if low_destination[1] not in self.output_bins:
                self.output_bins[low_destination[1]] = []

            low_output_bin = self.output_bins[low_destination[1]]

            current_bot.output_low_value_to(low_output_bin)

        if high_destination[0] == 'bot':
            high_bot = self.bots[high_destination[1]]
            current_bot.give_high_value_to(high_bot)

            if high_bot.is_full():
                self.execute(high_destination[1])
        elif high_destination[0] == 'output':
            if high_destination[1] not in self.output_bins:
                self.output_bins[high_destination[1]] = []

            high_output_bin = self.output_bins[high_destination[1]]

            current_bot.output_high_value_to(high_output_bin)


class Bot(object):
    """docstring for Bot"""

    def __init__(self):
        super().__init__()

        self.values = []
        self.instructions = deque()

    def is_full(self):
        return len(self.values) == 2

    def contains(self, number_1, number_2):
        return sorted(self.values) == [number_1, number_2]

    def next_instruction(self):
        instruction = self.instructions.popleft()

        low_destination_and_id = LOW_SEARCH.search(instruction).groups()
        high_destination_and_id = HIGH_SEARCH.search(instruction).groups()

        return (low_destination_and_id, high_destination_and_id)

    def give_low_value_to(self, other_bot):
        try:
            low_value = min(self.values)
            other_bot.values.append(low_value)
            self.values.remove(low_value)
        except ValueError:
            pass

    def give_high_value_to(self, other_bot):
        try:
            high_value = min(self.values)
            other_bot.values.append(high_value)
            self.values.remove(high_value)
        except ValueError:
            pass

    def output_low_value_to(self, output_bin):
        try:
            low_value = min(self.values)
            output_bin.append(low_value)
            self.values.remove(low_value)
        except ValueError:
            pass

    def output_high_value_to(self, output_bin):
        try:
            high_value = min(self.values)
            output_bin.append(high_value)
            self.values.remove(high_value)
        except ValueError:
            pass

def main():
    file_name = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day10_input.txt')

    warehouse = Warehouse(61, 17)

    warehouse.load_bots_with_instructions_from_file(file_name)
    warehouse.run_bots()

    print('Answer 1: {}'.format(warehouse.desired_bot_id))

    print('Answer 2: {}'.format(
        warehouse.output_bins['0'][0] * warehouse.output_bins['1'][0] *
        warehouse.output_bins['2'][0]))

if __name__ == '__main__':
    main()
          