"""Day 2 of Advent of Code.
"""
import os


def code_from_instructions(instructions):
    """Return bathroom code from an array of instructions.

    Args:
        instructions (list): A list of instructions to get the
                             bathroom key.

    Returns:
        (str): A string representing the bathroom key.
    """
    bathroom_key = ''

    current_key = 5

    for instruction in instructions:
        for symbol in instruction:
            if symbol == 'U':
                if current_key - 3 > 0:
                    current_key -= 3
            elif symbol == 'D':
                if current_key + 3 <= 9:
                    current_key += 3
            elif symbol == 'L':
                if (current_key - 1) % 3 != 0:
                    current_key -= 1
            elif symbol == 'R':
                if (current_key + 1) % 3 != 1:
                    current_key += 1

        bathroom_key += str(current_key)

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
    input_file = (
        os.path.join(
            os.path.join(os.path.dirname(__file__), 'data'), 'day2_input.txt'))

    instructions = instructions_from_file(input_file)

    print(
        'Code from the instructions: {}'.format(
            code_from_instructions(instructions)))

if __name__ == '__main__':
    main()
