"""Day 2 of Advent of Code.
"""
import os


KEYPAD = [
    ['', '', '1', '', ''],
    ['', '2', '3', '4', ''],
    ['5', '6', '7', '8', '9'],
    ['', 'A', 'B', 'C', ''],
    ['', '', 'D', '', '']]


def code_from_instructions(instructions, key_pad=None, start_location=None):
    """Return bathroom code from an array of instructions, using the predfined
       KEYPAD.

    Args:
        instructions (list): A list of instructions to get the
                             bathroom key.
        key_pad (list): A 2D list representing a keypad.
        start_location (list): A list representing the starting location in
                               keypad.

    Returns:
        (str): A string representing the bathroom key.
    """
    bathroom_key = ''

    if key_pad is None:
        key_pad = KEYPAD

    if start_location is None:
        start_location = [2, 0]

    current_key_location = start_location
    num_key_pad_rows = len(key_pad)
    num_key_pad_cols = len(key_pad[0])

    for instruction in instructions:
        for symbol in instruction:
            if symbol == 'U':
                if current_key_location[0] - 1 >= 0:
                    current_key_location[0] -= 1

                new_symbol = (
                    key_pad[current_key_location[0]][current_key_location[1]])

                if new_symbol == '':
                    current_key_location[0] += 1

            elif symbol == 'D':
                if current_key_location[0] + 1 < num_key_pad_rows:
                    current_key_location[0] += 1

                new_symbol = (
                    key_pad[current_key_location[0]][current_key_location[1]])

                if new_symbol == '':
                    current_key_location[0] -= 1

            elif symbol == 'L':
                if current_key_location[1] - 1 >= 0:
                    current_key_location[1] -= 1

                new_symbol = (
                    key_pad[current_key_location[0]][current_key_location[1]])

                if new_symbol == '':
                    current_key_location[1] += 1

            elif symbol == 'R':
                if current_key_location[1] + 1 < num_key_pad_cols:
                    current_key_location[1] += 1

                new_symbol = (
                    key_pad[current_key_location[0]][current_key_location[1]])

                if new_symbol == '':
                    current_key_location[1] -= 1

        bathroom_key += (
            key_pad[current_key_location[0]][current_key_location[1]])

    return bathroom_key


def instructions_from_file(file_path):
    """Return list of key instructions from file.

    Args:
        file_path (str): Text file containing file instructions.

    Returns:
        (list): A list of instructions to get the bathroom key.
    """
    list_of_instructions = []

    with open(file_path, 'r') as instruction_file:
        for line in instruction_file:
            line = line.rstrip('\r\n')
            list_of_instructions += [line]

    return list_of_instructions


def main():
    """Main of module.
    """
    input_file = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day2_input.txt')

    instructions = instructions_from_file(input_file)

    print(
        'Code from the instructions: {}'.format(
            code_from_instructions(instructions)))

if __name__ == '__main__':
    main()
