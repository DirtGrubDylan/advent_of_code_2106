import re
from collections import deque

BOT_SEARCH = re.compile('bot\s(\d+)')
LOW_SEARCH = re.compile('low\sto\s(\w+)\s(\d+)')
HIGH_SEARCH = re.compile('high\sto\s(\w+)\s(\d+)')
VAL_SEARCH = re.compile('value\s(\d+)\sgoes\sto\sbot\s(\d+)')


class Warehouse(object):
    def __init__(self, number_1, number_2):
        super().__init__()

        self.bots = {}
        self.from_input_instructions = []
        self.desired_bot_id = None

    def load_bots_with_instructions_from_file(self, file_name):
        with open(file_name) as data_file:
            for line in data_file:
                bot = BOT_SEARCH.search(line)
                main_bot_id = bot.groups()[0]

                while bot is not None:
                    if bot.groups()[0] not in self.bots:
                        self.bots[bot.groups()[0]] = Bot()

                    bot = BOT_SEARCH.search(line[bot.span()[1]:])

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

        if high_destination[0] == 'bot':
            high_bot = self.bots[high_destination[1]]
            current_bot.give_high_value_to(high_bot)

            if high_bot.is_full():
                self.execute(high_destination[1])


class Bot(object):
    """docstring for Bot"""

    def __init__(self):
        super().__init__()

        self.values = []
        self.instructions = deque()

    def is_full(self):
        return len(self.values) == 2

    def contains(self, number_1, number_2):
        return sorted(self.values) == sorted([number_1, number_2])

    def next_instruction(self):
        instruction = self.instructions.popleft()

        low_destination_and_id = LOW_SEARCH.search(instruction).groups()
        high_destination_and_id = HIGH_SEARCH.search(instruction).groups()

        return (low_destination_and_id, high_destination_and_id)

    def give_low_value_to(self, other_bot):
        low_value = min(self.values)
        other_bot.values.append(low_value)
        self.values.remove(low_value)

    def give_high_value_to(self, other_bot):
        high_value = min(self.values)
        other_bot.values.append(high_value)
        self.values.remove(high_value)

def main():
    pass

if __name__ == '__main__':
    main()
          