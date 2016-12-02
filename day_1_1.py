"""Day 1, problem 1, of Advent of Code."""

TEST_DIRECTIONS_1 = ['R2', 'L3']
TEST_DIRECTIONS_2 = ['R2', 'R2', 'R2']
TEST_DIRECTIONS_3 = ['R5', 'L5', 'R5', 'R3']
DIRECTIONS = [
    'R1', 'R3', 'L2', 'L5', 'L2', 'L1', 'R3', 'L4', 'R2', 'L2', 'L4', 'R2',
    'L1', 'R1', 'L2', 'R3', 'L1', 'L4', 'R2', 'L5', 'R3', 'R4', 'L1', 'R2',
    'L1', 'R3', 'L4', 'R5', 'L4', 'L5', 'R5', 'L3', 'R2', 'L3', 'L3', 'R1',
    'R3', 'L4', 'R2', 'R5', 'L4', 'R1', 'L1', 'L1', 'R5', 'L2', 'R1', 'L2',
    'R188', 'L5', 'L3', 'R5', 'R1', 'L2', 'L4', 'R3', 'R5', 'L3', 'R3', 'R45',
    'L4', 'R4', 'R72', 'R2', 'R3', 'L1', 'R1', 'L1', 'L1', 'R192', 'L1', 'L1',
    'L1', 'L4', 'R1', 'L2', 'L5', 'L3', 'R5', 'L3', 'R3', 'L4', 'L3', 'R1',
    'R4', 'L2', 'R2', 'R3', 'L5', 'R3', 'L1', 'R1', 'R4', 'L2', 'L3', 'R1',
    'R3', 'L4', 'L3', 'L4', 'L2', 'L2', 'R1', 'R3', 'L5', 'L1', 'R4', 'R2',
    'L4', 'L1', 'R3', 'R3', 'R1', 'L5', 'L2', 'R4', 'R4', 'R2', 'R1', 'R5',
    'R5', 'L4', 'L1', 'R5', 'R3', 'R4', 'R5', 'R3', 'L1', 'L2', 'L4', 'R1',
    'R4', 'R5', 'L2', 'L3', 'R4', 'L4', 'R2', 'L2', 'L4', 'L2', 'R5', 'R1',
    'R4', 'R3', 'R5', 'L4', 'L4', 'L5', 'L5', 'R3', 'R4', 'L1', 'L3', 'R2',
    'L2', 'R1', 'L3', 'L5', 'R5', 'R5', 'R3', 'L4', 'L2', 'R4', 'R5', 'R1',
    'R4', 'L3']

def interpret_directions(directions):
    """Interpret given directions.

    Args:
        directions (list): A list of strings indication turn direction and
                           length of distance.
    Returns:
        (int): Distance, in blocks, from origin.
    """
    index = 0
    ns_parity = 1
    ew_parity = 1
    direction_parity_table = {'R': 1, 'L': -1}

    ns_distance = 0
    ew_distance = 0

    for direction in directions:
        direction_parity = direction_parity_table[direction[0]]

        if index % 2 == 0: #heading ew
            if ns_parity == 1: #facing north
                ew_parity = direction_parity
            else: #facing south
                ew_parity = -direction_parity

            ew_distance += ew_parity * int(direction[1:])
        else: #heading ns
            if ew_parity == 1: #facing east
                ns_parity = -direction_parity
            else: #facing west
                ns_parity = direction_parity

            ns_distance += ns_parity * int(direction[1:])

        index += 1

    return abs(ns_distance) + abs(ew_distance)


def main():
    """Main of the file.
    """
    print(interpret_directions(['R8', 'R4']))
    print(interpret_directions(TEST_DIRECTIONS_1))
    print(interpret_directions(TEST_DIRECTIONS_2))
    print(interpret_directions(TEST_DIRECTIONS_3))
    print(interpret_directions(DIRECTIONS))

if __name__ == '__main__':
    main()
